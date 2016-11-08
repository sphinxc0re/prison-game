extern crate rand;
extern crate yaml_rust;

mod guard;
mod prisoner;
mod utils;
mod mp;

use guard::Guard;
use mp::Message;
use mp::Envelope;
use prisoner::Prisoner;
use std::thread;
use std::time::Duration;
use rand::Rng;
use yaml_rust::YamlLoader;
use std::fs::File;
use std::io::prelude::*;
use std::sync::{Arc, Barrier};

fn main() {
    static TREATMENT_RATIO: i8 = 4;

    // The amount and type of the prisoners needs as well as the names of the prisoners are
    // configurable through the config.yml file. At this point we have to throw an error to prevent
    // further damage via a corrupted yml file.
    let mut yaml_file = File::open("config.yml").expect("Could not load config.yml file.");
    let mut yaml_string = String::new();
    yaml_file.read_to_string(&mut yaml_string).expect("Could not read content of config.yml file into string.");

    let docs = YamlLoader::load_from_str(yaml_string.as_str()).unwrap();
    // The seperate yaml doc is extracted
    let doc = &docs[0];

    println!("Starting prisoners game");
    println!("=======================");

    // To be consistent, we save the names of the prisoners and their needs to seperate variables.
    // This way, we are able to use them in further production.
    let guard_needs: Vec<String> = utils::str_vec_from_yaml_vec(doc["needs"].clone());
    let guard_vector: guard::GuardVec = Guard::new_vec(guard_needs.clone());

    let prisoner_names: Vec<String> = utils::str_vec_from_yaml_vec(doc["prisoners"].clone());
    let prisoner_vector: prisoner::PrisonerVec = Prisoner::new_vec(prisoner_names.clone());

    let start_barrier = Arc::new(Barrier::new(prisoner_names.len() + guard_needs.len()));

    for mut prisoner in prisoner_vector {
        // Since each prisoner has to have a connection to each guard, a copy of every guard sender
        // is saved in the prisoner instance.
        for guar in &guard_vector {
            prisoner.add_guard(&guar)
        }

        let need_vec = guard_needs.clone();

        let prisoners_barrier = start_barrier.clone();

        // Spawn thread and move prisoner into it. Afterwards, the prisoner is set up to produce
        // complaints at random to send them to the respective guard to handle it.
        thread::Builder::new().name(prisoner.name.clone() + "_thread").spawn(move|| {
            prisoner.broadcast_alive();
            println!("{} is registered to be alive", prisoner.name);
            prisoners_barrier.wait();
            loop {
                let mut rng = rand::thread_rng();
                let millis = (&mut rng).gen_range(1, 1000);
                thread::sleep(Duration::from_millis(millis));
                let amount = (&mut rng).gen_range(-20, 20);
                let need_index = (&mut rng).gen_range(0, need_vec.len());
                if prisoner.track_need(&need_vec[need_index], amount).abs() > 100 {
                    prisoner.broadcast_dead();
                    println!("{} died", prisoner.name);
                    break;
                }
                let comp = Message::Complaint { need: need_vec[need_index].clone(), amount: amount };
                prisoner.complain(comp);
                println!("{} has a need for {} for an amount of {}", prisoner.name.clone(), need_vec[need_index].clone(), amount);
                let envelope = prisoner.receive_message();
                match envelope {
                    Some(input_envelope) => {
                        let message = input_envelope.message;
                        match message {
                            Message::Treatment { need, amount } => {
                                prisoner.track_need(&need, amount * -1);
                                println!("{}'s need for {} has been treated for an amount of {}", prisoner.name, need, amount);
                            },
                            message => panic!("Unable to handle packet of type: {:?}", message)
                        }
                    },
                    None => unreachable!()
                }
            }
        }).unwrap();
    }

    let handlers: Vec<_> = guard_vector.into_iter().map(|mut guar| {
        let guard_barrier = start_barrier.clone();

        // Spawn thread and move guard into it. Afterwards, the guard is set up to listen to
        // the prisoners complaints
        thread::Builder::new().name(guar.need.clone() + "_thread").spawn(move|| {
            guard_barrier.wait();
            loop {
                let opt_message: Option<Envelope> = guar.receive_message();
                match opt_message {
                    Some(input_envelope) => {
                        let Envelope { return_sender, message } = input_envelope;
                        match message {
                            Message::Complaint { ref need, ref amount } => {
                                    let envelope = Envelope::new(Message::Treatment { need: (*need).clone(), amount: *amount / TREATMENT_RATIO }, guar.get_sender());
                                    return_sender.send(envelope).expect("Message could not be sent");
                            },
                            Message::Dead { prisoner_name } => {
                                guar.untrack_prisoner(&prisoner_name);
                                if guar.tracked_prisoners() == 0 {
                                    break;
                                }
                            },
                            Message::Alive { prisoner_name } => {
                                guar.track_prisoner(&prisoner_name);
                            },
                            other => panic!("Guard is unable to handle message of type: {:?}", other)
                        }
                    },
                    None => unreachable!()
                }
            }
        }).unwrap()
    }).collect();

    for handle in handlers {
        handle.join().unwrap();
    }
}

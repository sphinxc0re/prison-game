extern crate rand;
extern crate yaml_rust;

mod guard;
mod complaint;
mod prisoner;
mod utils;

use guard::Guard;
use complaint::Complaint;
use prisoner::Prisoner;
use std::thread;
use std::time::Duration;
use rand::Rng;
use yaml_rust::YamlLoader;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    // The ammount and type of the prisoners needs as well as the names of the prisoners are
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
    let guard_needs: Vec<&str> = utils::str_vec_from_yaml_vec(&doc["needs"]);
    let guard_vector: guard::GuardVec = Guard::new_vec(guard_needs);

    let prisoner_names: Vec<&str> = utils::str_vec_from_yaml_vec(&doc["prisoners"]);
    let prisoner_vector: prisoner::PrisonerVec = Prisoner::new_vec(prisoner_names);

    for mut prisoner in prisoner_vector {
        // Since each prisoner has to have a connection to each guard, a copy of every guard sender
        // is saved in the prisoner instance.
        for guar in &guard_vector {
            prisoner.add_guard(&guar)
        }

        // Spawn thread and move prisoner into it. Afterwards, the prisoner is set up to produce
        // complaints at random to send them to the respective guard to handle it.
        thread::spawn(move|| {
            loop {
                let seconds = rand::thread_rng().gen_range(1, 5);
                thread::sleep(Duration::new(seconds, 0));
                let ammount = rand::thread_rng().gen_range(-20, 20);
                let comp = Complaint::new("food", ammount, prisoner.name.clone());
                prisoner.complain(comp);
            }
        });
    }

    for guar in guard_vector {
        // Spawn thread and move guard into it. Afterwards, the guard is set up to listen to
        // the prisoners complaints
        thread::spawn(move|| {
            loop {
                let message: Complaint = guar.wait_for_and_receive_complaint();
                println!("{:?} has a need for {:?} for an ammount of {:?}", message.prisoner_name, message.need, message.ammount);
            }
        }).join().unwrap();
    }
}

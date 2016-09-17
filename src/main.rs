extern crate rand;
extern crate yaml_rust;

mod guard;
mod complaint;
mod prisoner;

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
    let mut yaml_file = File::open("config.yml").expect("nothing");
    let mut yaml_string = String::new();
    yaml_file.read_to_string(&mut yaml_string).expect("nothing");

    let docs = YamlLoader::load_from_str(yaml_string.as_str()).unwrap();

    // Multi document support, doc is a yaml::Yaml
    let doc = &docs[0];

    println!("Starting prisoners game");
    println!("=======================");

    let guard_needs: Vec<&str> = doc["needs"]
        .as_vec()
        .unwrap()
        .iter()
        .map(|e| {
            e.as_str().unwrap()
        })
        .collect();

    let guard_vector = Guard::new_vec(guard_needs);

    let prisoner_names: Vec<&str> = doc["prisoners"]
        .as_vec()
        .unwrap()
        .iter()
        .map(|e| {
            e.as_str().unwrap()
        })
        .collect();

    let prisoner_vector = Prisoner::new_vec(prisoner_names);

    for mut pris in prisoner_vector {
        for guar in &guard_vector {
            pris.add_guard(&guar)
        }
        // Spawn thread and move Prisoner into it
        thread::spawn(move|| {
            loop {
                let seconds = rand::thread_rng().gen_range(1, 5);
                thread::sleep(Duration::new(seconds, 0));
                let ammount = rand::thread_rng().gen_range(-20, 20);
                let comp = Complaint::new("food", ammount, pris.name.clone());
                pris.complain(comp);
            }
        });
    }

    for guar in guard_vector {
        // Spawn thread and move guard into it
        thread::spawn(move|| {
            loop {
                let message: Complaint = guar.complaint_receiver.recv().unwrap();
                println!("{:?} has a need for {:?}  {:?}", message.prisoner_name, message.need, message.ammount);
            }
        }).join().unwrap();
    }
}

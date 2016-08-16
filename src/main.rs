extern crate rand;

mod guard;
mod complaint;
mod prisoner;

use guard::Guard;
use complaint::Complaint;
use prisoner::Prisoner;
use std::thread;
use std::time::Duration;
use rand::Rng;

fn main() {
    let guard_needs = vec![
        "food",
        "health",
        "love",
        "freedom",
    ];

    let guard_vector = Guard::new_vec(guard_needs);

    let prisoner_vector = vec![
        Prisoner::new("Henry"),
        Prisoner::new("George"),
        Prisoner::new("Alfred"),
    ];

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

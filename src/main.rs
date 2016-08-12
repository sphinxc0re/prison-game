mod guard;
mod complaint;
mod prisoner;

use guard::Guard;
use complaint::Complaint;
use prisoner::Prisoner;
use std::thread;
use std::time::Duration;

fn main() {
    let guard_vector = vec![
        Guard::new("food"),
        Guard::new("health"),
        Guard::new("sleep"),
    ];

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
                thread::sleep(Duration::new(2, 0));
                let comp = Complaint::new("food", 12, pris.name.clone());
                pris.complain(comp);
            }
        });
    }

    for guar in guard_vector {
        // Spawn thread and move guard into it
        thread::spawn(move|| {
            loop {
                let message: Complaint = guar.complaint_receiver.recv().unwrap();
                println!("{:?}, {:?}, {:?}", message.need, message.ammount, message.prisoner_name);
            }
        });
    }
}

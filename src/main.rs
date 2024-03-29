use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();
    thread::spawn(move || {
        let vals = [
            String::from("Hi"),
            String::from("Jai Shree Ram!"),
            String::from("Bhagwan ji maf krdo"),
            String::from("I love Krishna"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = [
            String::from("Lorem ipsum"),
            String::from("Jai Shree Ram!"),
            String::from("Bhagwan maf krdo"),
            String::from("I love Krishna ji"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(2));
        }
    });

    for recieved in rx {
        println!("Got :{}", recieved);
    }
}

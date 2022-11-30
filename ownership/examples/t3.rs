use std::time::Duration;
use std::{thread, thread::sleep};
use std::sync::{Arc, Mutex};

#[derive(Debug)]
struct User {
    name: String,
}

fn main() {
    let user_original = Arc::new(Mutex::new(User {
        name: "drogus".to_string(),
    }));

    let user = user_original.clone();
    let t1 = thread::spawn(move || {
        let mut locked_user = user.lock().unwrap();
        locked_user.name = String::from("piotr");
    });

    let user = user_original.clone();
    let t2 = thread::spawn(move || {
        sleep(Duration::from_millis(10));

        // it will print: Hello piotr
        println!("Hello {}", user.lock().unwrap().name);
    });

    t1.join().unwrap();
    t2.join().unwrap();
}

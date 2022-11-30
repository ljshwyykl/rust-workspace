use crossbeam::scope;

#[derive(Debug)]
struct User {
    name: String,
}

fn main() {
    let user = User {
        name: "drogus".to_string(),
    };

    scope(|s| {
        s.spawn(|_| {
            println!("Hello from the first thread {}", &user.name);
        });

        s.spawn(|_| {
            println!("Hello from the second thread {}", &user.name);
        });
    })
    .unwrap();
}
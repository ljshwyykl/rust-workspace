use std::thread::spawn;

#[derive(Debug)]
struct User {
    name: String
}

fn main() {
    let user = User { name: "drogus".to_string() };

    let t1 = Thread::spawn(move || {
        println!("Hello from the first thread {}", user.name);
    });

    let t2 = Thread::spawn(move || {
        println!("Hello from the second thread {}", user.name);
    });

    t1.join().unwrap();
    t2.join().unwrap();
}

/* 
fn main() {
    let user = User { name: "drogus".to_string() };

    spawn(move || {
        println!("Hello from the first thread {}", user.name);
    }).join().unwrap();
}
*/

//https://zhuanlan.zhihu.com/p/523959791
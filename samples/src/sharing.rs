use std::sync::{Arc, RwLock};

use crate::types::Person;

pub fn party_on_bob() {
    let person = Arc::new(RwLock::new(Person::new("Bob", 20)));
    let clone1 = person.clone();
    std::thread::spawn(move || make_older(clone1));
    let clone2 = person.clone();
    std::thread::spawn(move || make_name_inherited(clone2));

    loop {
        std::thread::sleep(std::time::Duration::from_millis(1000));
        let person = person.read().unwrap();
        println!("{} is now aged {}", person.name, person.age);
    }
}

fn make_older(person: Arc<RwLock<Person>>) {
    loop {
        std::thread::sleep(std::time::Duration::from_millis(750));
        let mut person = person.write().unwrap();
        person.age += 1;
    }
}

fn make_name_inherited(person: Arc<RwLock<Person>>) {
    loop {
        std::thread::sleep(std::time::Duration::from_millis(1100));
        let mut person = person.write().unwrap();
        person.name = format!("{} the Second", person.name);
    }
}

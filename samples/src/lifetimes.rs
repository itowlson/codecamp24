use crate::types::Person;

// fn ownership() {
//     let person = Person::new("Bob", 20);
//     do_thing_with_1(person);
//     println!("now to do more things with {}", person);
// }

// fn do_thing_with_1(person: Person) {
//     println!("I am doing a thing with {}", person);
// }

fn borrowing() {
    let person = Person::new("Bob", 20);
    do_thing_with_2(&person);
    println!("now to do more things with {}", person);
}

fn do_thing_with_2(person: &Person) {
    println!("I am doing a thing with {}", person);
}

// Functions can return a borrow of a borrowed argument, but
// the returned value will prolong the borrow
impl crate::types::ContactMethod {
    // Okay
    fn email_borrowed(&self) -> Option<&str> {
        if let Self::Email(email) = self {
            Some(email)
        } else {
            None
        }
    }

    // Not okay
    // fn email_borrowed_from_owned<'a>(self) -> Option<&'a str> {
    //     if let Self::Email(email) = &self {
    //         Some(email)
    //     } else {
    //         None
    //     }
    // }
}

fn return_borrow_lifetime() {
    let cm = crate::types::ContactMethod::DoNotContact;
    let email = cm.email_borrowed();
    // drop(cm);  // No! No! No!
    println!("{email:?}");
}

// fn lifetime() {
//     let person = Person::new("Bob", 20);
//     std::thread::spawn(|| do_thing_with_2(&person));
//     println!("meanwhile doing other things with {}", person);
// }

fn lifetime2() {
    let person = Person::new("Bob", 20);
    std::thread::scope(|scope| {
        scope.spawn(|| do_thing_with_2(&person));
        println!("meanwhile doing other things with {}", person);
    });
}

// fn exclusive_borrow() {
//     let mut person = Person::new("Bob", 20);
//     let person_ref1 = &mut person;
//     let person_ref2 = &mut person;
//     mutate(person_ref1);
//     mutate(person_ref2);
// }

// fn mutate(person: &mut Person) {
//     person.name = "Carol".to_string();
// }

fn fingers_crossed() {
    let mut person = Person::new("Bob", 20);
    let ptr = std::ptr::addr_of_mut!(person.name);
    unsafe { ptr.sub(10).write_bytes(0x99, 1) };  // NO! NO! NO!
}

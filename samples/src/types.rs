pub struct Person {
    pub name: String,
    pub age: u16,
    contact: ContactMethod,
}

pub enum ContactMethod {
    DoNotContact,
    Email(String),
    Postal(Address),
}

pub struct Address {
    // elided
}

impl Person {
    // Constructors

    pub fn new(name: &str, age: u16) -> Self {
        Self {
            name: name.to_owned(),
            age,
            contact: ContactMethod::DoNotContact,
        }
    }

    fn newborn(name: &str) -> Self {
        Self {
            name: name.to_owned(),
            age: 0,
            contact: ContactMethod::DoNotContact,
        }
    }

    // Instance methods

    fn print_greeting(&self) {
        println!("Hello {}!", self.name);
    }

    fn get_age(&self) -> u16 {
        self.age
    }
}

impl ContactMethod {
    fn to_string(&self) -> String {
        match self {
            Self::DoNotContact => String::from("Do not contact"),
            Self::Email(email) => email.clone(),
            Self::Postal(_address) => todo!(),
        }
    }

    fn email(&self) -> Option<String> {
        if let Self::Email(email) = self {
            Some(email.clone())
        } else {
            None
        }
    }
}

fn all_emails(people: &[Person]) -> Vec<String> {
    people.iter()
        .filter_map(|p| p.contact.email())  // Closure syntax
        .collect()
}

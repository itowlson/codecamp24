use crate::types::Person;

trait HeadCount {
    fn how_many_heads(&self) -> usize;
}

// Implement my trait on my type
impl HeadCount for Person {
    fn how_many_heads(&self) -> usize {
        1
    }
}

// Implement someone else's trait on my type
impl std::fmt::Display for Person {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.name)
    }
}

// Implement my trait on someone else's type
impl<T> HeadCount for Vec<T> {
    fn how_many_heads(&self) -> usize {
        if self.is_empty() {
            0
        } else {
            1
        }
    }
}

// impl<T: HeadCount> HeadCount for Vec<T> {
//     fn how_many_heads(&self) -> usize {
//         self.iter()
//             .map(|item| item.how_many_heads())
//             .sum()
//     }
// }

fn different_types() -> Result<(), std::io::Error> {
    use std::fs::create_dir_all;
    use std::path::{Path, PathBuf};

    let s1: &str = "first";
    create_dir_all(s1)?;
    let s2: String = "second".to_owned();
    create_dir_all(s2)?;
    let path1: PathBuf = PathBuf::from("third").join("fourth");
    create_dir_all(path1.clone())?;
    let path2: &Path = path1.parent().unwrap();
    create_dir_all(path2)?;
    Ok(())
}

fn make_headed_1() -> impl HeadCount {
    Person::new("Bob", 20)
}

// fn make_headed_2(singular: bool) -> impl HeadCount {
//     if singular {
//         Person::new("Bob", 20)
//     } else {
//         vec![Person::new("Bob", 20), Person::new("Carol", 25)]
//     }
// }

fn make_headed_3(singular: bool) -> Box<dyn HeadCount> {
    if singular {
        Box::new(Person::new("Bob", 20))
    } else {
        Box::new(vec![Person::new("Bob", 20), Person::new("Carol", 25)])
    }
}

// Return type polymorphism

trait HeadCount2<T> {
    fn how_many_heads2(&self) -> T;
}

impl HeadCount2<usize> for Person {
    fn how_many_heads2(&self) -> usize {
        1
    }
}

impl HeadCount2<String> for Person {
    fn how_many_heads2(&self) -> String {
        "one".to_owned()
    }
}

fn print_heads(person: &Person) {
    print_a_usize(person.how_many_heads2());
    print_a_str(person.how_many_heads2());
}

fn print_a_usize(n: usize) {
    println!("usize {n}");
}

fn print_a_str(s: String) {
    println!("str {}", s);
}

fn real_world_return_type_polymorphism() {
    use std::collections::HashSet;

    let source = vec![1, 2, 3, 1, 2, 3];

    let v: Vec<_> = source.iter().map(|n| n * 2).collect();
    let h: HashSet<_> = source.iter().map(|n| n * 2).collect();

    // equivalently, specify return type on the polymorphic function:
    // let v = source.iter().map(|n| n * 2).collect::<Vec<_>>();
    //                                             ^^^^^^^^^^

    println!("vec = {v:?}, set = {h:?}");
}

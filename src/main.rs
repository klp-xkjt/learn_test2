#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Person {
    name: String,
    age: u32
}
fn main() {
    let mut person_list: [Person; 3] = [
        Person {
            name: String::from("bbbb"),
            age: 25
        },
        Person {
            name: String::from("aaa"),
            age: 34
        },
        Person {
            name: String::from("cccc"),
            age: 30
        }
    ];
    person_list.sort_by_key(|r| r.age);
    println!("{person_list:?}");

    person_list.sort_by_key(|r| r.name.clone());
    println!("{person_list:?}");

    let mut num_sort_operations = 0;
    person_list.sort_by_key(|r| {
        num_sort_operations += 1;
        r.age
    });
    println!("{person_list:#?}, sorted in {num_sort_operations} operations");
}

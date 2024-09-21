use sandbox::Describe;

#[derive(Describe)]
struct Person {
    name: String,
    age: u32,
}

#[derive(Describe)]
enum MyEnum {
    A,
    B,
    C,
}

fn main() {
    let person = Person {
        name: "Alice".to_string(),
        age: 30,
    };
    println!("{}", person.describe());

    let my_enum = MyEnum::A;
    println!("{}", my_enum.describe());
}


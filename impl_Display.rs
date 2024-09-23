// https://doc.rust-lang.org/std/fmt/trait.Display.html
// Learn Rust in a Month of Lunches; page 134 

#[derive(Debug)]
struct Cat {
    name: String,
    age: u8,
}

impl std::fmt::Display for Cat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} is a cat who is {} years old", self.name, self.age)
    }
}

fn print_excitedly(input: String) {
    println!("{input}!!!!!!!");
}

fn main() {
    let mr_mantle = Cat {
        name: "Reggie Mantle".to_string(),
        age: 4,
    };

    // by implementing std::fmt::Display on a type it automatically has the ToString trait which has the to_string() method.
    print_excitedly(mr_mantle.to_string());
    println!("Mr. Mantle's String is {} letters long.", mr_mantle.to_string().chars().count())
}

pub trait Weird {
    fn get_name(&self) -> &str;
}

struct Car {
    name: String
}

impl Car {
    fn new(name: &str) -> Self {
        return Self { name: name.into() }
    }
}

impl Weird for Car {
    fn get_name<'a>(&'a self) -> &'a str {
        return &self.name;
    }
}

pub fn oink() -> Box<dyn Weird> {
    Box::new(Car::new("tss"))
}

/*
    Skod løsning:
    - da pointer til stack ikke giver mening

    Brug box
    Forstå &'static
 */

fn main() {
    let d = oink();
    
    println!("{}", d.get_name());
    println!("Hello, world!");
}
// https://doc.rust-lang.org/book/ch10-02-traits.html
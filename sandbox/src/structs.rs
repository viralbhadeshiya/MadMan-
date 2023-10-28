// Tradition struct
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

// Tuple struct
struct ColorOne(u8, u8, u8);

// Function struct
struct Person {
    first_name: String,
    last_name: String,
}

impl Person {
    fn new(first: &str, last: &str) -> Person {
        Person { first_name: first.to_string(), last_name: last.to_string() }
    }

    fn fullname(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    fn set_lastname(&mut self, last: &str) {
        self.last_name = last.to_string();
    }

    fn to_tuple(self) -> (String, String) {
        (self.first_name, self.last_name)
    }
}

pub fn run() {
    let mut c = Color {
        red: 255,
        green: 0,
        blue: 0,
    };

    c.red = 200;
    println!("Coloe r: {}, g: {}, b: {}", c.red, c.green, c.blue);

    let mut c1 = ColorOne(255, 0, 0);

    c1.0 = 200;
    println!("Coloe r: {}, g: {}, b: {}", c1.0, c1.1, c1.2);

    let mut p1 = Person::new("John", "Doe");

    p1.set_lastname("Wiliams");
    println!("Full name: {}", p1.fullname());
    println!("tuple: {:?}", p1.to_tuple());
}
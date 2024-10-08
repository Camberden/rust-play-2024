use std::fmt::Display;
use std::io;

fn main() {
    println!("Hello, world!");
    hi();
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to Read Line!");
    println!("{}{}{}{}","You said: ", input, "Pineapple said: ", Pineapple::new().ppap());
    println!("{}", SOME_DAY);
}

fn hi(){
    println!("sup");
    let d = 2.1;
    let q: f32 = 3.0f32;
    println!("{d} {} {}", d + q as f64, q);

    for i in 0..10 {
        println!("I am {} years old", i);
    }

    println!("{}", calculator(5, 5, "+".to_string()));
}

fn calculator(s: i64, p: i64, t: String) -> i64 {
    println!("{}{}{}", s, t, p );
    let result = match t.as_str() {
        "+" => s + p,
        "-" => s - p,
        "*" => s * p,
        "/" => s / p,
        _ => panic!(),
    };
    result
}

const SS: &str = "SuperString";

#[derive(Clone, Debug, Default)]
struct Pineapple {
    pen: u64,
    apple: u8,
    know: i64,
}
impl Pineapple {
    fn new() -> Pineapple {
        Pineapple { pen: 1, apple: 2, know: i64::default() }
    }
    fn ppap(&self) -> String {
        format!("{}",self.pen + self.apple as u64 + self.know as u64)
    }
}
impl Display for Pineapple {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-{}-{}", self.pen, self.apple, self.know)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Date {
    January(u64),
    Otheruary(u64)
}
impl Display for Date {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

const NEW_YEARS: Date = Date::January(1);
const SOME_DAY: Date = Date::Otheruary(9);

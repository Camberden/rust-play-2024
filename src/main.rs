fn main() {
    println!("Hello, world!");
    hi();
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

const ss: &str = "SuperString";



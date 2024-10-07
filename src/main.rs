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
}







mod calculator;

fn main() {
    println!("Hello, world!");
    let summ: f64=calculator::calculate(calculator::Operation::Add(1.0, 2.0));
    println!("Summ: {}", summ.to_string());
}

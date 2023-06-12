pub enum Operation {
    Add(f64, f64),
    Subtract(f64, f64),
    Multiply(f64, f64),
    Divide(f64, f64),
}

pub fn calculate(operation: Operation) -> f64 {
    match operation {
        Operation::Add(y, x) => y + x,
        Operation::Subtract(y, x) => y -x,
        Operation::Divide(y, x) => y / x,
        Operation::Multiply(y, x) => y * x,
    }
}

#[derive(Debug)]
pub enum CliCommand {
    Sum(i32, i32),
    Minus(i32, i32),
    Multiply(i32, i32),
    Divide(i32, i32),
}

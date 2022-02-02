use std::io;

fn main() {
    let mut first = String::new();
    io::stdin()
        .read_line(&mut first)
        .expect("failed to read the line");
    let first: i32 = first.trim().parse().expect("please put the number");
    let mut operator = String::new();
    io::stdin()
        .read_line(&mut operator)
        .expect("please input the operator");

    let mut second = String::new();
    io::stdin()
        .read_line(&mut second)
        .expect("please put the number");
    let second: i32 = second.trim().parse().expect("please put the number");
    let operator: char = operator.chars().nth(0).unwrap();
    println!("{}", form(first, operator, second))
}
fn form(first: i32, operator: char, second: i32) -> String {
    match operator {
        '+' => format!("{} {} {} = {}", first, operator, second, first + second),
        '-' => format!("{} {} {} = {}", first, operator, second, first - second),
        '*' => format!("{} {} {} = {}", first, operator, second, first * second),
        '/' => format!("{} {} {} = {}", first, operator, second, first / second),
        _ => panic!("input correct operator"),
    }
}

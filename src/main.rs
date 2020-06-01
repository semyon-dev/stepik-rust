use std::io::stdin;

// Пример ввода строки с консоли и конвертация в i32
fn main() {
    let mut string: String = String::new();
    stdin().read_line(&mut string).expect("Не получилось считать строку!");
    let number: i32 = string.trim().parse::<i32>().unwrap();
    println!("{}", number);
}

use std::io;

fn main() {
    let mut accumulator: i32 = 0;
    loop {
        let mut input = String::new();
        let line_reading_result = io::stdin().read_line(&mut input);
        match line_reading_result {
            Ok(_) => {}
            Err(_) => {
                println!("Failed to read stdin!");
                return;
            }
        }
        let number: i32 = match input.trim().parse() {
            Ok(x) => x,
            Err(_) => {
                println!("NaN");
                return;
            }
        };
        if number == -1 {
            break;
        }
        if number <= 0 {
            println!("NaN");
            return;
        }
        accumulator += number;
    }
    println!("{}", accumulator);
}

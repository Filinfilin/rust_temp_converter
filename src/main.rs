use colored::Colorize;
use std::io;

fn main() {
    loop {
        let text = concat!(
            "1. if you wat to convert Farh to Cels\n",
            "2. if you want to convert Cels to Farh"
        );
        println!("{}", format!("Temperature converter").yellow().bold(),);
        println!(
            "{}\n{}",
            format!("Please print number and press Enter")
                .green()
                .bold(),
            format!("{}", text).bold(),
        );

        let mut type_of_convert = String::new();

        io::stdin()
            .read_line(&mut type_of_convert)
            .expect("Filed to read line");

        let convert_to: u8 = match type_of_convert.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("{}", format!("Input temperature:").bold());
        let mut temperature = String::new();
        io::stdin()
            .read_line(&mut temperature)
            .expect("Filed to read line");

        let input: i32 = match temperature.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if convert_to == 1 {
            convert_to_fahr(input);
        }
        if convert_to == 2 {
            convert_to_cels(input);
        }
    }
}

fn convert_to_fahr(input: i32) {
    println!(
        "{}",
        format!("The temperature is: {}", (input * 9 / 5) + 32)
            .red()
            .bold()
    )
}
fn convert_to_cels(input: i32) {
    println!(
        "{}",
        format!("The temperature is: {}", (input - 32) * 5 / 9)
            .red()
            .bold()
    )
}

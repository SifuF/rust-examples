use std::io;

fn main() {

     loop {

        println!("Enter temperature in Farenheit:");

        let mut temp = String::new();

        io::stdin().read_line(&mut temp).expect("Failed to read line!");

        let num: f64 = match temp.trim().parse() {
            Ok(n) => n,
            Err(_) => 1.0, 
        };

        let cel = (num - 32.0) * (5.0/9.0);
        println!("{} in Farenheit = {} Celcius:", num, cel);
    }
}
use std::io;

fn main() {

    println!("Enter n");

    let mut n = String::new();

    io::stdin().read_line(&mut n).expect("Failed to read line!");

    let num: i64 = match n.trim().parse() {
        Ok(x) => x,
        Err(_) => 1,
    }; 

    let mut fib_i = 0;
    let mut fib_j = 1;

    print!("fib({}) = {}, {}, ",num, fib_i, fib_j);

    let mut i: i64 = 0;
    while i<num-1 {

        let fib_n = fib_i + fib_j;
        fib_i = fib_j;
        fib_j = fib_n;

        print!("{}, ",fib_n);

        i+=1;
	}

}

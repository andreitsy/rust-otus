use std::vec::Vec;

fn main() {
    const COUNT: usize = 100;
    let mut Result = Vec::with_capacity(COUNT);
    for i in 1..=COUNT {
        let s = match (i % 3, i % 5) {
            (0, 0) => String:from("FizzBuzz"), (0, _) => String::from("Fizz"),
            (_, 0) => String::from("Buzz"), (_, _) => format!("{}", i)
        };
        Result.push(s);
    }
    print_result(&Result);
}

fn print_result(result: &Vec<String>) {
    for r in result {
        println!("{}", r)
    }
}
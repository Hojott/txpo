use std::io;
use std::str::FromStr;
use std::fmt::Debug;

fn main() {
    let start: i128 = input();
    let rounds: i128 = input();
    let mut results_i: Vec<i64> = Vec::new();
    let mut results_l: Vec<i128> = Vec::new();

    for r in 0..rounds {
        let result = txpo(start + r, r);
        results_i.push(result.0);
        results_l.push(result.2);
    }

    let state = format!("start: {},\nrounds: {}", start, rounds);
    let best_results = format!(
        "most iterations: {}\nlargest number: {}",
        results_i.iter().cloned().max().unwrap(),
        results_l.iter().cloned().max().unwrap()
    );

    println!("{}", state);
    println!("{}", best_results);
}

fn txpo(start: i128, round: i128) -> (i64, i128, i128) {
    let mut number = start;
    let mut largest = number;
    let mut i = 0;
    
    if round % 10000 == 0 {
        print!("{}", format!("r: {}\r", round));
    }
    while number != 1 {
        i += 1;

        if number % 2 == 0 {
            number /= 2;
        } else {
            number = number * 3 + 1;
        }

        if number > largest {
            largest = number;
        }

        //let data = format!(
        //    "i: {},    \tn: {},    \tl: {}, {}\tr: {}    ",
        //    i,
        //    number,
        //    largest,
        //    " ".repeat((start.to_string().len() as f64).floor() as usize),
        //    round
        //);
        //io::stdout().flush().unwrap();
        //println!("{}", data);
    }
    (i, number, largest)
}

fn input<T: FromStr>() -> T where <T as FromStr>::Err: Debug {
    //print!("{}", text);
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().parse().unwrap()
}


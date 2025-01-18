use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        4 => match args[3].trim() {
            "+" | "-" | "/" | "*" => {
                operate(args[1].trim(), args[2].trim(), args[3].trim());
            }

            _ => {
                println!("Unsupported operators!")
            }
        },
        _ => {
            println!("Please provide only three arguments!");
        }
    }
}

fn operate(x: &str, y: &str, operator: &str) {
    // converting &str to f64
    // Using match to handle error is a better approach.
    let a = match x.parse::<f64>() {
        Ok(num) => num,
        Err(_) => {
            println!("The first argument is not a number!");
            return;
        }
    };

    // using .expect() will make the program panic if any error
    // let b = y
    //     .parse::<f64>()
    //     .expect("The second argument is not a number!");

    // let b: f64 = y.parse().expect("The second argument is not a number!");

    // using unwrap_or will assign default value of 0.0 if cannot be converted
    let b: f64 = y.parse().unwrap_or(0.0);

    match operator {
        "+" => {
            println!("{}", a + b)
        }
        "-" => {
            println!("{}", a - b)
        }
        "*" => {
            println!("{}", a * b)
        }
        "/" => {
            if b == 0.0 {
                println!("Cannot divide by 0!");
            } else {
                println!("{}", a / b)
            }
        }
        _ => {}
    }
}

use std::{io, u32};

fn main() {
    let mut emps: Vec<(String, u32, u32)> = vec![];
    let mut input = String::new();

    for i in 1..=2 {
        println!("Enter Employee {}'s name: ", i);
        io::stdin().read_line(&mut input).expect("Input Failed");
        let name = String::from(input.trim());
        input.clear();

        println!("Enter Employee {}'s age: ", i);
        io::stdin().read_line(&mut input).expect("Input Failed");
        let age = input
            .trim()
            .parse::<u32>()
            .expect("Has to be positive integer");

        input.clear();

        println!("Enter Employee {}'s salary: ", i);
        io::stdin().read_line(&mut input).expect("Input Failed");
        let salary: u32 = input.trim().parse().expect("Has to be integer");
        input.clear();

        emps.push((name, age, salary));
    }

    print_employees(&emps);
    let total_salary = total_salary(&emps);
    println!("{}", total_salary);
    println!("Average Salary: {}", total_salary / emps.len() as u32);

    let mut highest_salary_index = 0;
    let mut oldest_age_index = 0;

    let mut max_salary = u32::MAX;
    let mut max_age = u32::MAX;

    for (i, (_, age, salary)) in emps.iter().enumerate() {
        if *age > max_age {
            max_age = *age;
            oldest_age_index = i;
        }

        if *salary > max_salary {
            max_salary = *salary;
            highest_salary_index = i;
        }
    }

    println!(
        "{} - {}",
        &emps[highest_salary_index].2, &emps[oldest_age_index].1
    );
}

fn print_employees(emps: &Vec<(String, u32, u32)>) {
    for (i, emp) in emps.iter().enumerate() {
        println!(
            "Employee {}: Name {}, Age: {}, Salary: {}",
            i + 1,
            emp.0,
            emp.1,
            emp.2
        );
    }
}

fn total_salary(emps: &Vec<(String, u32, u32)>) -> u32 {
    let sum = emps
        .iter()
        .map(|emp| emp.2)
        .collect::<Vec<u32>>()
        .iter()
        .sum();

    sum
}

use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write};

fn main() {
    let mut store_vec: Vec<String> = Vec::new();

    loop {
        let mut input = String::new();
        println!("Input: ");
        io::stdin()
            .read_line(&mut input)
            .expect("Cannot read input!");

        if input.trim().is_empty() {
            break;
        }

        store_vec.push(input.trim().to_uppercase());
    }

    // create a file, overwrite if the file already exits
    let mut data_file = File::create("data.txt").expect("File creation failed");

    // create a file, append if the file already exits
    // let mut data_file = OpenOptions::new()
    //     .append(true)
    //     .write(true)
    //     .create(true)
    //     .open("data.txt")
    //     .expect("Error opening or creating a file!");

    for line in &store_vec {
        // data_file
        //     .write_all(format!("{}\n", line).as_bytes())
        //     .expect("Write failed");
        writeln!(data_file, "{}", line).expect("Writing Failed!");
    }

    // open a file
    let mut data_file = File::open("data.txt").expect("File open failed");
    let mut contents = String::new();
    data_file
        .read_to_string(&mut contents)
        .expect("File read failed");

    for line in contents.lines() {
        println!("{}", line);
    }
}

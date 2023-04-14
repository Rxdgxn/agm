use std::io::{stdin, stdout, Write};

fn read(input: &mut String) {
    stdout().flush().expect("Flush");
    stdin().read_line(input).expect("Read");
}

fn main() {

    let mut lc = 1u32; // line counter
    let mut line = String::from("");

    while line != "END;" {
        line = String::from("");
        read(&mut line);
        line = line.trim().to_string();
        println!("[OUT]: {:?}", line);

        if line.chars().last().unwrap() != ';' {
            panic!("Error at line {lc}. Line must end with ';'");
        }

        lc += 1;
    }

}
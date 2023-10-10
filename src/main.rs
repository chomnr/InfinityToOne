use std::{io::{BufRead, self, Write}, vec, time::Duration, thread::sleep};


fn main() {
    
    // read input
    print!("Enter a positive integer: ");
    io::stdout().flush().unwrap();

    let mut number = String::new();
    io::stdin().read_line(&mut number).unwrap();
    number = number.trim().to_string();

    // parse input
    let parse: u128 = number.parse().unwrap();

    // collatz conjecture
    let mut collatz = CollatzSequence::new(parse);

    // keep looping until last value == 1;
    while collatz.seq[collatz.i] != 1 {
        // sequence the collatz conjecture...
        collatz.sequence();
    }
}

struct CollatzSequence {
    i: usize,
    seq: Vec<u128>
}

impl CollatzSequence {
    fn new(start: u128) -> Self {
        CollatzSequence { i: 0, seq: vec![start]}
    }

    fn sequence(&mut self) {
        let v =  &self.seq[self.i];
        let mut r = 0;

        if self.seq[0] % 2 == 0 {
            let r1 = v/2; // even rule
            r = r1;
        } else {
            let r2 = 3*(v)+1; // odd rule
            r = r2;
        }
        self.seq.push(r);
        self.i += 1;

        println!("Result: {}", r);
    }
}
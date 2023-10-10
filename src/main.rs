use core::num;
use std::{io::{self, Write}};

use num_bigint::BigUint;

fn main() {
    print!("Enter a positive integer: ");
    io::stdout().flush().unwrap();

    let mut number = String::new();
    io::stdin().read_line(&mut number).unwrap();
    number = number.trim().to_string();

    //let parse: u128 = number.parse().unwrap();
    let parse: BigUint = number.parse().unwrap();


    // collatz conjecture
    //let mut collatz: CollatzSequence = CollatzSequence::new(parse);
    let mut collatz_v2: CollatzEnormousSequence = CollatzEnormousSequence::new(parse);

     /* 
    while collatz.seq[collatz.i] != 1 {
        // sequence the collatz conjecture...
        collatz.sequence();
    }*/

    while collatz_v2.seq[collatz_v2.i] != BigUint::from(1 as usize) {
        collatz_v2.sequence();
    }
}

// Sequence smaller numbers
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

        if v % 2 == 0 {
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

// Sequence larger numbers
struct CollatzEnormousSequence {
    i: usize,
    seq: Vec<BigUint>
}

impl CollatzEnormousSequence {
    fn new(start: BigUint) -> Self {
        CollatzEnormousSequence {
            i: 0,
            seq: vec![start]
        }
    }

    fn sequence(&mut self) {
        let v = &self.seq[self.i];
        let mut r = self.b(0);

        if v % self.b(2) == self.b(0) {
            let r1 = v/self.b(2); // even rule
            r = r1;
        } else {
            let r2 = self.b(3)*(v)+self.b(1); // odd rule
            r = r2;
        }

        self.seq.push(r.clone());
        self.i += 1;

        println!("[BIG] Result: {:?}", r);
    }
    
    fn b(&self, number: usize) -> BigUint {
        return BigUint::from(number)
    }
}
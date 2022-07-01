use std::time::{Duration, Instant};

fn main() {
    // We can use this function here, and define it somewhere later
    // fizzbuzz_to(100);
    // file writing
    // numpy equivalent
    // database interface
    // apache arrow interface
    // unix value generator
    // async network requests
    // HTTP3
    // email sending
    // datetime management
    // kafka
    // pager duty
    // data oriented design method
    // Parallel arrays / ECS

    // Thread pinning


    // Initialize lookup table.
    let mut table: [String; 128] = [0; 128];
    for i in 0..128 {
        table[i] = get_lookup_string(i.to_string()) as u8;
    }
    
    // Version 1: use match.
    let now1 = Instant::now();
    let mut result1: u64 = 0;
    for _ in 0..10000000 {
        for c in "abcdef".chars() {
            result1 += get_lookup_string(c.to_string()) as u64;
        }
    }
    println!("RESULT: {}", result1);
    println!("  TIME: {} ms", now1.elapsed().as_millis());
    
    // Version 2: use lookup table.
    let now2 = Instant::now();
    let mut result2: u64 = 0;
    for _ in 0..10000000 {
        for c in "abcdef".chars() {
            result2 += table[c.to_string()] as u64;
        }
    }
    println!("RESULT: {}", result2);
    println!("  TIME: {} ms", now2.elapsed().as_millis());

    // Verions 3: lazy static map

    let now3 = Instant::now();
    let mut result3: u64 = 0;

    // TODO implementation
    for _ in 0..10000000 {
        for c in "abcdef".chars() {
            // TODO result3 += table[c.to_string()] as u64;
        }
    }

    println!("RESULT: {}", result3);
    println!("  TIME: {} ms", now3.elapsed().as_millis());
}

fn get_lookup_value(x: u8) -> u8 {
    return match x {
        b'a' => 30,
        b'b' => 70,
        b'c' => 10,
        b'd' => 10,
        b'e' => 35,
        _ => 0
    };
}

fn get_lookup_string(x: String) -> u8 {
    return match x {
        "a" => 1,
        "b" => 2,
        "c" => 3,
        _ => 4
    };
}

// Function that returns a boolean value
fn is_divisible_by(lhs: u32, rhs: u32) -> bool {
    // Corner case, early return
    if rhs == 0 {
        return false;
    }

    // This is an expression, the `return` keyword is not necessary here
    lhs % rhs == 0
}

// Functions that "don't" return a value, actually return the unit type `()`
fn fizzbuzz(n: u32) -> () {
    if is_divisible_by(n, 15) {
        println!("fizzbuzz");
    } else if is_divisible_by(n, 3) {
        println!("fizz");
    } else if is_divisible_by(n, 5) {
        println!("buzz");
    } else {
        println!("{}", n);
    }
}

// When a function returns `()`, the return type can be omitted from the
// signature
fn fizzbuzz_to(n: u32) {
    for n in 1..n + 1 {
        fizzbuzz(n);
    }
}

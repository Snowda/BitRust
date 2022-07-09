use std::time::{Duration, Instant};
use lazy_static::lazy_static;

lazy_static! {
    static ref LOOKUP4: [char; 128] = {
        let mut table = ['a'; 128];

        for i in 0..128 {
            table[i] = get_lookup_string((i as u8) as char) as char;
        }

        table
    };
}

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
    let mut table: [char; 128] = ['a'; 128];
    for i in 0..128 {
        
        table[i] = get_lookup_string((i as u8) as char) as char;
    }
    
    // Version 1: use match.
    let now1 = Instant::now();
    let mut result1: u64 = 0;
    for _ in 0..10000000 {
        result1 += get_lookup_string('a') as u64;
        result1 += get_lookup_string('b') as u64;
        result1 += get_lookup_string('c') as u64;
        result1 += get_lookup_string('d') as u64;
        result1 += get_lookup_string('e') as u64;
        result1 += get_lookup_string('f') as u64;
    }
    println!("RESULT: {}", result1);
    println!("  TIME: {} ms", now1.elapsed().as_millis());
    
    // Version 2: use lookup table.
    let now2 = Instant::now();
    let mut result2: u64 = 0;
    for _ in 0..10000000 {
        result2 += table['a' as usize] as u64;
        result2 += table['b' as usize] as u64;
        result2 += table['c' as usize] as u64;
        result2 += table['d' as usize] as u64;
        result2 += table['e' as usize] as u64;
        result2 += table['f' as usize] as u64;
    }
    println!("RESULT: {}", result2);
    println!("  TIME: {} ms", now2.elapsed().as_millis());

    // Verions 3: lazy static map
    
    let now3 = Instant::now();
    let mut result3: u64 = 0;

    for _ in 0..10000000 {
        result3 += LOOKUP4['a' as usize] as u64;
        result3 += LOOKUP4['b' as usize] as u64;
        result3 += LOOKUP4['c' as usize] as u64;
        result3 += LOOKUP4['d' as usize] as u64;
        result3 += LOOKUP4['e' as usize] as u64;
        result3 += LOOKUP4['f' as usize] as u64;
    }

    println!("RESULT: {}", result3);
    println!("  TIME: {} ms", now3.elapsed().as_millis());
}

fn lookup(c: char) -> u64{
    return LOOKUP4[c as usize] as u64;
}

fn get_lookup_value(x: char) -> u8 {
    return match x {
        'a' => 30,
        'b' => 70,
        'c' => 10,
        'd' => 10,
        'e' => 35,
        _ => 0
    };
}

fn get_lookup_string(x: char) -> u8 {
    return match x {
        'a' => 1,
        'b' => 2,
        'c' => 3,
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

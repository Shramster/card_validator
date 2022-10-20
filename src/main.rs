#![allow(unused)]
// PARALLELIZED VERSION!!!
use std::any::type_name;
use std::thread;
use std::sync::mpsc::{channel, Sender, Receiver};
use std::time::Instant;


fn main() {
    println!("Credit Card Validating App");
    let now = Instant::now();
    let args = std::env::args().skip(1).collect();
    let card_vec_unverified= parse_input(args);
    let mut results_vec :Vec<(u64, String, bool, bool)> = Vec::new();

    for card in card_vec_unverified {
        let (txlength,rxlength) = channel();
        let (txbrand,rxbrand) = channel();
        let (txluhns1,rxluhns1) = channel();
        let (txluhns2,rxluhns2) = channel();
        let (txstep5,rxstep5) = channel();

        let mut results = (card ,String::from(""), false, false);
        // STEP I
        let step_1 = thread::spawn(move || {
            let results_length = verify_length(card);
            txlength.send(results_length).unwrap();
        });
        results.3 = rxlength.recv().unwrap();
        step_1.join().unwrap();

        // IF STEP 1 RETRUNS TRUE SPAWN NEXT STEP
        if(results.3) {
        //  STEP II
            let step_2 = thread::spawn(move || {
                let brand = sort_by_brand(card);
                txbrand.send(brand).unwrap();
            });
            results.1 = rxbrand.recv().unwrap();
            step_2.join().unwrap();

            // IF STEP 2 RETRUNS TRUE SPAWN NEXT STEP
            if(results.1 != String::from("")) {

                // STEP III
                let step_3 = thread::spawn(move || {
                    let r1 = luhns_step_1(card);
                    txluhns1.send(r1).unwrap();
                });
                let r1 = rxluhns1.recv().unwrap();

                // STEP IV
                let step_4 = thread::spawn(move || {
                    let r2 = luhns_step_2(card);
                    txluhns2.send(r2).unwrap();
                });
                let r2 = rxluhns2.recv().unwrap();

                step_3.join().unwrap();
                step_4.join().unwrap();

                // STEP V
                let step_5 = thread::spawn(move || {
                    let results_is_valid = is_valid(r1, r2);
                    txstep5.send(results_is_valid).unwrap();
                });
                results.2 = rxstep5.recv().unwrap();

                step_5.join().unwrap();
                }
            }
        results_vec.push(results);
   }
    let elapsed_time = now.elapsed();
    print_results(results_vec, elapsed_time.subsec_nanos());

}

fn parse_input(content: Vec<String>) -> Vec<u64> {
    let mut card_vec = Vec::new();
    for line in content {
        card_vec.push(line.parse::<u64>().expect("Not a number"));
    }
    return card_vec
}

// Check how many characters long each number is
fn verify_length(number: u64) -> bool {
    let mut count_invalid_size: u32 = 0;
    let mut count_valid_size: u32 = 0;
    let mut result :bool = false;
    // Smallest value for a CC and largest value
    // Had to use this as I am comparing int values
    if number >= 4000000000000 && number <= 6999999999999999999 {
        result = true;
        count_valid_size += 1;
    }
    else {
        result = false;
        count_invalid_size += 1;
    }
    return result;
}

// Explode each number into a vector
fn vectorize(int: u64) -> Vec<u64> {

    let mut input = int.clone();
    let mut vec :Vec<u64> = Vec::new();
    let mut prev :u64 = 0;

// Using int division
    for i in (0..19).rev() {
        let mut first_value = false;
        prev = (input / u64::pow(10, i));
        input = input - prev * u64::pow(10, i);
        if((prev != 0) || !vec.is_empty()) {
            vec.push(prev);
            first_value = true;
        }
    }
    vec
}

// Sort cards by brand
fn sort_by_brand(card: u64) -> String {

    let mut results_brand :String = String::from("None");

    let mut count_of_iin :u32 = 0;
    //  Amex Range: 34, 37 Len 15
    if(((card / u64::pow(10, 13)) >= 34 &&
        (card / u64::pow(10, 13)) <= 37)) {
        results_brand = String::from("Amex");
        count_of_iin += 1;
    }
    //  Visa Range: 4  Len 13, 16
    else if(((card / u64::pow(10, 12)) == 4 ||
        (card / u64::pow(10, 15)) == 4)) {
        results_brand = String::from("Visa");
        count_of_iin += 1;
    }
    //  MC Range: 51-55  Len 16
    else if(((card / u64::pow(10, 14)) >= 51 &&
        (card / u64::pow(10, 14)) <=55 )) {
        results_brand = String::from("MC");
        count_of_iin += 1;
    }
    //  Discover Range: 6  Len 16-19
    else if((
        (card / u64::pow(10, 15)) == 6 ||
        (card / u64::pow(10, 16)) == 6 ||
        (card / u64::pow(10, 17)) == 6 ||
        (card / u64::pow(10, 18)) == 6 )) {
        results_brand = String::from("Disc");
        count_of_iin += 1;
    } else {
        results_brand = String::from("None");
    }
    return results_brand;
}



fn luhns_step_1( card: u64) -> u64 {

    let luhns_closure = | mut num :&u64| -> u64 {
        if(num < &5) {
            num*2
        } else {
            match num {
                5 => 1,
                6 => 3,
                7 => 5,
                8 => 7,
                9 => 9,
                _ => 0
            }
        }
    };

    let mut r1 :u64 = 0;

    let card_vec = vectorize(card);
    let mut card_iter = card_vec.iter();

    // if length is even
    if(card_vec.len() % 2 == 0) {
        let vec_2: Vec<u64> = card_iter
                        .step_by(2)
                        .map(luhns_closure)
                        .collect();

        r1 = vec_2.iter().sum::<u64>();

    // if length is odd
    } else {
        let vec_2: Vec<_> = card_iter
                        .skip(1)
                        .step_by(2)
                        .map(luhns_closure)
                        .collect();

        r1= vec_2.iter().sum::<u64>();
    }
    return r1;
}

fn luhns_step_2(card: u64) -> u64 {
    let mut r2 :u64 = 0;

    let new_vec = vectorize(card);
    let mut new_vec_iter = new_vec.iter();

    // if length is even
    if(new_vec.len() % 2 == 0) {
        r2 = new_vec.iter().skip(1).step_by(2).sum::<u64>();
    // if length is odd
    } else {
        r2 = new_vec.iter().step_by(2).sum::<u64>();
    }
    r2
}

fn is_valid(r1: u64, r2: u64) -> bool {

    if((r1+r2)%10 == 0) {
        true
    } else {
        false
    }
}

fn print_results( results_vec: Vec<( u64, String, bool, bool)>, time: u32) {
    let width = 20;
    let mut count :u32  = 0;
    println!("Invalid Card Numbers");
    println!("---------------------");
    for results in &results_vec {
        if(!results.2 || !results.3 ) {
        println!("{:width$}",results.0);
        count += 1;
        }
    }
    println!("\n\n{:width$}\t{}","Card Number","Issuer");
    println!("--------------------\t-----------------");
    for results in &results_vec {
        if(results.2 && results.3 ) {
        println!("{:width$}\t{}",results.0, results.1);
        count += 1;
        }
    }
    println!("Results {}", count);
    println!("Took {} nano seconds", time);
}

// Used to check types cause they're complex in Rust
fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}



#![allow(unused)]

// Clap provides a library for handling CLI arguments
// Parser helps to wrap and handle errors
use clap::Parser;
use std::any::type_name;
use std::thread;
use std::sync::mpsc::{channel, Sender, Receiver};

#[derive(Parser)]
struct Cli {
    path: std::path::PathBuf,
}

struct Brand {
    amex: Vec<u64>,
    visa: Vec<u64>,
    mc: Vec<u64>,
    disc: Vec<u64>,
}

fn main() {
    let (txlength,rxlength) = channel();
    let (txbrand,rxbrand) = channel();
    let (txluhns1,rxluhns1) = channel();
    let (txluhns2,rxluhns2) = channel();
    let (txstep5,rxstep5) = channel();
    println!("Credit Card Validating App");

    let args = Cli::parse();

    // Might switch to std::io::BufReader
    let content = std::fs::read_to_string(&args.path).expect("could not read file");

    // Pre-Processing, take user input and convert to a
    // vector of ints and create a Vector called results
    // which holds a tuple of results.j
    //      Vec<u64>       <-----      &String
    let card_vec_unverified= parse_input(&content);



    // STEP I
    // Take Vector of ALL user input and verify by length
    // and return a vector of which cards are of acceptable
    // length
    let step_1_clone = card_vec_unverified.clone();
    let step_1 = thread::spawn(move || {
        let results_length = verify_length(step_1_clone);
        txlength.send(results_length).unwrap();
    });
    let results_length = rxlength.recv().unwrap();


//  STEP II
//  Sort Cards by the IIN
//  Struct  Brand::Vec<u64>    <----  Vec<u64>
    let step_2_clone = card_vec_unverified.clone();
    let step_2 = thread::spawn(move || {
        let brands = sort_by_brand(step_2_clone);
        txbrand.send(brands).unwrap();
    });
    let results_brand = rxbrand.recv().unwrap();



    // STEP III
    // Luhn's Algo
    let luhns_step_1_clone = card_vec_unverified.clone();
    let step_3 = thread::spawn(move || {
        let r1 = luhns_step_1(luhns_step_1_clone);
        txluhns1.send(r1).unwrap();
    });
    let results_luhns_step_1 = rxluhns1.recv().unwrap();


    // STEP IV
    // Luhn's Algo
    let luhns_step_2_clone = card_vec_unverified.clone();
    let step_4 = thread::spawn(move || {
        let r2 = luhns_step_2(luhns_step_2_clone);
        txluhns2.send(r2).unwrap();
    });
    let results_luhns_step_2 = rxluhns2.recv().unwrap();


    // STEP V
    let step_5 = thread::spawn(move || {
        let results_is_valid = is_valid(results_luhns_step_1, results_luhns_step_2);
        txstep5.send(results_is_valid).unwrap();
    });
    let results_is_valid = rxstep5.recv().unwrap();

    step_1.join().unwrap();
    step_2.join().unwrap();
    step_3.join().unwrap();
    step_4.join().unwrap();
    step_5.join().unwrap();
    let card_vec_final = card_vec_unverified.clone();
    let mut results :Vec<( u64, String, bool, bool)>  = Vec::new();
    let mut is_valid = results_is_valid.iter();
    let mut brand = results_brand.iter();
    let mut length = results_length.iter();
    for card in card_vec_final {
       results.push( (card, brand.next().unwrap().to_string(), *is_valid.next().unwrap(), *length.next().unwrap()) );
    }


    //// TEST
    print_results(results);
}

fn print_results( results: Vec<( u64, String, bool, bool)>) {
    let width = 20;
    println!("{:width$}\t{}\t{}\t{}","Card Number","Issuer","Luhns" ,"Lenght");
    println!("--------------------\t------\t-----\t------");
    for card in results {
        if(card.1 != "None".to_string() && card.2 && card.3) {
            println!("{:width$}\t{}\t{}\t{}",card.0, card.1, card.2, card.3);
       }
    }
}

fn is_valid(vec1: Vec<u64>, vec2: Vec<u64>) -> Vec<bool> {

    let div_closure = | mut num :&u64| -> bool {
        if(num%10 == 0) {
            true
        } else {
            false
        }
    };
    let results: Vec<u64> =
                vec1.iter()
                .zip(vec2.iter())
                .map(|(&a, &b)| a + b)
                .collect();

    let final_results: Vec<bool> =
        results.iter()
        .map(div_closure).collect();



    //let mut r1 = vec1.iter();
    //let mut r2 = vec2.iter();
    //let mut results_is_valid :Vec<String> = Vec::new();

    //if((r1.next().unwrap()+r2.next().unwrap())%10 == 0) {
    //   results_is_valid.push(String::from("Valid"));
    //} else {
    //}
    final_results
}


fn luhns_step_2(card_collection: Vec<u64>) -> Vec<u64> {
    let mut results_step_2 :Vec<u64> = Vec::new();

    for item in card_collection {
        let new_vec = vectorize(item);
        let mut new_vec_iter = new_vec.iter();

        // if length is even
        if(new_vec.len() % 2 == 0) {
        let r2 :u64 = new_vec.iter().skip(1).step_by(2).sum::<u64>();
        results_step_2.push(r2);

        // if length is odd
        } else {
        let r2 :u64 = new_vec.iter().step_by(2).sum::<u64>();
        results_step_2.push(r2);
        }
    }
    results_step_2
}
fn luhns_step_1( card_collection: Vec<u64>) -> Vec<u64> {
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

    let mut results_step_1 :Vec<u64> = Vec::new();
    let mut card_collection_iter = card_collection.iter();

    for item in card_collection {
        let card = vectorize(item);
        let mut card_iter = card.iter();
        // if length is even
        if(card.len() % 2 == 0) {
        let vec_2: Vec<u64> = card_iter
                        .step_by(2)
                        .map(luhns_closure)
                        .collect();

        let r1 :u64 = vec_2.iter().sum::<u64>();
        results_step_1.push(r1);
        // if length is odd
        } else {
        let vec_2: Vec<_> = card_iter
                        .skip(1)
                        .step_by(2)
                        .map(luhns_closure)
                        .collect();

        let r1 :u64 = vec_2.iter().sum::<u64>();
        results_step_1.push(r1);
        }
    }
    results_step_1
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
    // Return exploded int as Vec<64> --a u32 would suffice
    vec
}

// Sort cards by brand
fn sort_by_brand(card_vec: Vec<u64>) -> Vec<String> {

    let mut results_brand :Vec<String> = Vec::new();

    let mut count_of_iin :u32 = 0;
    for element in &card_vec {
        //  Amex Range: 34, 37 Len 15
        if(((element / u64::pow(10, 13)) >= 34 &&
            (element / u64::pow(10, 13)) <= 37)) {
            results_brand.push(String::from("Amex"));
            count_of_iin += 1;
        }
        //  Visa Range: 4  Len 13, 16
        else if(((element / u64::pow(10, 12)) == 4 ||
            (element / u64::pow(10, 15)) == 4)) {
            results_brand.push(String::from("Visa"));
            count_of_iin += 1;
        }
        //  MC Range: 51-55  Len 16
        else if(((element / u64::pow(10, 14)) >= 51 &&
            (element / u64::pow(10, 14)) <=55 )) {
            results_brand.push(String::from("MC"));
            count_of_iin += 1;
        }
        //  Discover Range: 6  Len 16-19
        else if((
            (element / u64::pow(10, 15)) == 6 ||
            (element / u64::pow(10, 16)) == 6 ||
            (element / u64::pow(10, 17)) == 6 ||
            (element / u64::pow(10, 18)) == 6 )) {
            results_brand.push(String::from("Disc"));
            count_of_iin += 1;
        } else {
            results_brand.push(String::from("None"));
        }
    }
    println!("{} cards passed the IIN test", count_of_iin);
    return results_brand;
}

fn parse_input(content: &String) -> Vec<u64> {
    let mut card_vec = Vec::new();
    for line in content.lines() {
        card_vec.push(line.parse::<u64>().expect("Not a number"));
    }
    return card_vec
}


// Check how many characters long each number is
fn verify_length(card_vec: Vec<u64>) -> Vec<bool> {
    let mut count_invalid_size: u32 = 0;
    let mut count_valid_size: u32 = 0;

    // Holds a
    let mut length_result = Vec::new();
    for number in card_vec {
        // Smallest value for a CC and largest value
        // Had to use this as I am comparing int values
        if number >= 4000000000000 && number <= 6999999999999999999 {
            length_result.push(true);
            count_valid_size += 1;
        }
        else {
            length_result.push(false);
            count_invalid_size += 1;
        }
    }
    println!("Total entries {}", count_invalid_size + count_valid_size);
    println!("{} entries not between 13 and 19 characters", count_invalid_size);
    println!("{} entries potential cards", count_valid_size);
    return length_result;
}

// Used to check types cause they're complex in Rust
fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}



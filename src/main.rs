#![allow(unused)]

// Clap provides a library for handling CLI arguments
// Parser helps to wrap and handle errors
use clap::Parser;
use std::any::type_name;

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
    println!("Credit Card Validating App");

    let args = Cli::parse();

    // Might switch to std::io::BufReader
    let content = std::fs::read_to_string(&args.path).expect("could not read file");

    // Create Vector of length verified card numbers
    let mut card_vec_length_verified = verify_length(&content);

    let brands = sort_by_brand(card_vec_length_verified);

    let new_vec = vectorize(brands.amex[0]);

    print!("Amex ");
    for i in new_vec {
        let i: u64 = i;
        print!("{}", i);
      }
    println!("");
}

// Explode each number into a vector
fn vectorize(int: u64) -> Vec<u64> {
//    for a in brands.amex {
//        println!("Amex {}", );
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
    //print!("Amex ");
    //for i in vec {
    //    let i: u64 = i;
    //    print!("{}", i);
    //  }
    //println!("");
    //}
    vec
}

// Sort cards by brand
fn sort_by_brand(card_vec: Vec<u64>) -> Brand {
    let mut brands = Brand {
        amex: Vec::<u64>::new(),
        visa: Vec::<u64>::new(),
        mc: Vec::<u64>::new(),
        disc: Vec::<u64>::new()
    };

    for element in card_vec {
        let mut length :u32 = 0;
        //  Amex Range: 34, 37 Len 15
        if(((element / u64::pow(10, 13)) == 34 ||
            (element / u64::pow(10, 13)) == 37)) {
            brands.amex.push(element);
        };
        //  Visa Range: 4  Len 13, 16
        if(((element / u64::pow(10, 12)) == 4 ||
            (element / u64::pow(10, 15)) == 4)) {
            brands.visa.push(element);
        };
        //  MC Range: 51-55  Len 16
        if(((element / u64::pow(10, 14)) >= 51 &&
            (element / u64::pow(10, 14)) <=55 )) {
            brands.mc.push(element);
        };
        //  Discover Range: 6  Len 16-19
        if((
            (element / u64::pow(10, 15)) == 6 ||
            (element / u64::pow(10, 16)) == 6 ||
            (element / u64::pow(10, 17)) == 6 ||
            (element / u64::pow(10, 18)) == 6 )) {
            brands.disc.push(element);
        };
    }
    return brands;
}

//fn vectorize(int :u64)-> Vec<u32> {
//    let mut vec = Vec::new();
//}

// Check how many characters long each number is
fn verify_length(content: &String) -> Vec<u64> {
    let mut count: u32 = 0;
    // Holds a
    let mut card_vec = Vec::new();
    for line in content.lines() {
        if line.len() >= 13 && line.len() <= 19 {
            card_vec.push(line.parse::<u64>().expect("Not a number"));
        //    card_vec.push(line.to_string());
        //println!("{}", element.parse::<u64>().expect("Not a number"));
        }
        // Test output for
        else {
            count += 1;
        }
    }
    println!("{} entries not between 13 and 19 characters", count);
    return card_vec;
}

// Used to check types cause they're complex in Rust
fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}





//  // Using Modulo but Vec is in reverse and still leading zeros
//        for i in (0..19).rev() {
//            println!("{}", (input % 10));
//            vec.push(input%10);
//            prev = (input % 10);
//            input = (input - prev)/10;
//        }

//   For reference if I have to mess with Strings
//  // Sting -> char
//  for a in &brands.amex {
//      println!("Amex {}", a);
//      let word :String = a.to_string();
//      let mut count = word.chars().count();
//      println!("{}", type_of(count));
//      let mut chars = word.chars();
//      while(count > 0)  {
//          println!("{}", chars.next().unwrap());
//              println!("{}", chars.next().unwrap());// .to_digit(10).unwrap()));
//          count -= 1;
//      }
//  }

//    for a in &brands.amex {
//        println!("Amex {}", a);
//        let mut input = a;
//        let mut vec :Vec<u64> = Vec::new();
//        let mut prev :u64 = 0;
//        for i in (0..19).rev() {
//            println!("{}", (input % u64::pow(10, i)));
//
//            let temp = (input / u64::pow(10, i) - prev);
//            vec.push(temp);
//            prev += temp * 10;
//        }
//        for i in vec {
//            let i: u64 = i;
//            print!(" {} ", i);
//        }
//    }
//

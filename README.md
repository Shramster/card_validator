# card_validator
Rust CLI to verify CC numbers using Luhn's Algorithm

Source is located at /src/main.rs

1. Install Rust

2. clone repo and run:<br>
  `cargo build` <br>
  `cargo run -- $(cat ./cardnumbers)`<br>
  The program takes card numbers as command line arguments<br>
  as that was a requirement. To show sample output of I created <br>
  the file `./cardnumbers`, it contains example numbers and is passed <br>
  using `cat` command.

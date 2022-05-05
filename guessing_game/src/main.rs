use std::io;
fn main() {
    println!("Hello, world!");

    println!("Please input your guess");

    let mut guess = String::new(); 

    // first_world(guess);

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed : {}", guess);

    for number in (1..4).rev() {
        println!("{}!", number)
    }
}

// fn first_world(s: &String) -> usize{
//     let bytes = s.as_bytes();

//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return i;
//         }
//     }

//     s.len();
// }

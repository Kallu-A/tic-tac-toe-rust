use std::io::Write;
use crate::game::Player;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};


/// # Function to get a input of type A1 or A3
/// use by game will loop always until a move in '[A-B-C][1-2-3]' is make
pub fn get_input(player: Player) -> (char, usize){

    // create a stdout standard stream with color to better interface control
    let mut stdout = StandardStream::stdout(ColorChoice::Always);


    loop {
        // reset color at the beginning because of continue
        stdout.reset().expect("Error when reset stdout");

        // show syntax and to wich one is the turn
        println!("Syntax : [A-B-C][1-2-3]");
        print!("Player {} enter a number : ", player);

        // flush to let user make their input on the same line as below
        std::io::stdout().flush().expect("Error will flushing the terminal");

        // set color to red because now everything print will be error at this point
        stdout
            .set_color(ColorSpec::new().set_fg(Some(Color::Red)))
            .expect("Error when changing color ");

        let mut buffer = String::new();

        if std::io::stdin().read_line(&mut buffer).is_err() {
            println!("Can't read the line, please retry "); continue
        } else {

            let buffer = buffer.trim();
            if buffer.len() != 2 {
                println!("Bad number of value ! "); continue
            }

            // collect the first value of the buffer = column and put it to uppercase to avoid difference beetween a and A
            let col = &buffer.chars().next().unwrap().to_ascii_uppercase();
            // see if is in A..=C which is all valid column
            if ! ('A'..='C').contains(col)  {
                println!("The column can only be A, B or C"); continue
            }


            let line;
            // parse second value line in usize
            if let Ok(number) = buffer[1..=1].parse::<usize>() {
                if !(1..=3).contains(&number) {
                    println!("The line can only be 1, 2, 3"); continue
                }
                // here everything is good so we return
                line = number;
                stdout.reset().expect("Error when reset stdout");
                return (*col, line - 1)
            } else {
                println!("Put a value on the line 1, 2, 3"); continue
            }
        }

    };


}
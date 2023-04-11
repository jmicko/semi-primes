use std::io;
use std::io::Write;
use num_bigint::BigInt;

fn clear_console() {
    print!("\x1B[2J\x1B[1;1H");
    io::stdout().flush().unwrap();
}

fn main() {
    println!("Hello, world!");
    // get the user's choice of options
    let choice = display_options();
    println!("You chose {}", choice);
    // call the function to generate a semi-prime, based on the user's choice
    let semi_prime = get_semi_prime(choice);
    println!("The semi-prime is {}", semi_prime);

    // call the restart function to ask if the user would like to do it again
    restart();
}

fn restart(){
    // ask if the user would like to do it again
    // if the user enters 'y' or 'Y' or 1, then the program will run again
    // if the user enters 'n' or 'N', then the program will exit
    // if the user enters anything else, then the program will exit
    loop {
        println!("Would you like to do it again? (1/yes, 2/no)");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let input = input.trim();
        if input == "y" || input == "Y" || input == "yes" || input == "Yes" || input == "1" {
            clear_console();
            main();
        } else if input == "n" || input == "N" || input == "no" || input == "No" || input == "2" {
            println!("Exiting...");
            std::process::exit(0);
        } else {
            println!("You did not enter a valid choice. Exiting...");
            std::process::exit(0);
        }
    }

}

// function to display a number of options, and return the user's choice
fn display_options() -> u8 {
    println!("Options:");
    println!("1) Generate a semi-prime for me");
    println!("2) Enter a semi-prime myself");
    println!("3) Use 77");
    println!("4) Pick one of the RSA numbers");
    println!("\nPlease enter your choice:");
    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");
    let choice: u8 = choice.trim().parse().expect("Please type a number!");
    choice
}

// function to generate a semi-prime, based on the user's choice
fn get_semi_prime(choice: u8) -> BigInt {
    let semi_prime: BigInt;
    match choice {
        1 => semi_prime = generate_semi_prime(),
        // 2 => semi_prime = enter_semi_prime(),
        // 3 => semi_prime = 77,
        // 4 => semi_prime = pick_rsa_number(),
        // if the user enters a number that is not 1, 2, 3 or 4, then the program will exit
        _ => {
            println!("You did not enter a valid choice. Exiting...");
            std::process::exit(0);
        }
    }
    semi_prime
}

// function to generate a semi-prime
fn generate_semi_prime() -> BigInt {
    // clear the console
    clear_console();
    println!(
        "You chose to generate a semi-prime for you. \
    \nEnter a number, and the next two primes after that number will be found. \
    \nThese two primes will then be multiplied to create a semi-prime\n"
    );
    let mut input = String::new();
    loop {
        println!("Enter a number greater than 1:");
        // get the user's input, and convert it to a u64
        // let mut input = String::new();
        // read the user's input and store it in the input variable
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        // check that the user entered a number, and convert the input to a u64
        // if the user did not enter a number, the program will panic
        match input.trim().parse::<BigInt>() {
            Ok(num) => {
                if num > BigInt::from(1) {
                    println!("You entered {}", num);
                    input = num.to_string();
                    break;
                } else {
                    println!("You did not enter a number greater than 1!");
                }
            }
            Err(_) => {
                println!("You did not enter a number!");
                // show what was entered
                println!("You entered {}", input);
                // clear the input variable
                input = String::new();
            }
        }
    }
    // convert the user's input to a BigInt
    let input: BigInt = input.trim().parse().expect("Please type a number!");
    // find the next two primes after the user's input
    let prime1 = find_next_prime(&input);
    let prime2 = find_next_prime(&prime1);
    println!("The two primes are {} and {}", prime1, prime2);
    // multiply the two primes together to create a semi-prime
    let semi_prime = prime1 * prime2;
    semi_prime
}

// function to find the next prime after a given number
fn find_next_prime(number: &BigInt) -> BigInt {
    let mut prime: BigInt = number + 1;
    while !is_prime(&prime) {
        prime += 1;
        println!("{} is not prime", prime)
    }
    println!("{} is prime", prime);
    prime
}

// function to check if a number is prime
fn is_prime(num: &BigInt) -> bool {
    let two = BigInt::from(2);
    let three = BigInt::from(3);

    if *num == two || *num == three {
        return true;
    }
    if *num <= BigInt::from(0) || num % &two == BigInt::from(0) || num % &three == BigInt::from(0) {
        return false;
    }

    let mut i = BigInt::from(5);
    while &i * &i <= *num {
        if num % &i == BigInt::from(0) || num % (&i + BigInt::from(2)) == BigInt::from(0) {
            return false;
        }
        i += BigInt::from(6);
    }

    true
}

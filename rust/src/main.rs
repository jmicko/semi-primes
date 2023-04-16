mod primes;
mod utilities;

use duckdb::Connection;
use num_bigint::BigInt;
use num_traits::ToPrimitive;
use primes::find_factors;
use primes::find_next_prime;
use primes::generate_primes_bigint;
use primes::generate_primes_u128;
use std::io;
use utilities::clear_console;

// initialize the database connection so it can be used in the whole program
struct Database {
    conn: Connection,
}

impl Database {
    fn new() -> Self {
        let conn = Connection::open_in_memory().unwrap();
        Self { conn }
    }
}

fn main() {
    println!("Welcome to cpu killer\n");
    // get the user's choice of options
    let choice = display_options();
    println!("You chose {}", choice);

    if choice == 5 {
        // if the choice is 5, then the user wants to generate primes
        // ask for an upper limit
        println!("Upper limit of primes to generate?");
        let mut upper_limit = String::new();
        io::stdin()
            .read_line(&mut upper_limit)
            .expect("Failed to read line");
        let upper_limit = upper_limit.trim();
        let upper_limit = upper_limit.parse::<BigInt>().unwrap();

        // call the appropriate generate primes function to generate the primes
        match upper_limit.to_u128() {
            Some(upper_limit) => generate_primes_u128(upper_limit),
            None => generate_primes_bigint(upper_limit),
        };
        // call the restart function to ask if the user would like to do it again
        restart();
    } else {
        // all other choices will get a semi-prime and factor it
        // call the function to generate a semi-prime, based on the user's choice
        let semi_prime = get_semi_prime(choice);
        println!("The semi-prime is {}", semi_prime);

        // get a time stamp before the program starts so we can calculate how long it took
        let start = std::time::Instant::now();
        println!("time before: {:?}", start);

        // calculate the factors of the semi-prime using the find_factors function
        let factors = find_factors(&semi_prime);
        println!("The factors are {:?}", factors);

        // get a time stamp after the program ends so we can calculate how long it took
        let end = std::time::Instant::now();
        println!("time after:  {:?}", end);

        // calculate the time it took to run the program
        let duration = end.duration_since(start);
        println!("time elapsed: {:?}", duration);

        // call the restart function to ask if the user would like to do it again
        restart();
    }
}

fn restart() {
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
    println!("5) Generate primes");
    println!("\nPlease enter your choice:");
    let mut input = String::new();
    // read the user's input, if it is invalid, have them try again
    loop {
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        match input.trim().parse::<u8>() {
            Ok(num) => {
                println!("You entered {}", num);
                break num;
            }
            Err(_) => {
                println!("Bad input. Please enter a number.");
                input = String::new();
            }
        };
    }
}

// function to generate a semi-prime, based on the user's choice
fn get_semi_prime(choice: u8) -> BigInt {
    let semi_prime: BigInt;
    match choice {
        1 => semi_prime = generate_semi_prime(),
        2 => semi_prime = enter_semi_prime(),
        3 => semi_prime = BigInt::from(77),
        4 => semi_prime = pick_rsa_number(),
        // if the user enters a number that is not 1, 2, 3 or 4, then the program will exit
        _ => {
            clear_console();
            println!("You did not enter a valid choice. Try again...");
            // start over
            main();
            semi_prime = BigInt::from(0);
        }
    }
    semi_prime
}

// function to generate a semi-prime
fn generate_semi_prime() -> BigInt {
    // clear the console
    clear_console();
    println!(
        "You chose to generate a semi-prime for you. \n\
    \nEnter a number, and the next two primes after that number will be found. \
    \nThese two primes will then be multiplied to create a semi-prime.\
    \nBe aware that larger numbers may take a very long time.\n"
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
                    input = String::new();
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

// function to get a semi-prime from the user
fn enter_semi_prime() -> BigInt {
    // clear the console
    clear_console();
    println!(
        "You chose to enter a semi-prime yourself. \n\
        This should be a number that is the product of two prime numbers.\n"
    );
    let mut input = String::new();
    loop {
        println!("Enter a semi-prime:");
        // get the user's input, and convert it to a BigInt

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        // check that the user entered a number
        match input.trim().parse::<BigInt>() {
            Ok(num) => {
                if num >= BigInt::from(6) {
                    println!("You entered {}", num);
                    // convert the user's input to a BigInt
                    input = num.to_string();
                    break;
                } else {
                    println!("Bad input!");
                    input = String::new();
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
    // return the user's input as a BigInt
    let input: BigInt = input.trim().parse().expect("Please type a number!");
    input
}

// function to pick a semi-prime from the list of RSA numbers
fn pick_rsa_number() -> BigInt {
    // create a list of RSA numbers as an array of tuples
    const RSA_NUMS: [(u8, &str); 4] = [
        (100, "1522605027922533360535618378132637429718068114961380688657908494580122963258952897654000350692006139"),
        (110, "35794234179725868774991807832568455403003778024228226193532908190484670252364677411513516111204504060317568667"),
        (120, "227010481295437363334259960947493668895875336466084780038173258247009162675779735389791151574049166747880487470296548479"),
        (129, "114381625757888867669235779976146612010218296721242362562561842935706935245733897830597123563958705058989075147599290026879543541"),
    ];
    // clear the console
    clear_console();
    println!("Pick one of the RSA numbers:\n");
    // loop through the list of RSA numbers and print them
    for (i, (bits, _)) in RSA_NUMS.iter().enumerate() {
        println!("{}) RSA-{}", i + 1, bits);
    }
    let mut input = String::new();
    loop {
        println!("\nChoose an option:");
        // get the user's input, and convert it to a u8
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        // check that the user entered a number, and convert the input to a u8
        // if the user did not enter a number, the program will panic
        match input.trim().parse::<u8>() {
            Ok(num) => {
                if num > 0 && num < 5 {
                    println!("You entered {}", num);
                    // input = num.to_string();
                    // return the corresponding semi-prime
                    let semi_prime: BigInt = RSA_NUMS[num as usize - 1].1.parse().unwrap();
                    return semi_prime;
                } else {
                    println!("You did not enter a number from 1 to 4!");
                    input = String::new();
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
}

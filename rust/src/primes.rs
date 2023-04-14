// prims.rs: tools for working with primes

use crate::utilities::clear_console;
use num_bigint::BigInt;
use num_traits::Zero;
// use num_traits::One;
// use std::collections::HashMap;

// function to find the next prime after a given number
pub fn find_next_prime(number: &BigInt) -> BigInt {
    let mut prime: BigInt = number + 1;
    if &prime % 2 == BigInt::zero() {
        prime += 1;
    }
    let mut found_by_3: bool = false;
    let mut by_3_counter: bool = false;

    while !is_prime(&prime) {
        // println!("starting loop");
        if !found_by_3 {
            if &prime % 3 == BigInt::zero() {
                prime += 2;
                found_by_3 = true;
                by_3_counter = true;
            } else {
                prime += 2;
            }
        } else {
            if by_3_counter {
                // println!("adding 4 to not prime number");
                prime += 4;
                by_3_counter = false;
            } else {
                // println!("adding 2 to not prime number");
                prime += 2;
                by_3_counter = true;
            }
        }
        // println!("adding 1 to not prime number");
        // prime += 2;
        // println!("{} is not prime", prime)
    }
    // println!("{} is prime", prime);
    prime
}

// function to find the factors of a given semi-prime number
pub fn find_factors(semi_prime: &BigInt) -> (BigInt, BigInt) {
    let mut prime1: BigInt = BigInt::from(2);
    let prime2: BigInt;
    // let mut found = false;
    loop {
        clear_console();
        println!("prime1: {}", prime1);
        if semi_prime % &prime1 == BigInt::zero() {
            prime2 = semi_prime / &prime1;
            // if is_prime(&prime2) {
            // found = true;
            break;
            // }
        }
        // println!("finding next prime");
        prime1 = find_next_prime(&prime1);
    }
    (prime1, prime2)
}

#[derive(Debug)]
pub struct PrimePair {
    prime: BigInt,
    current_mod_value: BigInt,
    left_until_next: BigInt,
    add_this_and_continue: BigInt,
}

//                                                                      v 49 right here combines the two jumps into a 6
//                                            6   2   6   4   2   4   2   4   6   2   6   4   2   4   2   4
//                                            6   2   6   4   2   4   6       6   2   6   4   2   4   6
pub fn generate_primes(limit: BigInt) -> Vec<PrimePair> {
    // with a limit of 100, we should end up with this:
    // primes: [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47,     53, 59, 61, 67, 71, 73,     79, 83, 89, 97]
    //         [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47,     53, 59, 61, 67, 71, 73,     79, 83, 89, 97, 101, 103, 107, 109, 113, 127, 131, 137, 139, 149, 151, 157, 163, 167, 173, 179, 181, 191, 193, 197, 199, 211, 223, 227, 229, 233,
    //            1  2  2  4  2   4   2   4   6   2   6   4   2   4   2   4   6   2   6   4   2   4   2   4   6   8   4    2    4    2    4    14   4    6    2    10   2    6    6    4    6
    //            1  2  2  4  2   4   2   4   6   2   6   4   2   4     6     6   2   6   4   2     6     4   6   8   4    2    4    2    4    14   4    6    2    10   2    6    6    4    6
    //                      ^9                                             6  6   2   6   4   2     6     4   6  2+6  4    2    4    2    4   6+2+6 4   2+4   2   4+6   2    6   4+2   4   2+4   6    2   6+4   2    4    2   4+6+2  12
    //                                       2+4     2+4               2+4           4+2           4+2       2+4 4+4                                               ^ 11*13=143?
    //                      ^ 9               ^ 25                     ^ 49 would be here.            ^?  77?                                   ^ 121=11*11 would be here. and combines three jumps into a 14? or is it also 5 cubed?
    //                                        ^ here we break the +2 +4 +2 +4 pattern, maybe because we hit 25, which is the square of 5, the next prime number after 3
    //                                        ^ the next increment would have been 4, then 2, but since we hit this square number, we add the two increments together to get 6, a new pattern?
    //                                          Notable that we broke the +2 +2 +2 pattern at 9, which is the square of 3, the next prime number after 2
    //                                          Do we start a new pattern which breaks at the square of the next prime number? That would be 49, which is the square of 7
    //                                  okay but what about the pattern change right after 89? 10*10? 9*11?
    //                                  It's a plus 8, which is something we haven't seen yet
    //                                  but maybe it's because the pattern isn't numbers directly? But 8 = 6+2, or 4+2+2, or 2+2+2+2
    //                                  we would expect another 2 at the 25 pattern break, but we get a 6 instead, which is the number we expect, plus the next number we expect. Or 2+4
    // when the pattern breaks at 3 squared, the new pattern length is 2  --> 4 2 or is it 4? 4 2 4 2
    // when the pattern breaks at 5 squared, the new pattern length is 8? --> 6 2 6 4 2 4 2 4
    // when the pattern breaks at 7 squared, the new pattern length is    --> 6 2 6 4 2 4 6 === 6 2 6 4 2 4 6 4 6 8 4 2 4 2 4
    //
    // okay so, the pattern jumps when we hit a prime squared, or when we hit a prime * the next prime.
    // which would I think mean the pattern would get much longer every time that happens, so there will be diminishing returns vs just testing every number that comes up in a pattern
    // so we can use the pattern to eliminate a good amount of primes below the square root of a semi-prime, and then test the rest

    //  4 6 8 9 10 12 14 15 16 18 20 21 22 24 26 27 28 30 32 33 34 35 36 38 39 40 42 44 45 46 48 50 51 52 54 55 56
    // a prime number will not be needed to rule out a number as composite until the square of that prime number is reached? Is that true?
    // for example 2 rules out 10, 3 rules out 15, 2 and 4 rule out 20. 5 isn't needed until 25, which is the square of 5.
    // but we don't want to rule numbers out, we want to jump to the next prime number with as little computation as possible
    // okay and that's not even the only way to rule numbers out. 2 and 3 rule out 6, but 6 isn't the square of anything.

    // but this is why you only need to test numbers up to the square root of a number to see if it's prime

    // 2 and 3 line up every other number, so we can skip every other 2, and increase by 4 instead
    // wait is that just because 2 + 4 = 6, and 2 * 3 = 6?
    // start with 2
    // add 2 to get 4, which we rule out by 2s
    // but then we can jump 4 to get to 8, skipping the 6 check because we already know 2 & 3 line up on 6
    // squares of prime numbers only have 1 prime factor, and it will be the first time that prime factor is used
    // was onto something with the squares of primes and multiples of those squares. Sieve of Atkin uses that strategy
    // maybe a sieve is the best strategy, and just use the pattern to eliminate a good amount of numbers first

    let mut primes: Vec<PrimePair> = Vec::new();
    let mut primes_only: Vec<BigInt> = Vec::new();
    let mut uncaught_composites: Vec<BigInt> = Vec::new();

    let mut current_num: BigInt = BigInt::from(3);
    // let jump_distance: Vec<u8> = vec![6, 2, 6, 4, 2, 4, 2, 4];
    let mut jump_distance: Vec<u8> = vec![2];
    let mut jump_index = 0;

    let two_pair = PrimePair {
        prime: BigInt::from(2),
        current_mod_value: BigInt::from(1),
        left_until_next: BigInt::from(1),
        add_this_and_continue: BigInt::zero(),
    };
    primes.push(two_pair);

    let start = std::time::Instant::now();
    // current_num += 1;
    let mut total_to_add = BigInt::zero();

    while &current_num <= &limit {
        let mut current_num_is_prime = true;
        for mut prime_pair in &mut primes {

            
            // set the current_mod_value to be (current_num +1) % prime_pair.prime
            // println!("current jump distance: {}", jump_distance[jump_index]);
            prime_pair.current_mod_value =
                (&prime_pair.current_mod_value + jump_distance[jump_index]) % &prime_pair.prime;


            if prime_pair.add_this_and_continue != BigInt::zero() {
                // println!("adding {} to total_to_add", prime_pair.add_this_and_continue);
                total_to_add += prime_pair.add_this_and_continue.clone();
                prime_pair.add_this_and_continue = BigInt::zero();
            }
                

            // println!("current_num: {}, prime_pair.prime: {}, current_mod_value: {}", current_num, prime_pair.prime, prime_pair.current_mod_value);
            if current_num_is_prime && prime_pair.current_mod_value == BigInt::zero() {
                // println!(
                //     "current num is not prime: {} , current_mod_value: {}, jump_distance: {}",
                //     current_num, prime_pair.current_mod_value, jump_distance[jump_index]
                // );
                current_num_is_prime = false;
                uncaught_composites.push(current_num.clone());
                // at this point we save the current total to add in the prime_pair.add_this_and_continue
                // then we can break out of the loop
                prime_pair.add_this_and_continue = total_to_add.clone();
            }


        }


        total_to_add = BigInt::zero();
        // println!("current_num_is_prime: {}", &current_num_is_prime);
        if current_num_is_prime {
            let new_prime_pair = PrimePair {
                prime: current_num.clone(),
                current_mod_value: BigInt::zero(),
                left_until_next: current_num.clone(),
                add_this_and_continue: BigInt::zero(),
            };
            clear_console();
            println!("Found a prime: {}", current_num);
            primes.push(new_prime_pair);
        }

        // need to switch to the new pattern here
        // I think there's a way to algorithmically figure out the pattern, but I'm not sure how to do it yet
        if &current_num == &BigInt::from(23) {
            // println!("current_num: {}", current_num);
            jump_distance = vec![6, 2, 6, 4, 2, 4, 2, 4];
            jump_index = jump_distance.len() - 1;
        }

        // need to keep index in bounds
        jump_index = (jump_index + 1) % jump_distance.len();

        // current_num += 2;
        // need to keep track of the jump distance
        current_num += jump_distance[jump_index];
    }
    let duration = start.elapsed();

    // println!("primes: {:?}", primes);
    for prime_pair in &primes {
        // println!("prime: {}, current_mod_value: {}", prime_pair.prime, prime_pair.current_mod_value);
        primes_only.push(prime_pair.prime.clone());
    }
    println!("uncaught_composites: {:?}", uncaught_composites);
    println!("primes_only: {:?}", primes_only);
    println!("Time elapsed in generate_primes() is: {:?}", duration);
    println!("total primes: {}", primes_only.len());
    primes
}

// pub fn find_next_prime(number: &BigInt) -> BigInt {
//     let limit = number + BigInt::one();
//     let primes = sieve_of_eratosthenes(&limit);
//     for prime in primes {
//         if prime > *number {
//             return prime;
//         }
//     }
//     // If no prime is found, return a default value (e.g., -1) or handle the error in your desired way
//     BigInt::from(-1)
// }

// fn sieve_of_eratosthenes(limit: &BigInt) -> Vec<BigInt> {
//     let mut is_prime: HashMap<BigInt, bool> = HashMap::new();
//     let mut primes = Vec::new();
//     let mut p = BigInt::from(2);

//     while &p * &p <= *limit {
//         println!("first loop, p: {}", p);
//         if !is_prime.contains_key(&p) {
//             primes.push(p.clone());
//             let mut i = &p * &p;
//             while i <= *limit {
//                 // if &i % 10000 == BigInt::from(0) {
//                 println!("second loop, i: {}", i);
//                 // }
//                 is_prime.insert(i.clone(), false);
//                 i += &p;
//             }
//         }
//         p += BigInt::one();
//     }

//     while p <= *limit {
//         println!("third loop, p: {}", p);
//         if !is_prime.contains_key(&p) {
//             primes.push(p.clone());
//         }
//         p += BigInt::one();
//     }

//     println!("primes: {:?}", primes);
//     primes
// }

// function to check if a number is prime
fn is_prime(num: &BigInt) -> bool {
    // println!("checking if {} is prime", num);
    let two = BigInt::from(2);
    let three = BigInt::from(3);

    // println!("{} == {} || {} == {}", *num, two, *num, three);

    // println!("point 1");
    if *num == two || *num == three {
        // println!("true 1");
        return true;
    }
    // check if the last digit is 5
    // all other numbers have been eliminated by this point
    // first convert the number to a string, then get the last character
    // this may not be faster for small numbers, but it should be faster for large numbers
    let last_digit = num.to_string().chars().last().unwrap();
    // println!("last digit: {}", last_digit);
    if last_digit == '5' {
        // println!("false 0");
        return false;
    }
    // println!("point 2");
    if *num <= BigInt::zero() || num % &two == BigInt::zero() || num % &three == BigInt::zero() {
        // println!("false 1");
        return false;
    }

    // println!("point 3");
    let mut i = BigInt::from(5);
    while &i * &i <= *num {
        if &i % 60 == BigInt::zero() {
            // println!("point 4, start of loop {} * {} <= {}", i, i, num);
        }
        if num % &i == BigInt::zero() || num % (&i + BigInt::from(2)) == BigInt::zero() {
            // println!("false 2");
            return false;
        }
        i += BigInt::from(6);
    }

    // println!("true 2");
    true
}

/*
need to be able to handle a big number like this
656176567519865571863455146635518656574864651732451676551866874865561764255166355617656517634551766534561786456547186455166455476645261786346575186561
*/

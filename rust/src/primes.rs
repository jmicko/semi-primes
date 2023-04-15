// prims.rs: tools for working with primes

use crate::utilities::clear_console;
use num_bigint::BigInt;
use num_traits::One;
use num_traits::ToPrimitive;
use num_traits::Zero;
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

// #[derive(Debug)]
pub struct PrimePairBigInt {
    prime: BigInt,
    current_mod_value: BigInt,
    left_until_next: BigInt,
    add_this_and_continue: BigInt,
}
// #[derive(Debug)]
pub struct PrimePair {
    prime: u128,
    current_mod_value: u128,
    left_until_next: u8,
    add_this_and_continue: u128,
}

pub enum PrimePairOrBigInt {
    Pair(Vec<PrimePair>),
    BigIntPair(Vec<PrimePairBigInt>),
}

//                                                                      v 49 right here combines the two jumps into a 6
//                                            6   2   6   4   2   4   2   4   6   2   6   4   2   4   2   4
//                                            6   2   6   4   2   4   6       6   2   6   4   2   4   6

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

// holy codebro man, if I just add the numbers regardless of primality and only based on the pattern,
// for 100_000 limit, it finishes in 2s vs 340s +- 20s if I check each one for primality
// this ends with ~26k numbers in the list, vs 9592 actual primes below 100_000
// maybe a sieve would work better?
// actually, if looking for factors of semi-primes, it's not mandatory to use primes only.
// If it takes longer to eliminate a number as composite than it would take to test it as a factor, then it's better to just test it as a factor
// using the pattern to cut out 3/4 of all numbers saves a lot of time on it's own
// when looking for factors of the RSA numbers, human error is also a factor.
// "For better security", RSA recommends not to us a very samll prime
// Which means there's no need to test small primes. Can skip a lot of numbers that way
// there's no mathematical way to asses the limit of "a small prime" though, right? So it's based on feelings
//
// for example, the RSA numbers and solutions:
// RSA-100
// 35794234179725868774991807832568455403003778024228226193532908190484670252364677411513516111204504060317568667
// solutions:
// 6122421090493547576937037317561418841225758554253106999
// 5846418214406154678836553182979162384198610505601062333
//
// RSA-120
// 227010481295437363334259960947493668895875336466084780038173258247009162675779735389791151574049166747880487470296548479
// solutions:
// 327414555693498015751146303749141488063642403240171463406883
// 693342667110830181197325401899700641361965863127336680673013
// RSA-129
// 114381625757888867669235779976146612010218296721242362562561842935706935245733897830597123563958705058989075147599290026879543541
// solutions:
// 3490529510847650949147849619903898133417764638493387843990820577
// 32769132993266709549961988190834461413177642967992942539798288533
//
// RSA-160
// 2152741102718889701896015201312825429257773588845675980170497676778133145218859135673011059773491059602497907111585214302079314665202840140619946994927570407753
// solutions:
// 45427892858481394071686190649738831656137145778469793250959984709250004157335359
// 47388090603832016196633832303788951973268922921040957944741354648812028493909367
//
// In each case, both numbers contain half the number of digits of the RSA number
// If the RSA number has an odd number of digits, they split the difference. One factor is 1 digit longer to make up for it
// So for example in the case of RSA-100, you only need to test the numbers between
// 1000000000000000000000000000000000000000000000000000000
// and
// 9999999999999999999999999999999999999999999999999999999
//
// Also, while possible, it is unlikely that the first digit of the factors will be 1
// So with RSA-100 for example, you could probably start at
// 2000000000000000000000000000000000000000000000000000000
//
// Similarily however, it is extremely unlikely the there are a bunch of 0s in a row
// still possible to have 0s tho
// There are 10 possible digits 0-9, so probably the numbers will consist of 1/10 of each digit
// statistically extremely unlikely that say, 1/3 of the digits are any given digit
// Could rule out a lot of numbers without any math simply by counting the number of each digit, and testing against the length of the number
// 49+7 in base 50 = 16. 1+6 is 7, so 16 (56 -> base 50) is divisible 7 in base 50
// because when you carry the 1, you are representing a multiple of 7 and subtracting 1 from the number you increase by
// that's why the trick works where you add the digits and if they = 3, 6, or 9, then it's divisible by 3
// not sure if converting numbers to base 7 is computationally easy tho, but my understanding is they convert to binary anyway? But maybe not with the BigInt library?
// using mod is probably faster or at least fast enough, and def easier

pub fn generate_primes(upper_limit: BigInt) -> PrimePairOrBigInt {
    let mut primes: Vec<PrimePairBigInt> = Vec::new();
    let mut primes_only: Vec<BigInt> = Vec::new();
    let mut uncaught_composites: Vec<BigInt> = Vec::new();

    let mut current_num: BigInt = BigInt::from(3);
    // let jump_distance: Vec<u8> = vec![6, 2, 6, 4, 2, 4, 2, 4];
    let mut jump_distance: Vec<u8> = vec![2];
    let mut jump_index = 0;

    let two_pair = PrimePairBigInt {
        prime: BigInt::from(2),
        current_mod_value: BigInt::one(),
        left_until_next: BigInt::one(),
        add_this_and_continue: BigInt::zero(),
    };
    primes.push(two_pair);

    let start = std::time::Instant::now();
    // current_num += 1;
    let mut total_to_add = BigInt::zero();

    while current_num <= upper_limit {
        let mut current_num_is_prime = true;

        total_to_add += jump_distance[jump_index];

        // don't need to check numbers below the square root of the current number
        // let mut check_until = current_num.clone();
        // let check_until = (current_num.clone() as f64).sqrt() as BigInt + 1;
        // println!("!!!!!!!!!!check_until: {}, current_num: {}", check_until, current_num);

        let primes_length = primes.len();
        let mut mod_shift = BigInt::zero();

        for (index, mut prime_pair) in primes.iter_mut().enumerate() {
            // set the current_mod_value to be (current_num +1) % prime_pair.prime
            prime_pair.current_mod_value =
                (&prime_pair.current_mod_value + total_to_add.clone()) % &prime_pair.prime;

            if prime_pair.add_this_and_continue != BigInt::zero() {
                total_to_add += prime_pair.add_this_and_continue.clone();
                prime_pair.add_this_and_continue = BigInt::zero();
            }

            if current_num_is_prime && prime_pair.current_mod_value == BigInt::zero() {
                current_num_is_prime = false;
                // at this point we save the current total to add in the prime_pair.add_this_and_continue
                // then we can break out of the loop
                prime_pair.add_this_and_continue = total_to_add.clone();
                break;
            }
            if prime_pair.prime.clone() * prime_pair.prime.clone() > current_num {
                // only do this if we are not on the last iteration of the loop
                if index != &primes_length - 1 {
                    prime_pair.add_this_and_continue = total_to_add.clone();
                    mod_shift += total_to_add.clone();
                } else if index == &primes_length - 1 {
                    prime_pair.add_this_and_continue = BigInt::zero();
                }
                break;
            }
        }
        // println!("broke out of loop, current_num_is_prime: {}", current_num_is_prime);

        total_to_add = BigInt::zero();
        // println!("current_num_is_prime: {}", &current_num_is_prime);
        if current_num_is_prime {
            // I this this is the source of the problem
            // if the loop breaks when it reaches the square root of the current number,
            // then there will be an "add_this_and_continue" value earlier in the vec
            // that should not be added to the new prime_pair
            // maybe could fix this be pre-modding the current_mod_value by the prime number minus whatever the add_this_and_continue value is
            let shifted_mod: BigInt = (current_num.clone() - mod_shift) % current_num.clone();
            // let shifted_mod = ((current_num.clone() % current_num.clone()) - mod_shift) % current_num.clone();
            // that way, when the add_this value comes along, it will be offset by the mod_shift
            let new_prime_pair: PrimePairBigInt = PrimePairBigInt {
                prime: current_num.clone(),
                // current_mod_value: BigInt::zero(),
                current_mod_value: shifted_mod,
                left_until_next: current_num.clone(),
                add_this_and_continue: BigInt::zero(),
            };
            if &current_num % 600 == BigInt::from(23) {
                clear_console();
                println!("Found a prime: {}", current_num);
            }
            primes.push(new_prime_pair);
        }

        // need to switch to the new pattern here
        // I think there's a way to algorithmically figure out the pattern, but I'm not sure how to do it yet
        if current_num == BigInt::from(23) {
            jump_distance = vec![6, 2, 6, 4, 2, 4, 2, 4];
            jump_index = jump_distance.len() - 1;
        }

        // need to keep index in bounds
        jump_index = (jump_index + 1) % jump_distance.len();

        // current_num += 2;
        // need to keep track of the jump distance
        // println!("======current_num: {}, jump_distance: {}", current_num, jump_distance[jump_index]);
        current_num += jump_distance[jump_index];
    }
    let duration = start.elapsed();

    // println!("primes: {:?}", primes);
    for prime_pair in &primes {
        primes_only.push(prime_pair.prime.clone());
    }
    // println!("uncaught_composites: {:?}", uncaught_composites);
    // println!("primes_only: {:?}", primes_only);
    println!("Time elapsed in generate_primes() is: {:?}", duration);
    println!("total primes: {}", primes_only.len());
    // primes
    PrimePairOrBigInt::BigIntPair(primes)
}

pub fn generate_primes_unumtype(upper_limit: u128) -> PrimePairOrBigInt {
    let mut primes: Vec<PrimePair> = Vec::new();
    let mut primes_only: Vec<u128> = Vec::new();
    let mut uncaught_composites: Vec<u128> = Vec::new();

    let mut current_num: u128 = 3;
    let mut jump_distance: Vec<u128> = vec![2];
    let mut jump_index = 0;

    let mut longest_prime_gap = 0;

    let two_pair = PrimePair {
        prime: 2,
        current_mod_value: 1,
        left_until_next: 1,
        add_this_and_continue: u128::zero(),
    };
    primes.push(two_pair);

    let start = std::time::Instant::now();
    // current_num += 1;
    let mut total_to_add = u128::zero();

    // create an empty vector of u8s

    while current_num <= upper_limit {
        let mut current_num_is_prime = true;

        let current_jump = jump_distance[jump_index];

        total_to_add += current_jump;

        let primes_length = primes.len();
        let mut mod_shift = 0;

        for (index, mut prime_pair) in primes.iter_mut().enumerate() {
            // set the current_mod_value to be (current_num +1) % prime_pair.prime
            prime_pair.current_mod_value =
                (&prime_pair.current_mod_value + total_to_add.clone()) % &prime_pair.prime;

            // set the left_until_next to be prime_pair.prime - current_mod_value
            let left_until_next = &prime_pair.prime - &prime_pair.current_mod_value;
            // cast it as a u8. If it can't be cast, then set it to 0
            let left_until_next_u8 = left_until_next.to_u8().unwrap_or(0);
            // now save it to the prime_pair
            prime_pair.left_until_next = left_until_next_u8;

            if prime_pair.add_this_and_continue != u128::zero() {
                total_to_add += prime_pair.add_this_and_continue.clone();
                prime_pair.add_this_and_continue = u128::zero();
            }

            // println!("current_num: {}, prime_pair.prime: {}, current_mod_value: {}", current_num, prime_pair.prime, prime_pair.current_mod_value);
            if current_num_is_prime && prime_pair.current_mod_value == u128::zero() {
                current_num_is_prime = false;
                uncaught_composites.push(current_num.clone());
                // at this point we save the current total to add in the prime_pair.add_this_and_continue
                // then we can break out of the loop
                prime_pair.add_this_and_continue = total_to_add.clone();
                break;
            }
            if prime_pair.prime * prime_pair.prime > current_num {
                // only do this if we are not on the last iteration of the loop
                if index != &primes_length - 1 {
                    prime_pair.add_this_and_continue = total_to_add.clone();
                    mod_shift += total_to_add.clone();
                } else if index == &primes_length - 1 {
                    prime_pair.add_this_and_continue = u128::zero();
                }
                break;
            }
        }

        total_to_add = u128::zero();

        if current_num_is_prime {
            // if we stop at the square root of the current num without finding a factor, then it is prime
            // but we need to shift the mod value since we are adding it to the vec
            let shifted_mod: u128 = (current_num.clone() - mod_shift) % current_num.clone();
            // that way, when the add_this value comes along, it will be offset by the mod_shift

            // set the left_until_next to be prime_pair.prime - current_mod_value
            let left_until_next = current_num.clone() - shifted_mod.clone();
            // cast it as a u8. If it can't be cast, then set it to 0
            let left_until_next_u8 = left_until_next.to_u8().unwrap_or(0);

            let new_prime_pair: PrimePair = PrimePair {
                prime: current_num.clone(),
                current_mod_value: shifted_mod,
                left_until_next: left_until_next_u8,
                add_this_and_continue: u128::zero(),
            };
            if &current_num % 600 == 23 {
                // printing to the console uses resources, so we don't want to do it very often
                clear_console();
                println!("Found a prime: {}", current_num);
            }
            // find the gap between current_num and the last prime in the primes vec
            let gap = current_num - primes[primes.len() - 1].prime;
            // if the gap is larger than the largest gap, update the largest gap
            if gap > longest_prime_gap {
                longest_prime_gap = gap;
            }
            primes.push(new_prime_pair);
        }

        // need to switch to the new pattern here
        // I think there's a way to algorithmically figure out the pattern, but I'm not sure how to do it yet
        if current_num == 23 {
            jump_distance = vec![6, 2, 6, 4, 2, 4, 2, 4];
            jump_index = jump_distance.len() - 1;
        }

        // need to keep index in bounds
        jump_index = (jump_index + 1) % jump_distance.len();
        // need to keep track of the jump distance
        current_num += jump_distance[jump_index];
    }
    let duration = start.elapsed();

    // println!("primes: {:?}", primes);
    for prime_pair in &primes {
        // println!("prime: {}, current_mod_value: {}", prime_pair.prime, prime_pair.current_mod_value);
        primes_only.push(prime_pair.prime.clone());
    }
    println!("uncaught_composites: {:?}", uncaught_composites.len());
    // println!("primes_only: {:?}", primes_only);
    println!("Time elapsed in generate_primes() is: {:?}", duration);
    println!("total primes: {}", primes_only.len());
    println!("Largest gap: {}", longest_prime_gap);
    PrimePairOrBigInt::Pair(primes)
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

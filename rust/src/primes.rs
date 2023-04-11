// prims.rs: tools for working with primes

use num_bigint::BigInt;
use num_traits::Zero;
// use num_traits::One;
// use std::collections::HashMap;

// function to find the next prime after a given number
pub fn find_next_prime(number: &BigInt) -> BigInt {
    let mut prime: BigInt = number + 1;
    while !is_prime(&prime) {
        println!("adding 1 to not prime number");
        prime += 1;
        println!("{} is not prime", prime)
    }
    println!("{} is prime", prime);
    prime
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
    println!("checking if {} is prime", num);
    let two = BigInt::from(2);
    let three = BigInt::from(3);

    // println!("{} == {} || {} == {}", *num, two, *num, three);

    println!("point 1");
    if *num == two || *num == three {
        println!("true 1");
        return true;
    }
    println!("point 2");
    if *num <= BigInt::zero() || num % &two == BigInt::zero() || num % &three == BigInt::zero() {
        println!("false 1");
        return false;
    }

    println!("point 3");
    let mut i = BigInt::from(5);
    while &i * &i <= *num {
        if &i % 60 == BigInt::zero() {
            println!("point 4, start of loop {} * {} <= {}", i, i, num);
        }
        if num % &i == BigInt::zero() || num % (&i + BigInt::from(2)) == BigInt::zero() {
            println!("false 2");
            return false;
        }
        i += BigInt::from(6);
    }

    println!("true 2");
    true
}

/*
need to be able to handle a big number like this
656176567519865571863455146635518656574864651732451676551866874865561764255166355617656517634551766534561786456547186455166455476645261786346575186561
*/

# Semi-Primes

## Description
This is a simple CLI tool to find prime factors of semi-prime numbers. This is an important topic in the field of cryptography, because semi-primes are used to generate RSA keys. The purpose of this project is to demonstrate how quickly a semi-prime can be generated, and to contrast that with how long it takes to then factorize it.

RSA encryption is currently very secure with todays computing power, however, it is known that quantum computers will soon be able to factorize semi-primes in a very short time. This may happen in as little as 20 years, possibly less. Anything that has been encrypted with RSA and stored by a third party will be easily decrypted if the party has access to a quantum computer. This is why it is important to move away from RSA encryption as soon as possible.

But that's not the point of this project. The point was to learn Rust. 

## Usage
There are two subfolers in the project. One is the original, written in JavaScript. The other is the Rust version. The rust version is notably faster, but that's because it uses a different algorithm. The JavaScript version uses an algorithm that quantum computers could implement very quickly, while the Rust version uses a simpler algorithm that is faster on modern computers.

Starting either program will display a number of options. The first option will demonstrate the time to compute a semi-prime vs the time to factorize that same semi-prime. You can also enter your own semi-prime, or pick one from a list. The algorithm used to factorize the semi-prime is the same for each option, but different for each language.

## Conclusion
Did I learn a lot about Rust? Yes I did. Would I recommend running this program on your computer? No, I would not, unless you need an electric heater and want to dry out your thermal compound and put wear on your PC components. But this is a single-threaded program, so you would probably be better off with a tool like Prime95 in that case.
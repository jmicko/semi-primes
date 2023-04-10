function raiseToPower(base, power) {
  let result = BigInt(1);
  for (let i = 0; i < power; i++) {
    result *= BigInt(base);
  }
  return result;
}

// https://en.m.wikipedia.org/wiki/Primality_test
function isPrime(num) {
  if (num == 2 || num == 3)
    return true;
  if (num <= 1 || num % 2 == 0 || num % 3 == 0)
    return false;
  for (let i = 5; i * i <= num; i += 6)
    if (num % i == 0 || num % (i + 2) == 0)
      return false;
  return true;
}

// function that will take a number and return the next two prime numbers
function nextTwoPrimes(num) {
  let nextPrime = Number(num) + 1;
  while (!isPrime(nextPrime)) {
    nextPrime++;
  }
  let nextNextPrime = nextPrime + 1;
  while (!isPrime(nextNextPrime)) {
    nextNextPrime++;
  }
  return [nextPrime, nextNextPrime];
}

// const nextPrimesOf = 516;
// const nextPrimes = nextTwoPrimes(nextPrimesOf);
// console.log('nextPrimes: ', nextPrimes);

// const semiprime = nextPrimes[0] * nextPrimes[1];
// console.log('semiprime: ', semiprime);

async function askUserForNumber() {
  console.clear();
  const readline = require('readline').createInterface({
    input: process.stdin,
    output: process.stdout
  });
  // 3 options: 1) generate a semi-prime for me, 2) enter a semi-prime, 3) use 77
  readline.question(
    `Options:\
    \n1) Generate a semi-prime for me\
    \n2) Enter a semi-prime myself\
    \n3) Use 77\

    \nChoose an option: `,
    (answer) => {

      if (answer == 1) {
        // ask user for a number and then find the next two primes
        readline.question('Enter a number greater than 1: ', (answer) => {
          // make sure answer is > 1
          if (answer <= 1) {
            console.log('invalid input, start over in 3 seconds');
            readline.close();
            setTimeout(() => {
              askUserForNumber();
            }, 3000);
            return;
          }
          const nextPrimes = nextTwoPrimes(answer);
          console.log('nextPrimes: ', nextPrimes);
          const semiprime = nextPrimes[0] * nextPrimes[1];
          console.log('semiprime: ', semiprime);
          readline.close();
          // return semiprime;
          console.log('calling main')
          main(semiprime);
        });
      } else if (answer == 2) {
        // ask user for a semi-prime
        readline.question('Enter a semi-prime: ', (semiP) => {
          console.log('semiprime: ', semiP);
          readline.close();
          // return semiP;
          console.log('calling main')
          main(semiP);
        });
      } else if (answer == 3) {
        // use 77
        console.log('semiprime: ', 77);
        readline.close();
        // return 77;
        console.log('calling main')
        main(77);
      // } else if (answer == 4) {
      //   // pick one of the RSA numbers
      //   readline.question('Pick one of the RSA numbers: ', (answer) => {

      //   });
      } else {
        console.log('invalid input');
        readline.close();
        askUserForNumber();
      }
    });
}
// main();
askUserForNumber();

async function main(n) {
  // const number = await askUserForNumber();
  const number = BigInt(n);
  console.log('number: ', number);
  const result = badGuess(number, 2);

  let guess = result[0];
  let power = result[1];
  console.log(`${guess} to the power of ${power} is 1 more than a multiple of ${number}`);

  const shareFactorsWithNumber = twoEquations(guess, power);
  // console.log('equations: ', shareFactorsWithNumber);

  const firstNum = shareFactorsWithNumber[0];

  console.log('calculating factors of ' + number + '...');
  // const factors = factor(number); // [3, 5]
  // console.log(factors);


  const factor1 = euclid(firstNum, number);

  console.log('factor1: ', factor1);

  // now find the other factor by dividing the number by the first factor
  const factor2 = number / factor1;

  console.log('factor2: ', factor2);
  // askUserForNumber();
  // ask user if they want to factor another number
  // if yes, call askUserForNumber()
  // if no, exit
  doAgain();
  function doAgain() {
    const readline = require('readline').createInterface({
      input: process.stdin,
      output: process.stdout
    });
    readline.question('Would you like to factor another number? (y/n): ', (answer) => {
      if (answer == 'y') {
        readline.close();
        askUserForNumber();
      } else if (answer == 'n') {
        readline.close();
        process.exit();
      } else {
        console.log('invalid input');
        readline.close();
        doAgain();
      }
    });
  }
}
// const number = askUserForNumber();
// const number = BigInt(semiprime);

// console.log('calculating factors of ' + number + '...');

// const factors = factor(number); // [3, 5]
// console.log(factors);

function badGuess(n, g) {
  let guess = g;
  let power = 1;

  while (true) {
    // console.log('power: ', power);
    // raise guess to the power
    let guessToPower = raiseToPower(guess, power);
    // console.log('guessToPower: ', guessToPower);
    // now find remainder of guessToPower divided by n
    let remainder = BigInt(guessToPower) % n;
    // console.log('remainder: ', remainder);
    if (remainder == 1) {
      // console.log('guess: ', guess);
      // console.log('power: ', power);
      return [guess, power];
    }
    power++;
  }
}

// const result = badGuess(number, 8);

// console.log('result: ', result);
// let guess = result[0];
// let power = result[1];
// console.log(`${guess} to the power of ${power} is 1 more than a multiple of ${number}`);

// now run these two equations on guess and power
// (guess ^ (power/2)) + 1
// (guess ^ (power/2)) - 1
function twoEquations(guess, power) {
  let guessToPower = raiseToPower(guess, power);
  let guessToHalfPower = raiseToPower(guess, power / 2);
  let first = guessToHalfPower + BigInt(1);
  let second = guessToHalfPower - BigInt(1);
  return [first, second];
}

// const shareFactorsWithNumber = twoEquations(guess, power);
// // console.log('equations: ', shareFactorsWithNumber);

// const firstNum = shareFactorsWithNumber[0];
// const secondNum = shareFactorsWithNumber[1];

// run euclid's algorithm on the two numbers to find the shared factors
function euclid(a, b) {
  let remainder = a % b;
  if (remainder == 0) {
    return b;
  } else {
    return euclid(b, remainder);
  }
}

// const factor1 = euclid(firstNum, number);

// console.log('factor1: ', factor1);

// // now find the other factor by dividing the number by the first factor
// const factor2 = number / factor1;

// console.log('factor2: ', factor2);
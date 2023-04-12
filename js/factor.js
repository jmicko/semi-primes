const interface = [
    'Welcome to cpu killer\n',
    // this can save lines that should show up after console.clear()
];

async function refreshInterface(newLine) {
    console.clear();
    for (let i = 0; i < interface.length; i++) {
        console.log(interface[i]);
    }
    if (newLine) {
        console.log(newLine);
    }
}

function clearInterface() {
    interface.length = 0;
    interface.push('Welcome to cpu killer\n');
    console.log('interface: ', interface);
    refreshInterface();
}

// function to pause for x milliseconds in any async function
function sleep(milliseconds) {
    return new Promise(resolve => setTimeout(resolve, milliseconds))
}

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
    for (let i = 5; i * i <= num; i += 6) {
        if (num % i == 0 || num % (i + 2) == 0) {
            return false;
        }
    }
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
    // console.clear();
    clearInterface();
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
    \n4) Pick one of the RSA numbers
    \nChoose an option: `,
        (answer) => {

            if (answer == 1) {
                refreshInterface('You chose to generate a semi-prime for you. \
        \nEnter a number, and the next two primes after that number will be found. \
        \nThese two primes will then be multiplied to create a semi-prime\n');
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
                    interface.push(`The next two primes are: , ${nextPrimes[0]}, ${nextPrimes[1]}\n`);
                    refreshInterface();
                    // console.log('nextPrimes: ', nextPrimes);
                    const semiprime = nextPrimes[0] * nextPrimes[1];
                    interface.push(`The semi-prime is: ${semiprime}\n`);
                    refreshInterface();
                    // console.log('semiprime: ', semiprime);
                    readline.close();
                    // return semiprime;
                    setTimeout(() => {
                        clearInterface();
                        console.log('calling main')
                        main(semiprime);
                    }, 3000);
                });
            } else if (answer == 2) {
                refreshInterface('You chose to enter a semi-prime yourself. This should be a number that is the product of two prime numbers.\n');
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
                // console.log('semiprime: ', 77);
                readline.close();
                // return 77;
                console.log('calling main')
                main(77);
            } else if (answer == 4) {
                // console.clear();
                refreshInterface();
                // pick one of the RSA numbers
                const rsaNums = [
                    { name: 'RSA-100', number: '1522605027922533360535618378132637429718068114961380688657908494580122963258952897654000350692006139' },
                    { name: 'RSA-110', number: '35794234179725868774991807832568455403003778024228226193532908190484670252364677411513516111204504060317568667' },
                    { name: 'RSA-120', number: '227010481295437363334259960947493668895875336466084780038173258247009162675779735389791151574049166747880487470296548479' },
                    { name: 'RSA-129', number: '114381625757888867669235779976146612010218296721242362562561842935706935245733897830597123563958705058989075147599290026879543541' },
                ]
                let prompt = `Pick one of the RSA numbers:\n`
                for (let i = 0; i < rsaNums.length; i++) {
                    prompt += `${i + 1}) ${rsaNums[i].name}\n`;
                }
                prompt += `Choose an option: `;

                readline.question(prompt, (answer) => {
                    if (answer > 0 && answer <= rsaNums.length) {
                        // console.log('semiprime: ', rsaNums[answer - 1].number);
                        readline.close();
                        // return rsaNums[answer - 1].number;
                        console.log('calling main')
                        main(rsaNums[answer - 1].number);
                    } else {
                        console.log('invalid input, start over in 3 seconds');
                        readline.close();
                        setTimeout(() => {
                            askUserForNumber();
                        }, 3000);
                        return;
                    }
                });
            } else {
                console.log('invalid input');
                readline.close();
                askUserForNumber();
            }
        });
}
// main();
askUserForNumber();

// n is the semi-prime
async function main(n) {
    // console.clear();
    interface.push(`calculating factors of:\n${n}`)
    refreshInterface();
    // const number = await askUserForNumber();
    const number = BigInt(n);

    const result = await badGuess(number, 2);

    let guess = result[0];
    let power = result[1];
    // console.log(`${guess} to the power of ${power} is 1 more than a multiple of ${number}`);

    const shareFactorsWithNumber = twoEquations(guess, power);
    // console.log('equations: ', shareFactorsWithNumber);

    const firstNum = shareFactorsWithNumber[0];

    const factor1 = euclid(firstNum, number);
    const factor1Str = factor1.toString();

    // now find the other factor by dividing the number by the first factor
    const factor2 = number / factor1;

    console.log(`\nThe factors of ${number} are ${factor1} and ${factor2}`);


    doAgain();
    function doAgain() {
        const readline = require('readline').createInterface({
            input: process.stdin,
            output: process.stdout
        });
        readline.question('\nWould you like to factor another number? (1=yes / 2=no): ', (answer) => {
            if (answer == 'y' || answer == '1' || answer == 'yes') {
                readline.close();
                askUserForNumber();
            } else if (answer == 'n' || answer == '2' || answer == 'no') {
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

async function badGuess(n, g) {
    const guess = g;
    let power = 1;

    // let guessToPower;
    let oldGuessToPower = BigInt(1);

    interface.push(`\nRaising ${guess} to higher and higher powers until it is 1 more than a multiple of the semi-prime.\
  \nAfterward, we will user euclid's algorithm to find the two prime factors of the semi-prime.`)

    interface.push(`\nThe number displayed below is only updated every 100 iterations. All powers are being checked.`)

    while (true) {
        if (power % 100 == 0) {
            refreshInterface('checking power: ' + power);

        }
        // console.log('power: ', power);
        // raise guess to the power
        // guessToPower = raiseToPower(guess, power);
        // multiply oldGuessToPower by guess to get new guessToPower
        oldGuessToPower *= BigInt(guess);
        // console.log('oldGuessToPower: ', oldGuessToPower);
        // console.log('guessToPower___: ', guessToPower);
        // await sleep(10000);
        // now find remainder of guessToPower divided by n
        let remainder = BigInt(oldGuessToPower) % n;
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
use std::slice::range;

fn main() {
    //multiples();
    //fibonacci_even();
    print!("{}", largest_prime_factor(600851475143));
}

// Exercicio 1
fn multiples() {
    let mut rs = 0;

    for i in 1..1000 {
        if i % 3 == 0 || i % 5 == 0 {
            rs = rs + i;
        }
    }

    return print!("Multiples of 3 or 5 bellow 1000 sum:{}", rs);
}

// Exercicio 2
fn fibonacci(mut nth: i32) -> i32 {
    /*let mut n1 = 1;
    let mut n2 = 2;
    let mut rs = 0;
    print!("{},{}",n1,n2);
    for _ in 0..(nth-2) {
        rs = n2 + n1;
        n1 = n2;
        n2 = rs;
        print!(",{}",rs);
    }*/
    return if nth == 0 {
        nth
    } else if nth == 1 {
        nth
    } else {
        fibonacci(nth - 2) + fibonacci(nth - 1)
    };
}

fn fibonacci_even() {
    let mut aux = 1;
    let mut even_sum = 0;
    loop {
        if fibonacci(aux) > 4000000 {
            break;
        }
        if fibonacci(aux) % 2 == 0 {
            println!("[{}]", fibonacci(aux));
            even_sum += fibonacci(aux);
        }
        aux += 1;
    }
    print!("Even-Valued Terms lesser than 4 million sum: {}", even_sum);
}

fn largest_prime_factor(mut n: i32) -> i32
{
    let mut max_prime = -1;

    while n % 2 == 0 {
        max_prime = 2;
        n /= 2;
    }

    return max_prime;
}
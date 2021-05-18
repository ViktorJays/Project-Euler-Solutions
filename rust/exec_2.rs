fn fibonacci(mut nth: i32) -> i32 {
    return if nth == 0 {
        nth
    } else if nth == 1 {
        nth
    } else {
        fibonacci(nth - 2) + fibonacci(nth - 1)
    };
}

pub(crate) fn fibonacci_even() {
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
fn main() {
    multiples();
}

// Exercicio 1
fn multiples() {
    let mut rs = 0;

    for i in 1..1000 {
        if i % 3 == 0 || i % 5 == 0 {
            rs = rs + i;
        }
    }

    return print!("{}", rs);
}


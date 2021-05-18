pub(crate) fn multiples() {
    let mut rs = 0;

    for i in 1..1000 {

        if i % 3 == 0 || i % 5 == 0 {
            rs = rs + i;
        }
    }

    return print!("Multiples of 3 or 5 bellow 1000 sum:{}", rs);
}
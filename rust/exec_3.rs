pub(crate) fn largest_prime_f(num: i64) {
    let mut _largest: i64 = 0;
    let mut _divisor: i64 = 2;
    let mut _aux: i64 = num;

    if num == 0 {
        println!("infinite");
    } else if num == 1 {
        _largest = num;
        println!("largest prime factor({}) = {}", num, _largest);
    } else {
        while _aux != 1 {
            if _aux % _divisor == 0 {
                _largest = _divisor;
                println!("{} / {}", _aux, _largest);
                _aux /= _divisor;
                _divisor = 2;
            } else {
                _divisor += 1;
            }
        }
        println!("largest prime factor({}) = {}", num, _largest);
    }
}
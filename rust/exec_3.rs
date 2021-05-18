pub(crate) fn prime_n() {
    let mut flag = 0;
    let mut loop_num = 1;
    let prime_n = 5;
    let loop_limit = 10;
    let mut p_n = 1;

    loop {
        if p_n > prime_n {
            break;
        } else {
            //println!("Q: {} is prime? x loop: {}", p_n, loop_num);
            if loop_num < loop_limit {
                if p_n % loop_num == 0 {
                    flag += 1;
                    loop_num += 1;
                } else {
                    loop_num += 1;
                }
            } else if flag <= 2 {
                println!("A: {} IS prime!", p_n);
                p_n += 1;
                loop_num = 1;
                flag = 0;
            } else {
                println!("A: {} is NOT prime!", p_n);
                p_n += 1;
                loop_num = 1;
                flag = 0;
            }
        }
    }
}
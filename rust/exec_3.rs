pub(crate) fn prime_n() {
    let mut flag = 0;
    let mut loop_num = 1;
    let prime_sequence = 1000;
    let loop_limit = prime_sequence;
    let mut p_n = 1;
    let mut counter = 1;

    loop {
        if p_n > prime_sequence {
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
                println!("{}: {} IS prime!", counter, p_n);
                p_n += 1;
                counter += 1;
                loop_num = 1;
                flag = 0;
            } else {
                //println!("A: {} is NOT prime!", p_n);
                p_n += 1;
                loop_num = 1;
                flag = 0;
            }
        }
    }
}
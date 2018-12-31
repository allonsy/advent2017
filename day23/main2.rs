/* Note: After reducing the input code, it is determined that the code
asks for the number of composites values between 106500 and 123500 inclusive */

fn main() {
    print_composite();
}

fn print_composite() {
    let mut num_composite = 0;
    let mut start = 106500;

    while start <= 123500 {
        if !is_prime(start) {
            num_composite += 1;
        }
        start += 17;
    }

    println!("num composites: {}", num_composite);
}

fn is_prime(n: i64) -> bool {
    if n == 2 {
        return true;
    }
    if n % 2 == 0 {
        return false;
    }

    let sqrt = (n as f64).sqrt() as i64;
    for val in 2..(sqrt + 1) {
        if n % val == 0 {
            return false;
        }
    }

    return true;
}

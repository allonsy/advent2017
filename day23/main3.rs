

fn main() {
    let mut b: i64 = 0;
    let mut c: i64 = 0;
    let mut d: i64 = 0;
    let mut e: i64 = 0;
    let mut f: i64 = 0;
    let mut g: i64 = 0;
    let mut h: i64 = 0;

    b = 106500;
    c = 123500;
    
    while {
        f = 1;
        d = 2;
        while {
            e = 2;
            while {
                g = d * e;

                if g == b {
                    f = 0;
                }
                e += 1;
                g = e;
                g -= b;
                g != 0
            } {};
            
            d += 1;
            g = d;
            g -= b;
            g != 0
        } {}
        
        if f == 0 {
            h += 1;
        }
        g = b;
        g -= c;
        
        b += 17;
        g != 0
    } {}

    println!("h is: {}", h);
}
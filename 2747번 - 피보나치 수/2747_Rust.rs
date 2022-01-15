fn main() {
    let mut n = String::new();
    std::io::stdin().read_line(&mut n);
    let mut n: u32 = n.trim().parse().expect("");
    
    let mut a = 0;
    let mut b = 1;

    while n != 0 {
        b = a + {a = b; b};
        n = n - 1;
    }
    print!("{}", a);
}

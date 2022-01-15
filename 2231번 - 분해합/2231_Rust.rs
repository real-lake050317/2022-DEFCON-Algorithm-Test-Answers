use std::{convert::{TryFrom}};
macro_rules! parse_line {
    (
        $n: expr,
        $t: ty
    ) => ({
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        let v = line.split_whitespace()
            .take($n).map(|x| x.parse::<$t>().unwrap())
            .collect::<Vec<$t>>();

        <[$t; $n]>::try_from(v).ok().unwrap()
    })
}

fn main() {
    let [n] = parse_line!(1, i32);
    let answer = divide_sum(n);
    println!("{}", answer);
}

fn divide_sum(n: i32) -> i32 {
    fn _proc(n: i32) -> i32 {
        let mut sum = 0;
        let mut k = n.clone();
        while k > 0 {
            sum += k % 10;
            k /= 10;
        }
        sum + n
    }
    let mut k = n.clone();
    let mut num_digit = 1;
    while k > 0 {
        k /= 10;
        num_digit += 1;
    }

    for i in (n - (9 * num_digit))..n {
        if _proc(i) == n {
            return i
        }
    }
    0
}

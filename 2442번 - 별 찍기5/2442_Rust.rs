use std::io;

fn main() {
    let mut str = String::new();

    io::stdin()
        .read_line(&mut str)
        .unwrap();
    
    let n: i32 = str
        .trim()
        .parse()
        .unwrap();
    
    let mut i = 0;
    while i < n {
        i += 1;
        let mut j = 0;
        let mut s = String::new();
        while j < n - i {
            j += 1;
            s += " ";
        }
        while j < n + i - 1 {
            j += 1;
            s += "*";
        }
        println!("{}", s);
    }
}

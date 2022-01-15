use std::io;
use std::collections::BTreeMap;

struct Solve {
    vis: BTreeMap<i32, bool>,
}

impl Solve {
    fn dfs(&mut self, n: i32) -> i32 {
        self.vis.insert(n, true);
        let mut sum = 0;
        let mut tmp = n;
        while tmp != 0 {
            sum += tmp % 10;
            tmp /= 10;
        }
        let next = (n % 10) * 10 + sum % 10;
        if self.vis.contains_key(&next) {
            return 1;
        } else {
            return self.dfs(next) + 1;
        }
    }

    fn main(&mut self) {
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        let n = line.trim().parse::<i32>().unwrap();
        println!("{}", self.dfs(n));
    }
}

fn main() {
    let mut solve = Solve {
        vis: BTreeMap::new(),
    };
    solve.main();
}

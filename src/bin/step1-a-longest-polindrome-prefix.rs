use std::io::{self};

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).expect("Unable to read");
    let n:u32 = line.trim().parse().unwrap();
    
    for _i in 0..n {
        let mut line = String::new();

        io::stdin().read_line(&mut line).expect("Unable to read");
        let m = line.chars().count();
        
        let mut mx = 1;

        for j in 2..m {
            let mut p = 0;
            let mut q = j-1;
            while p < q && line.chars().nth(p).unwrap() == line.chars().nth(q).unwrap() {
                p += 1;
                q -= 1;
            }

            if p >= q {
                mx = j;
            }
        }
        println!("{}", mx);

    }
}
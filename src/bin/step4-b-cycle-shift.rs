use std::io;
use std::cmp;
use std::io::prelude::*;

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).expect("Unable to read");
    let q:usize = line.trim().parse().unwrap();

    let lines:Vec<String> = io::stdin().lock().lines().map(|x| x.unwrap().trim().to_string()).collect();

    for i in 0..q {

        let s = lines[i*2].trim().chars().collect::<Vec<char>>();
        let t = lines[i*2+1].trim().chars().collect::<Vec<char>>();

        let line = [t.as_slice(), vec!['$'].as_slice(), s.as_slice(), s.as_slice()].concat();

        match z_func(&line).iter().position(|&x| x == t.len()) {
            Some(j) => println!("{}", j - 1 - t.len()),
            None => println!("{}", -1)
        }

    }
}

fn z_func(line: &Vec<char>) -> Vec<usize> {

    let mut l: usize = 0;
    let mut r: usize = 0;
    let n = line.len();
    let mut z = vec![0;n];

    for i in 1..n {
        if i < r {
            z[i] = cmp::min(r-i, z[i-l]);
        }
        while z[i] + i < n && line[z[i]] == line[z[i] + i] {
            z[i] += 1;
        }
        if z[i] + i > r {
            l = i;
            r = z[i] + i;
        }
    }
    return z;
}

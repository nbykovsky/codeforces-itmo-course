use std::io;
use std::io::prelude::*;
use std::cmp;

fn main() {
    let mut line = String::new();

    io::stdin().read_line(&mut line).expect("Unable to read");
    let q:usize = line.trim().parse().unwrap();

    let lines:Vec<String> = io::stdin().lock().lines().map(|x| x.unwrap().trim().to_string()).collect();

    for line in lines.into_iter() {
        let zs = z_func(&(line.chars().collect::<Vec<char>>()));
        let mut result = vec![1;zs.len()];
        let mut z_prev = 0;
        for (i,&z) in zs.iter().enumerate() {
            for j in z_prev..z {
                result[j] = zs.len() - i + 1;
            }

            z_prev = z;
        }
        println!("{}", result.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" "));
    }

}

fn z_func(line: &Vec<char>) -> Vec<usize>{
    let mut l = 0;
    let mut r = 0;
    let n = line.len();
    let mut z = vec![0;n];

    for i in 1..n {
        if i < r {
            z[i] = cmp::min(z[i-l], r-i);
        }
        while z[i] + i < n && line[z[i]] == line[z[i] + i] {
            z[i] += 1;
        }
        if z[i] + i > r {
            l = i;
            r = z[i] + i;
        }
    }
    z.sort();
    return z;
}
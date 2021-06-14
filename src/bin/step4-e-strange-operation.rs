use std::io;
use std::cmp;

fn main() {

    let mut s = String::new();
    let mut t = String::new();

    io::stdin().read_line(&mut s).expect("unable to read");
    io::stdin().read_line(&mut t).expect("unable to read");

    let s:Vec<char> = s.trim().chars().collect();
    let t:Vec<char> = t.trim().chars().collect();

    if s.len() != t.len() {
        println!("No");
        return;
    }
    let n = s.len();

    let mut m = 0;
    while m < n && s[m] == t[n-m-1] {
        m += 1;
    }

    let ts:Vec<char> = [t, vec!['$'], s].concat();

    for (i, x) in z_func(&ts).iter().enumerate() {
        if i + x == ts.len() && n - x <= m {
            println!("Yes");
            println!("{}", n - x);
            return;
        }
    }
    println!("No");
}

fn z_func(line: &Vec<char>) -> Vec<usize> {

    let mut l: usize = 0;
    let mut r: usize = 0;
    let n = line.len();
    let mut z: Vec<usize> = vec![0;n];

    for i in 1..n {
        if i < r {
            z[i] = cmp::min(r-i, z[i-l]);
        }
        while i + z[i] < n && line[i + z[i]] == line[z[i]] {
            z[i] += 1;
        }
        if i + z[i] > r {
            l = i;
            r = i + z[i];
        }
    }
    return z;
}
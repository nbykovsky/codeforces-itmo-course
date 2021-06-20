use std::io;
use std::cmp;

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).expect("unable to read");
    let line:Vec<char> = line.trim().chars().collect();
    let n = line.len();

    let mut z_vec:Vec<usize> = vec![0;n];

    for i in 0..n {
        for (j,&z) in z_func(&(line.as_slice()[i..])).iter().enumerate() {
            z_vec[i + j] = cmp::max(z_vec[i + j], z);
        }
    }

    let mut result:u128 = 0;

    for (i,z) in z_vec.iter().enumerate() {

        for l in (i + z + 1)..=n {
            result += (l - i) as u128;
        } 
    }

    println!("{}", result);
}

fn z_func(line: &[char]) -> Vec<usize> {
    let mut l:usize = 0;
    let mut r:usize = 0;
    let n:usize = line.len();
    let mut z:Vec<usize> = vec![0;n];
    for i in 1..n {
        if i < r {
            z[i] = cmp::min(r-i, z[i-l]);
        }
        while i + z[i] < n && line[i + z[i]] == line[z[i]] {
            z[i] += 1;
        }
        if i + z[i] > r {
            l = i;
            r  = i + z[i];
        }
    }
    return z;
}


/*
ABACABAB

*/
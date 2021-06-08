use std::io;
use std::cmp;

fn main(){
    let mut line = String::new();

    io::stdin().read_line(&mut line).expect("Unable to read");
    let line = line.trim().chars().collect::<Vec<char>>();
    let n = line.len();
    
    let mut z: Vec<usize> = vec![0;n];
    let mut l:usize = 0;
    let mut r:usize = 0;

    for i in 1..n {
        if i < r {
            z[i] = cmp::min(r-i, z[i-l]);
        }
        while z[i] + i < n && line[z[i]] == line[i+z[i]] {
            z[i] += 1;
        }
        if i + z[i] > r {
            l = i;
            r = i + z[i];
        }
    }

    println!("{}", z.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" "));
}
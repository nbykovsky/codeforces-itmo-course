use std::io;
use std::cmp;

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).expect("unable to read");
    let s:Vec<char> = line.trim().chars().collect();



    let ss = [
        s.as_slice(), 
        vec!['$'].as_slice(), 
        s.iter()
         .rev()
         .map(|x| *x)
         .collect::<Vec<char>>()
         .as_slice()
        ].concat();

    // println!("{:?}", ss);
    for (i, x) in z_func(&ss).iter().enumerate() {
        if x + i == ss.len() {
            println!("{}", x);
            break
        }
    }

}

fn z_func(line: &Vec<char>) -> Vec<usize>{
    let mut l:usize = 0;
    let mut r:usize = 0;
    let n = line.len();
    let mut z = vec![0; n];
    for i in 1..n {
        if i < r {
            z[i] = cmp::min(r - i, z[i - l]);
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
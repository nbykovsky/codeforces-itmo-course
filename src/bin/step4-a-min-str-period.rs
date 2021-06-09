use std::io;
use std::cmp;


fn main() {
    let mut line = String::new();

    io::stdin().read_line(&mut line).expect("Unable to read");
    let t:usize = line.trim().parse().unwrap();

    for _ in 0..t {
        let mut line = String::new();

        io::stdin().read_line(&mut line).expect("Unable to read");
        let line:Vec<char> = line.trim().chars().collect();

        let z = [z_func(&line), vec![0]].concat();
        for (i,n) in z.iter().enumerate() {

            if i + n == line.len() || i == line.len(){
                println!("{}", &line[..i].iter().map(|x| x.to_string()).collect::<Vec<String>>().join(""));
                break;
            }
        }
    }

}

fn z_func(line: &Vec<char>) -> Vec<usize>{

    let mut l:usize = 0;
    let mut r:usize = 0;
    let n = line.len();

    let mut z:Vec<usize> = vec![0;n];
    for i in 1..n {
        if i < r {
            z[i] = cmp::min(z[i-l], r - i);
        }
        while i + z[i] < n && line[z[i]] == line[i+z[i]] {
            z[i] += 1;
        }
        if i + z[i] > r {
            l = i;
            r = i + z[i];
        }
    }

    return z;
}


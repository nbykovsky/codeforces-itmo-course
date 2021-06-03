use std::io;


fn main(){
    let mut line = String::new();
    io::stdin().read_line(&mut line).expect("unable to read");
    let line = line.trim().chars().collect::<Vec<char>>();
    let n:usize = line.len();
    let mut z:Vec<usize> = vec![0;n];
    for i in 1..n {
        while i + z[i] < n && line[z[i]] == line[z[i]+i] {
            z[i] += 1;
        }
    }
    println!("{}", z.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(""))

}
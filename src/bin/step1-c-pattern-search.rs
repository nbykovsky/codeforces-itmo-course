use std::io;

fn main() {

    let mut line = String::new();
    io::stdin().read_line(&mut line).expect("Unable to read");
    let q:u32 = line.trim().parse().unwrap();

    for _i in 0..q {
        let mut t = String::new();
        let mut p = String::new();
        io::stdin().read_line(&mut t).expect("Unable to read");
        io::stdin().read_line(&mut p).expect("Unable to read");
        
        let t = t.trim();
        let p = p.trim();
        let n = t.chars().count();
        let m = p.chars().count();

        let mut res:Vec<usize> = Vec::new();

        if n >= m {
            for l in 0..(n-m+1) {
                let s = &t[l..(l+m)];
                let mut good:bool = true;
                for pair in p.chars().zip(s.chars()) {
                    if pair.0 != pair.1 && pair.0 != '?' {
                        good = false;
                        break
                    }
                }
                if good {
                    res.push(l);
                }
            }
        }
        println!("{}", res.len());
        println!("{}", res.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" "));
    }


}
use std::io;

fn main(){
    let mut lq = String::new();
    io::stdin().read_line(&mut lq).expect("unable to read");
    let q:u32 = lq.trim().parse().unwrap();

    for _ in 0..q {
        let mut s = String::new();
        io::stdin().read_line(&mut s).expect("unable to read");
        let s = s.trim().chars().collect::<Vec<char>>();
        let n = s.len();

        let mut t = String::new(); 
        io::stdin().read_line(&mut t).expect("unable to read");
        let t = t.trim().chars().collect::<Vec<char>>();
        let m = t.len();



        for l in 0..(n) {
            for r in (l+1)..(n+1) {
                let sub = &s[l..r];
                let mut good = true;
                for w in sub.windows(m) {
                    if w == t {
                        good = false;
                        break;
                    }
                }

                if good {
                    cnt += 1;
                }
            }
        }
        println!("{}", cnt);

    }

}
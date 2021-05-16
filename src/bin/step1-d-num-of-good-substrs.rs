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

        let mut cnt:u32 = 0;

        let mut idxs:Vec<usize> = Vec::new();

        for (i,w) in s.windows(m).enumerate() {
            if w == t {
                idxs.push(i);
            }
        }

        for l in 0..(n) {
            for r in (l+1)..(n+1) {
                let mut good = true;
                for &i in &idxs {
                    if (m <= r) && (l <= i) && (i <= r - m) {
                        good = false;
                        break;
                    } 
                }
                if good {
                    cnt += 1;
                } else {
                    break;
                }
            }
        }
        println!("{}", cnt);

    }

}
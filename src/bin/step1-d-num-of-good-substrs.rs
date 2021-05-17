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

        let mut cnt:usize = 0;

        let mut l:usize = 0;
        let mut r:usize = 0;

        while r <= n {
            if r - l >= m {
                let su = &s[r - m..r];

                if su == t {
                    l = r - m + 1;
                }
            }
            cnt += r - l;
            r += 1;
        }

        println!("{}", cnt);

    }

}
use std::io;

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).expect("Unable to read");
    let t:u32 = line.trim().parse().unwrap();
    
    for _i in 0..t {
        let mut line = String::new();
        io::stdin().read_line(&mut line).expect("Unable to read");
        let line = line.trim();

        let n = line.chars().count();
        let mut cnt:u32 = 0;
        for l in 0..(n) {
            for r in (l)..(n+1) {
                let m = r - l;
                
                let ln = &line[l..r];
                let pr = &line[0..m];
                let su = &line[(n-m)..n];

                if (ln == pr && ln != su) || (ln == su && ln != pr){
                    cnt += 1;
                }
            }
        }
        println!("{}", cnt);


    }

}
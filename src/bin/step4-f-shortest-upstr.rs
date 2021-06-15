use std::io;
use std::cmp;

/*

xxxxyyyyyyy
    yyyyyyyzzzzz

xxxxxyyyyyyyxxxxx
     yyyyyyy
        


*/

fn main() {

    let mut q = String::new();
    io::stdin().read_line(&mut q).expect("unable to read");
    let q:usize = q.trim().parse().unwrap();

    for _ in 0..q {
        let mut s = String::new();
        io::stdin().read_line(&mut s).expect("unable to read");
        let s:Vec<char> = s.trim().chars().collect();

        let mut t = String::new();
        io::stdin().read_line(&mut t).expect("unable to read");
        let t:Vec<char> = t.trim().chars().collect();

        println!("{}", find(&s,&t));
    }

}

fn find(s: &Vec<char>, t: &Vec<char>) -> String{
        let st:Vec<char> = [s.as_slice(), &['$'], t.as_slice()].concat();
        let ts:Vec<char> = [t.as_slice(), &['$'], s.as_slice()].concat();


        let mut cand: Vec<char> = [s.as_slice(), t.as_slice()].concat();

        for (i,&z) in z_func(&st).iter().enumerate() {
            if z == s.len() {
                cand = t.clone();
                break
            }

            if z + i == st.len() {
                cand = [t.as_slice(), &(s.as_slice())[z..]].concat();
                break;
            }
        }


        for (i, &z) in z_func(&ts).iter().enumerate() {
            if z == t.len() {
                cand = s.to_vec();
                break
            }
            if z + i == st.len() && s.len() + t.len() - z < cand.len(){
                cand = [s.as_slice(), &(t.as_slice())[z..]].concat();
                break;
            }
        }

        return cand.iter().map(|x| x.to_string()).collect::<Vec<String>>().join("");
}

fn z_func(line: &Vec<char>) -> Vec<usize> {
    let mut l: usize = 0;
    let mut r: usize = 0;
    let n = line.len();
    let mut z: Vec<usize> = vec![0;n];
    for i in 1..n {
        if i < r {
            z[i] =cmp::min(r-i, z[i-l]);
        }
        while i + z[i] < n && line[i+z[i]] == line[z[i]] {
            z[i] += 1;
        }
        if i + z[i] > r {
            l = i;
            r = i + z[i];
        }
    }
    return z;
}
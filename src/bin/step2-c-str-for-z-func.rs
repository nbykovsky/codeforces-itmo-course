use std::io;
use std::char;

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).expect("Unable to read");
    let t:u32 = line.trim().parse().unwrap();


    for _ in 0..t {

        let mut line = String::new();
        io::stdin().read_line(&mut line).expect("Unable to read");
        let n:usize = line.trim().parse().unwrap();
        
        let mut line = String::new();
        io::stdin().read_line(&mut line).expect("Unable to read");
        let nums:Vec<usize> = line.trim().split(" ").map(|x| x.trim().parse().unwrap()).collect();

        let mut table:Vec<char> = [(97..123).collect::<Vec<u32>>(),(65..91).collect::<Vec<u32>>()]
            .concat()
            .iter()
            .map(|&x| char::from_u32(x).unwrap())
            .collect::<Vec<char>>();


        let mut result:Vec<char> = Vec::new();

        let mut good: bool = true;
        
        'outer: for (i,&m) in nums.iter().enumerate() {
            
            if (i == 0 && m != 0) || (m + i > n) {
                good = false;
                //println!("i={} m={} n={}", i, m, n);
                break 'outer;
            } else if i == 0 {
                result.push(table.pop().unwrap());
            } else {

                let mut cand: Option<char> = None;

                for j in (0..=i).rev()  {
                    if nums[j as usize] > i - j{
                        match cand {
                            Some(c) if c != result[i-j] => {
                                //println!("c={} result[i-j]={}", c, result[i-j]);
                                good = false;
                                break 'outer;
                            }
                            _ => {
                                cand = Option::Some(result[i-j]);
                            }

                        }
                    }
                }
                
                match cand {
                    Some(c) => {
                        result.push(c);
                    }
                    None => {
                        result.push(table.pop().unwrap());
                    }
                }

            }
        }
        // println!("{:?}", result);
        if !good || z_func(&result) != nums{
            println!("!")
        } else {
            println!("{}", result.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(""));
        }
    }

    // println!("{:?}", table);
}

fn z_func(line: &Vec<char>) -> Vec<usize> {
    
    let n:usize = line.len();
    let mut z:Vec<usize> = vec![0;n];
    for i in 1..n {
        while i + z[i] < n && line[z[i]] == line[z[i]+i] {
            z[i] += 1;
        }
    }
    return z;
}

/*

A B A C A B A D A B A C A B A
a b a c a b a z a b a c a b a

*/
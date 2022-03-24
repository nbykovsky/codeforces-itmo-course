use std::io;


fn main() {
  let mut line = String::new();
  io::stdin().read_line(&mut line).expect("unable to read");
  line = line.trim().to_string();
  line.push('$');
  let _n = line.chars().count();
  let mut p:Vec<(char, usize)> = line.chars().zip(0..).collect();
  p.sort();
  let mut c:Vec<usize> = vec![0;_n];
  for _i in 1.._n {
    if p[_i].0 == p[_i - 1].0{
        c[p[_i].1] = c[p[_i-1].1];
    } else {
        c[p[_i].1] = c[p[_i-1].1] + 1;
    }
  }

  let mut result:Vec<usize> = Vec::new();
  let mut i:usize = 0;
  while (1 << i) < _n {
    let mut p:Vec<((usize, usize), usize)> = Vec::new(); 
    for j in 0.._n {
        p.push(((c[j],c[(j + (1<<i))%_n]),j));
    }
    
    p.sort();
    c[p[0].1] = 0;
    for _i in 1.._n {
      if p[_i].0 == p[_i - 1].0{
        c[p[_i].1] = c[p[_i-1].1];
      } else {
        c[p[_i].1] = c[p[_i-1].1] + 1;
      }
    }
    i += 1;
    if (1 << i) >= _n  {
      result = p.iter().map(|x| x.1).collect(); 
    }

    
  }
  println!("{}",result.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" "));
}

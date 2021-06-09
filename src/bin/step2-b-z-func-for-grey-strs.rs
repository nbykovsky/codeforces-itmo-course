use std::io;


fn main(){
   let mut line = String::new();
   io::stdin().read_line(&mut line).expect("unable to read");

   let t:u32 = line.trim().parse().unwrap();

   for _ in 0..t {
      let mut line = String::new();
      io::stdin().read_line(&mut line).expect("unable to read");
      let nums:Vec<u32> = line.trim().split(" ").map(|x| x.trim().parse().unwrap()).collect();
      let (k,j) = (nums[0], nums[1]);

      println!("{}", helper(k, j));

   }
}

fn helper(k:u32, j:u32) -> u32 {
   let n:u32 = u32::pow(2, k) - 1 - 2;
   if k == 1 || j < 2  {
      return 0;
   }else if j == (n/2 + 2) {
      return u32::pow(2, k-1) - 1; 
   } else if j == (n/2 + 1) || j == (n/2 + 3) {
      return 0;
   } else if j < n/2+1 {
      return helper(k-1, j);
   } else {
      // j > n/2 + 2
      return helper(k-1, j - 2 - n/2);
   }

}



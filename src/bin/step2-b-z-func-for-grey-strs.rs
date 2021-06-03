use std::io;

/*

a
0

aba
001

abacaba
0010301

abacabadabacaba
001030107010301

abacabadabacabaea bacabadabacaba
00103010701030101501030107010301

len_i = 2 ^ i - 1

if i < 2:
   return 0
elif

7
3 0
3 1
3 2
3 3
3 4
3 5
3 6

16
4 0
4 1
4 2
4 3
4 4
4 5
4 6
4 7
4 8
4 9
4 10
4 11
4 12
4 13
4 14
4 15


*/


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



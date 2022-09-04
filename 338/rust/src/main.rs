fn main() {
    println!("{:?}", solve(5));
}

fn solve(n: u32) -> Vec<i32> {
    let mut ans = vec![];
    for mut i in 0..n+1 {
        let mut count = 0;
       while i != 0 {
           if i % 2 == 1 {
               count += 1;
           }
           i = i /2;
       }
        ans.push(count);
    }
    ans
}
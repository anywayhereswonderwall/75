use std::cmp::min;

fn main() {
    println!("{:?}", solve([2].to_vec(), 3));
}

fn solve(coins: Vec<i32>, amount: i32) -> i32 {
    let mut dp = vec![amount + 1; (amount + 1) as usize];
    dp[0] = 0;
    for i in 1..(amount + 1) {
        for coin in &coins {
            if *coin <=i {
                dp[i as usize] = min(dp[i as usize], (1 + dp[(i - coin) as usize]));
            }
        }
    }
    if dp[amount as usize] == amount + 1 {
        return  -1;
    } 
    dp[amount as usize]
}
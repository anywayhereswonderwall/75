use std::cmp::max;

fn main() {
    let  prices = vec![7,1,5,3,6,4];
    println!("{:?}", solve(prices));
}

fn solve(arr: Vec<i32>) -> i32 {
    let len = arr.len();
    let mut max_profit = 0;
    let mut l = 0;
    let mut r = 1;
    while r < len {
        let cur_profit = arr[r] - arr[l];
        println!("{}", cur_profit);
        max_profit = max(max_profit, cur_profit);
        if arr[l] > arr[r] {
            l = r;
        }
        r += 1;
    }
    max_profit
}
    
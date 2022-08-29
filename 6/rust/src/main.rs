use std::cmp::max;

fn main() {
    let numbers = vec![-2,0,-1];
    println!("{:?}", solve(numbers));
}

fn solve(nums: Vec<i32>) -> i32 {
    let mut res = *nums.iter().max().unwrap();
    let mut cur_min = 1;
    let mut cur_max = 1;
    for n in nums {
        if n == 0 {
            cur_min = 1;
            cur_max = 1;
        } else {
            let temp_vec = vec![n, cur_max*n, cur_min*n];
            cur_min = *temp_vec.iter().min().unwrap();
            cur_max = *temp_vec.iter().max().unwrap();
            res = max(cur_max, res);
        }
    }
    res
}
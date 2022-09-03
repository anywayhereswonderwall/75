use std::cmp::{max, min};

fn main() {
    let input_nums = vec![1,8,6,2,5,4,8,3,7];
    println!("{:?}", solve(input_nums));
}

fn solve(height: Vec<i32>) -> i32 {
    let mut res = 0;
    let mut l = 0;
    let mut r = height.len() - 1;
    while l < r {
        let len = (r - l) as i32;
        let cur_volume = len * min(height[l], height[r]);
        res = max(cur_volume, res);
        if height[l] < height[r] {
            l += 1;
        } else if height[l] > height[r] {
            r -= 1;
        } else {
            if height[l + 1] > height[r - 1] {
                l += 1;
            } else {
                r -= 1;
            }
        }
    }
    res
}
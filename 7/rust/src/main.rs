use std::cmp::min;

fn main() {
    let numbers = vec![5,6,0,1,2,3];
    println!("{:?}", solve(numbers));
}

fn solve(nums: Vec<i32>) -> i32 {
    let mut res = nums[0];
    let mut l = 0;
    let mut r= nums.len() - 1;
    while l <= r {
        if nums[l] < nums[r] {
            res = min(res, nums[l]);
            break
        }

        let mid = (l + r) / 2;
        res = min(res, nums[mid]);
        if nums[mid] > nums[r] {
            l = mid + 1;
        } else {
            r = mid - 1;
        }
    }
    res
}
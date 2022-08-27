fn main() {
    let input_nums = vec![1, 2, 3, 5];
    println!("{:?}", solve(input_nums));
}

fn solve(nums: Vec<i32>) -> Vec<i32> {
    let len = nums.len();
    let mut prefix = 1;
    let mut postfix = 1;
    let mut answer = vec![1; len];
    for i in 0..len {
        answer[i] *= prefix;
        prefix *= nums[i];
    }
    for i in (0..len).rev() {
        answer[i] *= postfix;
        postfix *= nums[i];
    }
    answer
}
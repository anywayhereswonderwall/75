use std::collections::HashMap;

fn main() {
    let target = 6;
    let nums = vec![3, 2, 4];
    println!("{:?}", solve(target, nums));
}

fn solve(target: i32, nums: Vec<i32>) -> Vec<i32>  {
    let mut nums_map = HashMap::new();
    let mut answer: Vec<i32> = vec![];
    for (i, num) in nums.iter().enumerate() {
        let value = target - num;
        if nums_map.contains_key(&value) {
            answer.push(nums_map[&value]);
            answer.push(i.try_into().unwrap());
            break;
        } else {
            nums_map.insert(num, i.try_into().unwrap());
        }
    }
    answer
}
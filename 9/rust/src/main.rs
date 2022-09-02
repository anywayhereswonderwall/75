
fn main() {
    let input_numbers = vec![-1,0,1,2,-1,-4];
    println!("{:?}", solve(input_numbers));
}

fn solve(mut n:  Vec<i32>) -> Vec<(i32, i32, i32)> {
    let mut ans = Vec::new();
    n.sort();
    for (i, x) in n.iter().enumerate() {
        let mut l = i + 1;
        let mut r = n.len() - 1;
        while l < r {
            if n[l] + n[r] + x > 0 {
                r = r - 1;
            } else if n[l] + n[r] + x < 0 {
                l = l + 1
            } else {
                let tup = (n[l], n[r], *x);
                if !ans.contains(&tup) {
                    ans.push(tup);
                }
                l = l + 1;
            }
        }
    }
    ans
}
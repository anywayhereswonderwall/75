fn main() {
    let input_number = 0b00000000000000000000000000001011;
    println!("{input_number}");
    println!("{:?}", solve(input_number));
}

fn solve(n: u32) -> i32 {
    let mut num = n;
    let mut ans = 0;
    while num != 0 {
       if num % 2 == 1 {
           ans += 1;
       }
        num >>= 1;
    }
    ans
}
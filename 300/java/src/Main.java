

public class Main {
    public static void main(String[] args) {
        int arr[] = {7,7,7,7,7,7};
        System.out.println(solve(arr));
    }
    public static int solve(int[] nums) {
        int len = nums.length;
        int[] dp;
        dp = new int[len];
        int ans = 1;
        for (int i = len - 1; i >= 0; i--) {
            dp[i] = 1;
            for (int j = i; j < len; j++) {
                if (nums[i] < nums[j]) {
                        dp[i] = Math.max(dp[i], dp[j] + 1);
                }
            }
            ans = Math.max(ans, dp[i]);
        }
        return ans;
    }
}
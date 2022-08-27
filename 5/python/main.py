import sys

numbers = [1]


def solve(nums):
    max_sum = -sys.maxsize
    cur_sum = 0
    for num in nums:
        if cur_sum < 0:
            cur_sum = 0
        cur_sum += num
        max_sum = max(max_sum, cur_sum)
    return max_sum


print(solve(numbers))
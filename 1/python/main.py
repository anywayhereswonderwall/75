numbers = [3, 2, 4]
target = 6


def solve(target_sum, nums):
    nums_map = {}
    for i, num in enumerate(nums):
        remainder = target_sum - num
        if remainder in nums_map.keys():
            return nums_map[remainder], i
        else:
            nums_map[num] = i


print(solve(target, numbers))
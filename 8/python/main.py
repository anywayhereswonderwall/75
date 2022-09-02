nums = [5,1,3]
target = 3


def solve(n, t):
    l, r = 0, len(n) - 1
    while l <= r:
        m = (l + r) // 2
        if n[m] == t:
            return m
        if n[l] <= n[m]:
            if n[m] > t >= n[l]:
                r = m - 1
            else:
                l = m + 1
        else:
            if n[r] >= t > n[m]:
                l = m + 1
            else:
                r = m - 1
    return -1

print(solve(nums, target))
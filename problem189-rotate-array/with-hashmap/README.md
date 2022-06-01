Introduction
===

This works but doesn't pass Leetcode some tests, for example when `k` is bigger than the lenght of the array, this code left the `nums` array untouched but leetcode want it rotated.

__Leetcode Test Case__:

```
Input: [1, 2]
k = 3
Expect = [2, 1]
```

Because Leetcode assumes that the rotation is made by 1 step each time, so it can manage to rotate the array.

But I use a hashmap and do all rotation after that with hashmap elements:
```rust
let mut map: HashMap<usize, i32> = HashMap::with_capacity(nums.len());
```

__PS:__ _I choose to submit this solution to this repository since I'm not sure Leetcode's assumption is right._

Explanation
===

Works and passes all the tests!

Submission [details](https://leetcode.com/submissions/detail/712083567/?from=explore&item_id=646).


This way there is no hashmap to store array information, in `with-hashmap` code I assumed that the array is an actual -no growth- array and I left untouched the array, instead used a hash map to store the new array information. 

In this solution, the trick is using `modulo operation` to calculate `k` otherwise the array -vector- would need too much space.
## Approach
I wanted to make use of fold to explore this syntax and challenges related to borrowing / mutability. The solution would have been much simpler had I loops and offset indexes or zip. 

Part 2 Extension: I'd like to extend my implementatino to support windows of variable length vs. 3 in current implementation. 

Regarding benchmarking, I have split the benchmarks into those which include file reading and parsing and those which focus on the algorithm performance. The `...-full` benchmarks are inclusive of file reading and parsing time.

After looking at other solutions, this problem could be solved by zipping offset lists of depths together and doing tuple-wise comparisons/ 

### Note on comparing adjacent windowed sums: 
Given that we are comparing every depth value, neighboring sums only differ in the first entry of the trailing window and the new depth for the entry being compared. Given this, we can avoid calculating a sum and simply compare these two entries. 

For Example
```  
1 2 3 4 5  
-----------
A B C      
  B C D
    C D E

Col3Sum = A + B + C
Col4Sum =     B + C + D

Checking if Col4Sum > Col3Sum can be done by simply comparing value at Col4 with Col1

Based on the problem statement, we can start at col3 since we will not consider partial sums.

Note: This can be reduced to a simple loop with offsets

```

Extended: If we are simply comparing 2 values at fixed offsets, we could also use *zip* with an appropriate offset starting index and then filter on 2nd entry > 1st entry (in zip tuple) and count the items left after filtering.

For Example
```
For Part 1 (single offset) 

[A B C D E]  zipped with -> 
[B C D E] 
= 
[(A, B) (B, C) (C, D) (D, E)] filter (second > first) and return length

For Part 2 (offset = 3)
[A B C D E F] zipped with ->
[D E F]
= 
[(A, D), (B, E), (C, F)]

Similar to above (loop solution), we are simply comparing the current value with the one from 3 indices prior


```
 
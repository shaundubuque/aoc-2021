## Approach
I wanted to make use of fold to explore this syntax and challenges related to borrowing / mutability. 

Part 2 Extension: I'd like to extend my implementatino to support windows of variable length vs. 3 in current implementation. 

Regarding benchmarking, I have split the benchmarks into those which include file reading and parsing and those which focus on the algorithm performance. The `...-full` benchmarks are inclusive of file reading and parsing time.

After looking at other solutions, this problem could be solved by zipping offset lists of depths together and doing tuple-wise comparisons/ 
## Initial Thoughts

First glance, seems like I could solve this with ~cellular automata where the logic of each clock tick is managed by an instance of a lanternship and some signaling back to main loop for events that occur (and need to be managed at environment level)

### Input Parsing
Looks to be pretty straightforward with single list to be split on ','

### Part 2
For part 2, we start to run into inefficiencies in implementation where running for 256 takes exponentially more time. 

#### Compute Optimizations

Instead of keeping track of all fish, we could keep track of groups of counts and then just add groups on each iteration. 

For example, at start (for sample) we have: 

1: 1, 2: 1, 3: 2, 4: 1
0: 1, 1: 1, 2: 2, 3: 1
0: 1, 1: 2, 2: 1, 6: 1, 8:1
0:2, 1:1, 5:1, 6:1, 7:1, 8:1



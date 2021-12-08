## Initial Thoughts

Part 1 seems to be a simple exercise in counting the number of output elements whose length is in set of patterns which are unique. 

If we have uniqueLengthSet = [2, 3, 4, 7] // corresponds to ["1", "7", "4", "8"]

Solution = [flatmap output].filter(|val| val.len() in uniqueLengthSet).len()

### Input parsing
We will want to split input into 2 sections, samples, output

Output of algo is count of times that specific digits appear, in part1 this is [1,4,7,8]


## Initial thoughts

On first read, I def thought we'd be parsing the diagnostic report line-by-line. 

Nope, this will about 2-dimensional array access (or 1-d with some cleverness). 

### Simple Approach (pseudocode)

Read each line

Init counters for each index (5 - for sample, but n for input text). 

*NOTE:* Input text differs from sample in bit vector length, i.e. 12 vs 5. It also differs in the numbner of input vectors. 


```
Init counters -- one for each index, length n where n is col - length

Loop through each line
    Loop through each index in line
        if val at index is 0, decrement counters[i]
        else increment counters[i]


loop through each index
    gamma: 
        if counters[i] < 0 gamma[i] then 0 else 1
    epsilon
        if counters[i] < 0 gamma[i] then 1 else 0
```
### Fun Approach

```
Read in each line, append into single vector
Map each value in vector to 1 if 1, -1 if 0
Calc chunked sums (sum across col)

for gamma: map sum -> sum < 0 then 0 else 1
for epsilon: map gamma -> 0 then 1 else 0
```

*NOTE:* Instead of mapping 0 -> -1, we could just use fold and apply logic in fold to do mapping. This is not as 'clean' as a sum but additional clarity is probably nice.  

As an alternative to flattening and chunking, we could also use the [ndarray](https://docs.rs/ndarray/latest/ndarray/) crate which always for more creative ways of slicing multi-dimensional arrays, similar to [Python's NumPy ndarray's](https://numpy.org/doc/stable/reference/generated/numpy.ndarray.html)

In this case:
```
Read in input vectors
Map values 1 -> 1, 0 -> -1
Create slices for each column
Sum [:,i] -- sum across each column
gamma = map sum -> sum < 0 then 0 else 1
epsilon = map gamma -> gamma < 0 then 1 else 0
```

*NOTE:* We don't need to invert epsilon, it's value will be 2^(bit_vec_length - 1) - gamma

## Part 2

We can borrow most common / least common functionality from part 1. 

At first glance, this feels like a sequentional filtering based on prefix of most/least common bit pattern. 

Ideally, this would be a simple filtering exercise based on full bit pattern but of course, this has to be more complicated :). 

*NOTE:* D'Oh, my initial approach used previous work to identify the most/least common bits but in fact, part-2 requires recomputing this for each filtered collection step. 

### Simple approach

```
Loop n-bit times (n = length of max


collection = filtered collection
if collection length == 1 return this string as matching value
```
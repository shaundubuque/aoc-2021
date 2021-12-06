## Initial Thoughts

Going to go more straightforward on this one. 

### Input Parsing

Split on '->', then parse pairs as tuples

*NOTE* x is a horizontal coordinate, not to be confused with row as in [row, col] addressing. 

#### Part1 

##### Filter out diagonals
We need to filter out any coordinate pairs where x's or y's do not match (only looking at horizontal and vertical line pairs)

##### Problem Input

The sample input references a 9x9 grid but the puzzle input has indices which exceed these dimensions. Looks like we will need to find the max value for both X and Y to initialize our grid. 


##### Accumulate overlapping coordinates
Loop through each line (either horizontally or vertically, incrementing the map coords at each location)

##### Count coordinates where count > 2

We can either flatten the map array and filter/count or accumulate across rows


## Initial Thoughts

* Would average of values work? Nope, solves for a different constraint.
* Next thought, could I just use a linear regression method to fit a line as this minimizes distance from a line? 
* Well, solution is pretty simple, just mechanical solve for optimal solution
* Since we want to find optimial solution which is discrete, probably means we need to just mechanically work the problem. 
* It is likely that part2 will make simple solution's runtime really long, probably going to call for memoization or grouping but we don't need to solve for this now. 

## Part 1

### Input Parsing
Simple, just split on ','

### Fuel Minimization

options = [min(input), max(input)]
best_option = MAX
```
for each option
   calc sum of distance (new_option) of each crab sub from option
   if new_option < best_option best_option = new_option
   
return best_option
```


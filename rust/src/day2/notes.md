## Initial thoughts

This feels like a great fit for a match case against parsed commands where each matched arm updates state according to the command provided. 

We will need to two state variables: 
1. Horizontal Position 
2. Depth

### Simple Approach (pseudocode)
Parse inputs into commands (command name + value)
For each command, mutate state according to command.

For parsing commands, I can use the util method I created from day 1 to parse a string into a struct in this case. I'll just need to implement the FromStr and Err:Debug traits.

### Fun Approach
Instead of commands which are matched and used to update state, we could translate commands into State Functions which take in state and mutate it. Then, our solution would just be a composition of functions. 

In this case, the command name is the function and the value is a curried parameter. Then, the function could simply be applied to state and sequenced. 



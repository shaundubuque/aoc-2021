## Initial Thoughts

Couple of interesting problems here: parsing the board and selecting an efficient data structure / pattern for tracking bingo selections

### Input Parsing

2 Modes
- Bingo Selections
- Board Parsing

#### Bingo Selections
Easy, just take first line and split on ','

#### Board Parsing
Observations:
- Fixed width entries vs. nicely delimited (e.g. ',')
- Splitting on comma would require some additional cleanup (filter out empty strings after splitting)
- Alternative would be to chunk the strings into 2 char blocks and parse these individually
- Need to logically separate different boards using blank line
  - New / blank line pushes current board and starts new board
    - lines in this state get pushed to separate board
- With separate boards (as Vec<String>)
  - Step through each line (by chunks) and initialize logical board (which can be used to track bingo status)

### State Tracking

First thought to keep things efficient, though maybe not simple, is to use a hashmap (int index) to point to a row and col index that can be used to increment corresponding entries. 

*Assumptions (to be validated)*
- No duplicated numbers on boards, e.g. X only appears in a single location on a board
- No duplicated selections, e.g. if X has been picked, it will not ever be picked again
- Boards are square

#### Goals for efficient approach (using above assumptions)

- Marking board selections is O(1)
- Checking board completion is O(1) (done when marking selection)
- Summing non-selected entries of winning board is O(n) where n < m*m

*STUMBLES* 
I ran into a couple of stumbling blocks when solving for part2 based on some simplifying assumptions made in solving part1, namely, that only a single board would win for each selection. 




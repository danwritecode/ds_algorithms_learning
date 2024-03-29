## Logarithms

Logarithms can be thought of as the inverse of exponents. 

Examples: 

log10 100 = 2
log2 8 = 3
log2 16 = 4
log5 25 = 2

Think about it this way: what number needs to be the exponent of the "base" in order to equal the argument 


## Binary Search

Covered in the code example. 


## Running Time and BigO Notation

#### Running Time

For a simple search of 100 items, it can take 100 guess, this is: *linear time*
For a binary search of 100 items, worst case is 7 cases: this is: *log time* (log2 128 = 7, log2 64 = 6...worst case is 7)

For a simple search of *1,000,000,000* items, it can take 1,000,000,000 guesses, this is: *linear time*
For a binary search of *1,000,000,000* items, worst case is 30 cases: this is: *log time* (log2 1,000,000,000 = 29.9)


#### BigO

Big O tells you the number of operations an algorithm will make in the WORST CASE

Notation is typically this: O(n)

*Examples:*
  - Linary search = O(n)
  - Binary Search log n = O(log n)
  - Quicksort = O(n * log n)
  - Selection sort = O(n^2)
  - Traveling Salesperson = O(n!)


*Grid Drawing Example:*
  - Draw 16 boxes on a page = O(n)
  - Fold the page 4 times = O(log n)



#### The Traveling Salesperson

O(n!)

No known way to make this algorithm faster.

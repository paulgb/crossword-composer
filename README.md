Crossword Composer
==================

Crossword Composer is a browser-based tool for making crosswords. It consists of two main components:

- A generic word puzzle auto-filler which, finds a set unique of words that conforms to given shared-letter constraints. The auto-filler is written in Rust, and could be used as a standalone library. The auto-filler itself is not aware of the structure of the crossword. Its input is a list of constraints that the output must satisfy (for example, the "third letter of the third word must be the same as the second letter of the fifth word"). As a result, it could easily be used for most crossword variants with no modification, besides generating suitable input.
- A browser-based UI which displays a blank crossword and allows the user to put blocks in place to shape the grid. The UI loads a version of the auto-filler compiled to WebAssembly to (try to) fill in the puzzle as the user types.

Auto-filler
-----------

The auto-filler takes two inputs: a dictionary of allowable words, and a set of constraints that a solution must satisfy. It returns a flat list of characters which specify a complete solution. The mapping from characters in words to indices in the final output is a many-to-one mapping, since multiple words (two in the case of a crossword) can share the same character. This mapping _is itself_ the only representation of the puzzle that the filler sees; it is sufficient to describe the constraints that need to be satisfied.

The filler uses a standard backtracking approach, but two enhancements are made to make it decently fast:

- The backtracking algorithm works by taking (and untaking) atomic steps of one word at a time. The order of words visited in these steps is always the same within an individual solution attempt. The order is chosen as follows:
  1. For each word, we start a count of how many "previously taken" words overlap with that word. Initially, these are set to zero, because we haven't taken any words yet.
  2. Until no words remain, we take the word with the *most* overlaps with other words, breaking ties by taking the *longest* word. These measures are both heuristics for picking the words where we will have the fewest choices at that point in the search, leading to dead-ends quickly when we've started down an impossible path.
- For each step in the backtracking search, an in-memory index is created. The index provides a fast mapping between possible sets of the letters that would be *known* at a given stage to all of the sets of letters that could be filled in for the remaining letters to create a valid word.

Browser UI
----------


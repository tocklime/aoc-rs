# Advent of code, all solutions, in Rust.

## 2022 further ideas notes

* day 8 - I think this can be done quicker than 'consider each location' - p1 could try to flood from each of the 4 directions, and mark visible trees on a separate map.
* day 8 p2 - Again, I think there's a quicker way. For each row, keep a map of height to most-recent index of that height. Then to see how many trees you can see to the left, find all entries with height at least mine, and take the largest index. If you see the largest index, you can clear the map. Repeat for all 4 directions, collect products into a separate map.
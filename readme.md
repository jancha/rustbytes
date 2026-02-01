# Rust Bytes challenge solutions

### Feb 1, 2026 - Issue 104

This time - tip about use of then() - shorthand for code that checks condition before returning Some(T) or None alternatively.

### Jan 25, 2026 - Issue 103

This time - tip about use of `#[non_exhaustive]` attribute. This prohibits external crates to setup struct directly, and requires use of new() - useful, where internally struct can change over time - forcing use of new() ensures that caller does not break on struct change.

### Jan 18, 2026 - Issue 102

This time - tip about use of `#inline(always)` attribute. Tells compiler to perform the function in place for better performance. Code wise expect slightly larger binary.


### Jan 10, 2026 - Issue 101

This time we are looking into how to compare cyclically shifted strings. Basic idea is to take two strings, convert into `VecDeque<u8>` and rotate the other for `len` times and check if values are matching. If values match, push to the matches and perform outer continue.


### Jan 4, 2026 - Issue 100

Classical problem of vector merging. Key idea - sort first, then loop through and simply expand until there is no overlap, and push the expanded vector in the resultset.

### Dec 29, 2025 - Issue 99

In this week's task, we are looking at basic functionality of [`Arc`][1] and [`Mutex`][2]. [`Arc`][1] stands for 'Atomically reference counted', which provides a way to safely access shared memory among threads, while [`Mutex`][2] provides a thread-safe interior mutability. The combination of both ensures safe multi-thread access to shared memory (in single thread case consider using  [`Rc`][3] and [`RefCell`][4] alternatives).

When dealing with max, using modulus to get the reminder to reset the value to. If the value can get negative, you can use [`rem_euclid()`][5] function, to get the non-negative remainder.

[1]: https://doc.rust-lang.org/std/sync/struct.Arc.html
[2]: https://doc.rust-lang.org/std/sync/struct.Mutex.html
[3]: https://doc.rust-lang.org/std/rc/struct.Rc.html
[4]: https://doc.rust-lang.org/std/cell/struct.RefCell.html
[5]: https://doc.rust-lang.org/std/?search=rem_euclid

### Dec 22, 2025 - Issue 98

Given that a HashMap does not provide a guaranteed order, we are first looping through the HashMap of the graph and building a HashMap of the nodes and paths that take to them.
Then we are converting the HashMap into a sorted Vec, to get the top nodes (the ones with the fewest paths leading to them).
After this we are iterating through the nodes according to the path count. For every top node we are then performing a path walking to determine any cycles that stem from that node. If we find any, we return the set of the nodes.
Otherwise, we push the nodes in the order Vec, that contains the shortest path of the rumor to the person.

Initially I had solved it to return all involved nodes in any cycles, but the task is simpler - only the first cycle is required.

So, the combination of BFS and DFS is the solution for the task.


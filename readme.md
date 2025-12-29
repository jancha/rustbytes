# Rust Bytes challenge solutions

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


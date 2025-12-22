# Rust Bytes challenge solutions


### Dec 22, 2025 - Issue 98

Given that a HashMap does not provide a guaranteed order, we are first looping through the HashMap of the graph and building a HashMap of the nodes and paths that take to them.
Then we are converting the HashMap into a sorted Vec, to get the top nodes (the ones with the fewest paths leading to them).
After this we are iterating through the nodes according to the path count. For every top node we are then performing a path walking to determine any cycles that stem from that node. If we find any, we return the set of the nodes.
Otherwise, we push the nodes in the order Vec, that contains the shortest path of the rumor to the person.

Initially I had solved it to return all involved nodes in any cycles, but the task is simpler - only the first cycle is required.

So, the combination of BFS and DFS is the solution for the task.

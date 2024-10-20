# Binary Search Tree vs Balanced Binary Search Tree
This rust project showcases runtime comparison between a Binary search tree and Balanced binary search tree.

For both cases we built a tree with a **predefined** number of nodes and then we search for a random number in the
tree. We measure the time it takes to search for the number in the tree.

A successful will run will show results like this:
``` 
---------------------------------
Building binary tree, with nodes 100000
Elapsed time: 30.261062ms
---------------------------------
Searching for: [62213227]
Elapsed time: 551ns
Found: true
No of read: 23
---------------------------------
Searching for: [8809]
Elapsed time: 430ns
Found: false
No of read: 21
BST Tree length: 41
---------------------------------
Building AVL tree, with nodes 100000
Elapsed time: 231.744956412s
---------------------------------
Searching for: [62213227]
Elapsed time: 651ns
Found: true
No of read: 17
---------------------------------
Searching for: [68792]
Elapsed time: 461ns
Found: false
No of read: 17
AVL BST Tree length: 20
```

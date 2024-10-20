use dotenv::dotenv;
use rand::Rng;
use std::cmp::{max, Ordering};
use std::env;

#[derive(Debug)]
struct Node<T> {
    value: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

fn insert_node<T: PartialOrd>(value: T, root: &mut Node<T>) {
    if value <= root.value {
        match &mut root.left {
            Some(left) => insert_node(value, left),
            None => {
                root.left = Some(Box::new(Node {
                    value,
                    left: None,
                    right: None,
                }));
            }
        }
    } else {
        match &mut root.right {
            Some(right) => insert_node(value, right),
            None => {
                root.right = Some(Box::new(Node {
                    value,
                    left: None,
                    right: None,
                }));
            }
        }
    }
}

fn search<T: PartialOrd + Ord>(value: T, root: &Node<T>, no_of_read: &mut i32) -> bool {
    *no_of_read += 1;
    match value.cmp(&root.value) {
        Ordering::Equal => true,
        Ordering::Less => match &root.left {
            Some(left) => search(value, left, no_of_read),
            None => false,
        },
        Ordering::Greater => match &root.right {
            Some(right) => search(value, right, no_of_read),
            None => false,
        }
    }
}

fn build_tree(dataset: &Vec<i32>) -> Node<i32> {
    println!("---------------------------------");
    let mut root = Node {
        value: dataset[0],
        left: None,
        right: None,
    };
    println!("Building binary tree, with nodes {:?}", dataset.len());
    let time_start = std::time::Instant::now();

    for &i in dataset {
        insert_node(i, &mut root);
    }
    let elapsed = time_start.elapsed();
    println!("Elapsed time: {:?}", elapsed);
    root
}

fn search_value(node: &Node<i32>, value: i32) {
    println!("---------------------------------");
    println!("Searching for: [{:?}]", value);
    let time_start = std::time::Instant::now();
    let mut no_of_read = 0;
    let found = search(value, node, &mut no_of_read);
    let elapsed = time_start.elapsed();
    println!("Elapsed time: {:?}", elapsed);
    println!("Found: {:?}", found);
    println!("No of read: {:?}", no_of_read);
}

fn build_avl_tree(dataset: &Vec<i32>) -> Node<i32> {
    println!("---------------------------------");
    let mut root = Some(Box::new(Node {
        value: dataset[0],
        left: None,
        right: None,
    }));
    println!("Building AVL tree, with nodes {:?}", dataset.len());
    let time_start = std::time::Instant::now();

    for &i in dataset {
        insert_avl_node(i, &mut root);
    }
    let elapsed = time_start.elapsed();
    println!("Elapsed time: {:?}", elapsed);
    *root.unwrap()
}

fn insert_avl_node<T: PartialOrd + Copy>(value: T, root: &mut Option<Box<Node<T>>>) {
    if let Some(node) = root {
        if value <= node.value {
            insert_avl_node(value, &mut node.left);
        } else {
            insert_avl_node(value, &mut node.right);
        }

        let balance = balance_factor(node);

        if balance > 1 {
            if let Some(left) = &node.left {
                if value < left.value {
                    right_rotate(root);
                } else {
                    left_rotate(&mut node.left);
                    right_rotate(root);
                }
            }
        } else if balance < -1 {
            if let Some(right) = &node.right {
                if value > right.value {
                    left_rotate(root);
                } else {
                    right_rotate(&mut node.right);
                    left_rotate(root);
                }
            }
        }
    } else {
        *root = Some(Box::new(Node {
            value,
            left: None,
            right: None,
        }));
    }
}

fn left_rotate<T: PartialOrd>(node: &mut Option<Box<Node<T>>>) {
    if let Some(mut x) = node.take() {
        if let Some(mut y) = x.right.take() {
            x.right = y.left;
            y.left = Some(x);
            *node = Some(y);
        } else {
            *node = Some(x);
        }
    }
}

fn right_rotate<T: PartialOrd>(node: &mut Option<Box<Node<T>>>) {
    if let Some(mut y) = node.take() {
        if let Some(mut x) = y.left.take() {
            y.left = x.right;
            x.right = Some(y);
            *node = Some(x);
        } else {
            *node = Some(y);
        }
    }
}

fn balance_factor<T>(node: &mut Box<Node<T>>) -> i32 {
    height(&node.left) - height(&node.right)
}

fn height<T>(node: &Option<Box<Node<T>>>) -> i32 {
    if let Some(node) = node {
        max(height(&node.left), height(&node.right)) + 1
    } else {
        0
    }
}

fn main() {
    dotenv().ok();
    let n: i32 = env::var("N").expect("N not set").parse().expect("N is not a valid integer");
    let max_value: i32 = env::var("MAX_VALUE").expect("MAX_VALUE not set").parse().expect("MAX_VALUE is not a valid integer");
    let mut data_set = Vec::new();
    let mut rng = rand::thread_rng();
    for _ in 0..n {
        let random_value = rng.gen_range(1..max_value);
        data_set.push(random_value);
    }

    let root = build_tree(&data_set);
    // search for a value that is in the tree
    let value = data_set[rng.gen_range(0..n) as usize];
    search_value(&root, value);

    let mut rng = rand::thread_rng();
    let random_value = rng.gen_range(1..n);
    // search for a value that is not in the tree
    search_value(&root, random_value);
    println!("BST Tree length: {:?}", height(&Some(Box::new(root))));

    let root = build_avl_tree(&data_set);
    search_value(&root, value);
    // search for a value that is not in the tree
    search_value(&root, random_value);
    println!("AVL BST Tree length: {:?}", height(&Some(Box::new(root))));
}

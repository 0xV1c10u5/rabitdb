# Arc(itecture)
Onboarding/Design Explanation

## What is this document?
It seems very quickly it might make sense to have a journal of design decisions and quick references to explanations of relevant concepts, in order to help anyone who might want to contribute, as well as myself in the future if there is a lapse in development time for whatever reason.

### Core Concepts

What is a "B+ Tree"? Why is it important to what we are trying to build?
<p>
🌳 What is a B+ Tree?

A B+ tree is a balanced multiway search tree optimized for block-oriented storage (disk pages, SSD pages, memory cache lines). It’s the standard index structure in databases.

Key traits:

Internal nodes: only store keys and child pointers (no values).

Leaf nodes: store all the (key, value) pairs.

Linked leaves: leaves are connected in a linked list → fast range scans.

Balanced: all leaves are at the same depth.

High fanout: each node holds many keys (hundreds or thousands if node size ≈ 4 KB).

🔑 Why Databases Use B+ Trees

Disk/SSD efficiency: One node ≈ one page. Accessing a key = just a few page reads.

Shallow depth: With fanout ≈ hundreds, even a billion rows fit in 3–4 levels.

Range queries: Linked leaves make BETWEEN and prefix lookups very fast.

Good concurrency: Locks at node/page level are manageable.

Updates are localized: Only a few nodes split/merge per insert/delete.

That’s why almost every RDBMS (Postgres, MySQL, SQLite, etc.) uses B+ trees as their main index structure.

🦀 Implementing a Simple B+ Tree in Rust

Here’s a toy in-memory implementation with:

Internal nodes: guide search

Leaves: store key/value pairs + next pointer

Basic insert + search

This is educational (not production-grade). Real DB implementations manage pages, caching, and concurrency.

use std::cmp::Ordering;
use std::rc::Rc;
use std::cell::RefCell;

const ORDER: usize = 4; // max children per internal node

// Node type: Internal or Leaf
enum Node<K, V> {
    Internal {
        keys: Vec<K>,
        children: Vec<Rc<RefCell<Node<K, V>>>>,
    },
    Leaf {
        keys: Vec<K>,
        values: Vec<V>,
        next: Option<Rc<RefCell<Node<K, V>>>>,
    },
}

pub struct BPlusTree<K, V> {
    root: Rc<RefCell<Node<K, V>>>,
}

impl<K: Ord + Clone, V: Clone> BPlusTree<K, V> {
    pub fn new() -> Self {
        Self {
            root: Rc::new(RefCell::new(Node::Leaf {
                keys: Vec::new(),
                values: Vec::new(),
                next: None,
            })),
        }
    }

    pub fn search(&self, key: &K) -> Option<V> {
        let mut node = self.root.clone();
        loop {
            match &*node.borrow() {
                Node::Internal { keys, children } => {
                    // binary search
                    let mut idx = keys.binary_search(key).unwrap_or_else(|i| i);
                    if idx >= children.len() { idx = children.len() - 1; }
                    node = children[idx].clone();
                }
                Node::Leaf { keys, values, .. } => {
                    return keys.iter()
                        .position(|k| k == key)
                        .map(|i| values[i].clone());
                }
            }
        }
    }

    pub fn insert(&mut self, key: K, value: V) {
        let root = self.root.clone();
        let (new_key, new_node) = Self::insert_rec(root.clone(), key, value);

        if let Some((k, child)) = new_key.zip(new_node) {
            // root split
            let new_root = Node::Internal {
                keys: vec![k],
                children: vec![root, child],
            };
            self.root = Rc::new(RefCell::new(new_root));
        }
    }

    fn insert_rec(
        node: Rc<RefCell<Node<K, V>>>,
        key: K,
        value: V,
    ) -> (Option<K>, Option<Rc<RefCell<Node<K, V>>>>) {
        let mut n = node.borrow_mut();

        match &mut *n {
            Node::Leaf { keys, values, next } => {
                match keys.binary_search(&key) {
                    Ok(i) => values[i] = value, // overwrite
                    Err(i) => {
                        keys.insert(i, key);
                        values.insert(i, value);
                    }
                }
                if keys.len() >= ORDER {
                    // split leaf
                    let mid = keys.len() / 2;
                    let split_keys = keys.split_off(mid);
                    let split_vals = values.split_off(mid);

                    let new_leaf = Rc::new(RefCell::new(Node::Leaf {
                        keys: split_keys.clone(),
                        values: split_vals,
                        next: next.take(),
                    }));

                    *next = Some(new_leaf.clone());
                    let sep_key = split_keys[0].clone();
                    return (Some(sep_key), Some(new_leaf));
                }
                (None, None)
            }

            Node::Internal { keys, children } => {
                let mut idx = keys.binary_search(&key).unwrap_or_else(|i| i);
                let (new_key, new_child) = Self::insert_rec(children[idx].clone(), key, value);

                if let Some((k, child)) = new_key.zip(new_child) {
                    keys.insert(idx, k);
                    children.insert(idx + 1, child);

                    if keys.len() >= ORDER {
                        // split internal node
                        let mid = keys.len() / 2;
                        let sep_key = keys[mid].clone();

                        let right_keys = keys.split_off(mid + 1);
                        let right_children = children.split_off(mid + 1);

                        let new_internal = Rc::new(RefCell::new(Node::Internal {
                            keys: right_keys,
                            children: right_children,
                        }));

                        keys.pop(); // remove sep_key from left half
                        return (Some(sep_key), Some(new_internal));
                    }
                }
                (None, None)
            }
        }
    }
}

// --- demo ---
fn main() {
    let mut tree = BPlusTree::new();
    for (k, v) in [(1,"a"),(2,"b"),(3,"c"),(4,"d"),(5,"e"),(6,"f")] {
        tree.insert(k, v);
    }

    println!("{:?}", tree.search(&3)); // Some("c")
    println!("{:?}", tree.search(&10)); // None
}

🚀 Next Steps Toward Database-Grade B+ Trees

Page-based storage: Nodes should be serialized into fixed-size pages (e.g., 4 KB).

Buffer manager: Cache hot pages in memory, flush dirty ones.

Persistence: Replace Rc<RefCell<_>> with page IDs + disk-backed pager.

Range scans: Add an iterator that follows leaf.next.

Concurrency: Implement latch crabbing (lock-coupling).

Deletes & merges: Balance underflowing nodes by borrowing/merging.

### Desired Features

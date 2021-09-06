#![no_main]

use std::collections::BTreeSet;
use std::fmt;

use arbitrary::Arbitrary;
use libfuzzer_sys::fuzz_target;

use scapegoat::{Node, NodeArena};

// Note: the hard_*() methods call their Option-returning equivalents
// E.g. hard_get() calls get()
// We only need to test the former to get coverage, since the harness upholds valid index invariants (same as library)
#[derive(Arbitrary, Debug)]
enum ArenaMethod<K: Ord + fmt::Debug, V: fmt::Debug> {
    New,
    // capacity() returns a constant. Omitted, irrelevant coverage.
    Add { key: K, val: V },
    Iter,
    IterMut,
    HardRemove { idx: usize },
    HardGet { idx: usize },
    HardGetMut { idx: usize },
    Len,
    // sort() exercised through SGMap fuzz target (input invariants are complex, tree structure related)
}

fuzz_target!(|methods: Vec<ArenaMethod<usize, usize>>| {
    let mut arena = NodeArena::new();   // Arena under test
    let mut idx_set = BTreeSet::new();  // Currently used arena indexs

    for m in methods {
        match m {
            ArenaMethod::New => {
                arena = NodeArena::new();
                idx_set.clear();
            },
            ArenaMethod::Add { key, val } => {
                let node = Node::new(key, val);
                let idx = arena.add(node);
                idx_set.insert(idx);
            },
            ArenaMethod::Iter => {
                let _ = arena.iter();
            },
            ArenaMethod::IterMut => {
                let _ = arena.iter_mut();
            },
            ArenaMethod::HardRemove { idx } => {
                match idx_set.remove(&idx) {
                    false => continue,
                    true => {
                        let _ = arena.hard_remove(idx);
                    }
                }
            },
            ArenaMethod::HardGet { idx } => {
                match idx_set.get(&idx) {
                    Some(valid_idx) => {
                        let _ = arena.hard_get(*valid_idx);
                    },
                    None => continue,
                }
            },
            ArenaMethod::HardGetMut { idx } => {
                match idx_set.get(&idx) {
                    Some(valid_idx) => {
                        let _ = arena.hard_get_mut(*valid_idx);
                    },
                    None => continue,
                }
            },
            ArenaMethod::Len => {
                let _ = arena.len();
            }
        }
    }
});
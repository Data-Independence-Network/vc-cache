
/**
 *  We need a dynamically sized data structure for adding polls.  The data structure should be
 *  memory efficient but even more importantly should be computationally efficient.  HashMap
 *  is limited by its need to re-hash.  Hence this is a custom ...
 *
 *  Least Significant digit (bit shift operation based) tree.  It consists of branch and leaf nodes
 *  of variable depth.  It grows by occasionally adding a root node to a (sub)-branch, and otherwise
 *  adding child branches.  It is only fully locked up for read access when the root node is being
 *  replaced due to addition.
 *
 *  The final leafs are Vec<Vec<PollId>>
 *
 *  It is computationally efficient (especially with higher branch counts) because navigation
 *  from a tree node to a tree node is based on bit shifting of the least significant digits
 *  and because of higher branch factors (defaults to 8 branches per node).
 *
 *  It is reasonably memory efficient and is acceptable from that point of view because it is only
 *  used for the future periods.
 *
 *  A Hash Trie Map optimized for integer keys and
 *
 */
pub struct LsbShiftTree<T> {
    rootNode:Vec<Node<T>>,
    rootBranchFactor: usize,
    branchFactor: usize,
}

impl<T> LsbShiftTree<T> {
    pub fn with_branch_factor(
        rootBranchFactor: usize,
        branchFactor: usize
    ) -> LsbShiftTree<T> {
        LsbShiftTree {
            rootNode: Vec::with_capacity(rootBranchFactor),
            rootBranchFactor,
            branchFactor
        }
    }

    pub fn set(mut self, key: u64, value: T) {

        match rootNode[key << 48] {
            None => {

            }
            Some(node) => {
                
            }
        }
    }

    pub fn get(self, index: I) -> T {

    }
}

struct Node<T> {
    value: Value<T>
}

enum Value<T> {
    Value(T),
    ChildNodes(Vec<Node<T>>)
}
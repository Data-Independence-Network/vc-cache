
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
pub struct GlobalNode<T> {
    rootNode:Vec<Node<T>>,
}

impl<T> GlobalNode<T> {
    pub fn new() -> GlobalNode<T> {
        LsbShiftTree {
            rootNode: Vec::with_capacity(16777216),
        }
    }

    pub fn set(mut self, key: u64, value: T) {
        let leafIndex = key & 0x00000fffu64;

        match self.rootNode.get(leafIndex) {
            None => {
                self.rootNode[leafIndex] = value;
            }
            Some(node) => {
                let subKey = key >> 24;
                match node {
                    Value(value) => {
                        let childNode = Node::new();
                        childNode.set(subKey, value);
                        self.rootNode[leafIndex] = childNode;
                    }
                    ChildNodes(childNode) => {
                        childNode.set(subKey, value);
                    }
                }
            }
        }
    }

    pub fn get(self, index: I) -> T {

    }
}

struct NestedNode<T> {
    value: Vec<Value<T>>
}

impl<T> NestedNode<T> {
    pub fn new() {
        Node {
            value: Vec::with_capacity(8)
        }
    }

    pub fn set(mut self, key: usize, value: T) {
        let mut leafIndex = key & 0x0000000fu64;
        let mut subKey = key;

        let mut array = self.value;
        loop {
        match array.get(leafIndex) {
            None => {
                array[leafIndex] = Value::Value(value);
                return;
            }
            Some(node) => {
                subKey = subKey >> 8;
                match node {
                    Value(existingKey, value) => {
                        
                        let data = Vec::with_capacity(8);
                        newValue = Value::ChildNode(data);
                        let childNode = Node::new();
                        childNode.set(subKey, value);
                        array[leafIndex] = childNode;
                        return;
                    }
                    ChildNodes(childArray) => {
                        leafIndex = subKey & 0x0000000fu64;
                        array = childArray;
                    }
                }
            }
        }
        }
    }

    pub fn get(self, key: usize) -> Option<V> {
        let leafIndex = key & 0x0000000fu64;

        match self.value.get(leafIndex) {
            None => {
                return Option::None;
            }
            Some(node) => {
                match node {
                    Value(value) => {
                        return Option::Some(value);
                    }
                    ChildNodes(childNode) => {
                        return childNode.get(key >> 8);
                    }
                }
            }
        }
    }
}

enum Value<T> {
    Value(usize, T),
    ChildNode(Vec<Value<T>>)
}
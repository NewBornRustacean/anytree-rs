use std::{cell::RefCell, rc::Rc};

type NodeRef<T> = Rc<RefCell<Node<T>>>;

#[derive(Default, Debug)]
pub struct Tree<T>{
    root: Option<NodeRef<T>>,
    height: i32,
}
#[derive(Default, Debug)]
pub struct Node<T> {
    children: Option<NodeRef<T>>,
    value: T,
}

impl<T> Node<T> {
    pub fn new(value: T) -> Self{
        Node {
            value: value,
            children: None,
        }
    }

    pub fn is_leaf(&self) -> bool{
        self.children.is_none()
    }
}

impl<T> Tree<T> {
    pub fn new() -> Self{
        Tree {
            root: None,
            height:0,
        }
    }
}

#[cfg(test)]
mod test_tree_node  {
    use super::*;

    #[test]
    fn test_tree_new() {
        let tree:Tree<i32> = Tree::new();
        assert_eq!(tree.height, 0);
    }

    #[test]
    fn test_node_new_is_leaf() {
        let node_int: Node<i32> = Node::new(42);
        assert_eq!(node_int.value, 42);
        assert!(node_int.is_leaf());

        let node_string: Node<String> = Node::new("hello".to_string());
        assert_eq!(node_string.value, "hello".to_string());
        assert!(node_string.is_leaf());
    }
}
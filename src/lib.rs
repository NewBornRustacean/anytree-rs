use std::{cell::RefCell, rc::Rc};

type NodeRef<T> = Rc<RefCell<Node<T>>>;

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
}


#[cfg(test)]
mod test_node  {
    use super::*;

    #[test]
    fn test_new() {
        let node_int: Node<i32> = Node::new(42);
        assert_eq!(node_int.value, 42);
        assert!(node_int.children.is_none());

        let node_string: Node<String> = Node::new("hello".to_string());
        assert_eq!(node_string.value, "hello".to_string());
        assert!(node_string.children.is_none());
    }
}
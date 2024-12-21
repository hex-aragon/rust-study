use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
struct Node {
    value: i32,
    children: RefCell<Vec<Rc<Node>>>,
}

impl Node{
    fn new(value: i32) -> Rc<Node>{
        Rc::new(Node{
            value,
            children: RefCell::new(vec![]),
        })
    }

    fn add_child(&self, child: Rc<Node>){
        self.children.borrow_mut().push(child);
    }
}

fn main() {
    let root = Node::new(1);
    let child1 = Node::new(2);
    let child2 = Node::new(3);

    root.add_child(Rc::clone(&child1));
    root.add_child(Rc::clone(&child2));

    child1.add_child(Rc::clone(&child2));

    println!("Root {:?}", root);
    #[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node_creation() {
        let node = Node::new(5);
        assert_eq!(node.value, 5);
        assert!(node.children.borrow().is_empty());
    }

    #[test]
    fn test_add_child() {
        let parent = Node::new(1);
        let child = Node::new(2);
        parent.add_child(Rc::clone(&child));
        assert_eq!(parent.children.borrow().len(), 1);
        assert_eq!(parent.children.borrow()[0].value, 2);
    }

    #[test]
    fn test_multiple_ownership() {
        let node1 = Node::new(1);
        let node2 = Node::new(2);
        let node3 = Node::new(3);

        node1.add_child(Rc::clone(&node2));
        node1.add_child(Rc::clone(&node3));
        node2.add_child(Rc::clone(&node3));

        assert_eq!(Rc::strong_count(&node3), 3);
    }
}
}

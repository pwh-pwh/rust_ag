use std::cell::{Cell, RefCell};
use std::rc::Rc;
use std::sync::Arc;

#[derive(Debug, Eq, PartialEq)]
struct Node {
    id: usize,
    downstream: Option<Rc<Node>>,
}

impl Node {
    fn new(id: usize) -> Self {
        Self {
            id,
            downstream: None,
        }
    }
    fn update_downstream(&mut self, downstream: Rc<Node>) {
        self.downstream = Some(downstream);
    }
    fn get_downstream(&self) -> Option<Rc<Node>> {
        self.downstream.as_ref().map(|r| r.clone())
    }
}

fn main() {
    let mut node1 = Node::new(1);
    let mut node2 = Node::new(2);
    let mut node3 = Node::new(3);
    let node4 = Node::new(4);
    node3.update_downstream(Rc::new(node4));
    node1.update_downstream(Rc::new(node3));
    node2.update_downstream(node1.get_downstream().unwrap());
    let node5 = Node::new(5);
    let node3 = node1.get_downstream().unwrap();
    //error Rc immut
    //node3.update_downstream(Rc::new(node5));
    println!("node1:{:#?}\n,node2:{:#?}", node1, node2);

    let data = RefCell::new(1);
    {
        let mut num = data.borrow_mut();
        *num += 1;
    }
    println!("data:{:?}", data.borrow());
    let c = Cell::new(3);
}

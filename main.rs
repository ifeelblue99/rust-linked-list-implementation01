use std::fmt::Display;

fn main() {
    let mut linked_list = LinkedList::new(Box::new(Node {
        value: 5,
        next: None,
    }));
}

struct Node<T: Display> {
    value: T,
    next: Option<Box<Node<T>>>,
}
impl<T> Node<T>
where
    T: Display,
{
    fn new(val: T) -> Self {
        Node {
            value: val,
            next: None,
        }
    }
    fn add_to_next(&mut self, node: Box<Node<T>>) {
        self.next = Some(node)
    }
    fn print(&self) {
        println!("Node value: {}", &self.value);
    }
}

struct LinkedList<T>
where
    T: Display,
{
    head: Option<Box<Node<T>>>,
    tail: Option<Box<Node<T>>>,
    length: u32,
}
impl<T> LinkedList<T>
where
    T: Display,
{
    fn new(node: Box<Node<T>>) -> LinkedList<T> {
        LinkedList {
            head: Some(node),
            tail: None,
            length: 0,
        }
    }
    fn add_node(&mut self, new_node: Box<Node<T>>) {
        match &mut self.tail {
            None => {
                self.tail = Some(new_node);
                self.length += 1;
                return;
            }
            Some(tail_node) => {
                tail_node.add_to_next(new_node);
                self.length += 1;
            }
        }
    }
    fn print_nodes(&self) {
        if let Some(current) = &self.tail {
            current.print();
        }
    }
}

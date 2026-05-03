use std::cell::RefCell;
use std::rc::Rc;

// 子要素を後から追加できるノードツリー。
// 単一スレッド内で複数所有 + 内部可変性が必要なケースの典型。
struct Node {
    value: u32,
    children: RefCell<Vec<Rc<Node>>>,
}

fn new_node(value: u32) -> Rc<Node> {
    Rc::new(Node {
        value,
        children: RefCell::new(Vec::new()),
    })
}

fn add_child(parent: &Rc<Node>, child: Rc<Node>) {
    parent.children.borrow_mut().push(child);
}

fn main() {
    let root = new_node(0);
    add_child(&root, new_node(1));

    let two = new_node(2);
    add_child(&root, Rc::clone(&two));
    add_child(&two, new_node(21));

    println!("root value = {}", root.value);
    println!("root has {} children", root.children.borrow().len());
    println!("two  has {} children", two.children.borrow().len());
    println!("two  strong_count = {}", Rc::strong_count(&two));
}

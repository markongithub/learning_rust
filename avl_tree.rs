type AVLTree<T> = Option<Box<AVLNode<T>>>;

// #[derive(Eq, PartialEq)]
struct AVLNode<T: Eq + Ord> {
    label: T,
    height: u8,
    left: AVLTree<T>,
    right: AVLTree<T>,
}

fn insert<T: Ord>(tree: &mut AVLTree<T>, new_label: T) {
    match &tree.as_ref() {
        Some(t) => println!("Inserting into a Some"),
        None => println!("Inserting into a None"),
    }
    //    println!("lol");
}

fn singleton<T: Ord>(new_label: T) -> AVLTree<T> {
    let new_node = AVLNode {
        label: new_label,
        height: 1,
        left: None,
        right: None,
    };
    Some(Box::new(new_node))
    //    panic!("I didn't implement this.");
}
/*
impl<T:Eq + Ord> MyAVLTree<T> {
    fn insert(&mut self, newLabel: T) {
        match &self.root {
            None => self.root = Some(Box::new(AVLNode {
                label: newLabel,
                height: 1,
                left: Box::new(MyAVLTree {root: None}),
                right: Box::new(MyAVLTree {root: None}),
            })),
            Some(node) => {
                if newLabel < node.label {
                    node.left.insert(newLabel);
                } else if newLabel > node.label {
                    node.right.insert(newLabel);
                } else {
                    panic!("I fucked up and didn't implement the equality case.");
                }
            }
        }
    }
}
*/

fn main() {
    println!("why is this necessary");
    let mut my_tree: AVLTree<char> = None;
    insert(&mut my_tree, 'a');
}

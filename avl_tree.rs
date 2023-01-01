type AVLTree<T> = Option<Box<AVLNode<T>>>;

// #[derive(Eq, PartialEq)]
struct AVLNode<T: Eq + Ord> {
    label: T,
    height: u8,
    left: AVLTree<T>,
    right: AVLTree<T>,
}

/*

impl<T: Eq + Ord> AVLNode<T> {
    fn insert_left(&mut self, new_label: T) {
        let ref_to_option: &Option<Box<AVLNode<T>>> = &mut self.left;
        let option_to_mut: Option<&mut Box<AVLNode<T>>> = ref_to_option.as_mut();
        if option_to_mut.is_some() {
            let node_ref: &AVLNode<T> = option_to_mut.unwrap();
            node_ref.insert_left(new_label);
        }
    }
}
*/
/*
fn insert<T: Ord>(tree: &mut AVLTree<T>, new_label: T) {
    if tree.is_some() {
        println!("Inserting into a Some");
        let option_immut_box: Option<&Box<AVLNode<T>>> = tree.as_ref();
        let immut_box_ref: &Box<AVLNode<T>> = option_immut_box.unwrap();
        let immut_box: Box<AVLNode<T>> = *immut_box_ref;
        //let left_child_option: &mut AVLTree<T> = *immut_box_ref.left;

    } else {
        println!("Inserting into a None");
        *tree = singleton(new_label);
    }
    //    println!("lol");
}
*/

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

struct AnotherNode {
    val: u32,
    l: Option<Box<AnotherNode>>,
    r: Option<Box<AnotherNode>>,
}

fn insert_different(not_self: &mut AnotherNode, new_val: u32) {
    if not_self.val == new_val {
        return;
    }
    let target_node: &mut Option<Box<AnotherNode>> = if new_val < not_self.val {
        &mut not_self.l
    } else {
        &mut not_self.r
    };

    let option_amp_mut: Option<&mut Box<AnotherNode>> = target_node.as_mut();
    if option_amp_mut.is_some() {
        println!("option_amp_mut is Some.");
        let mut_box: &mut Box<AnotherNode> = option_amp_mut.unwrap();
        //mut_box.insert(new_val);
        insert_different(mut_box, new_val);
    } else {
        let new_node = AnotherNode {
            val: new_val,
            l: None,
            r: None,
        };
        *target_node = Some(Box::new(new_node));
    }
}

fn insert_into_option(tree: &mut Option<Box<AnotherNode>>) {
    println!("here we go");
}

fn main() {
    println!("why is this necessary");
    let mut my_tree: AVLTree<char> = None;
    /*
        insert(&mut my_tree, 'a');
        insert(&mut my_tree, 'b');
    */
}

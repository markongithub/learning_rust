use std::fmt::Debug;

type AVLTree<T> = Option<Box<AVLNode<T>>>;

// #[derive(Eq, PartialEq)]
struct AVLNode<T: Eq + Ord> {
    label: T,
    height: u8,
    left: AVLTree<T>,
    right: AVLTree<T>,
}

fn insert_into_option<T: Ord>(tree: &mut AVLTree<T>, new_label: T) {
    println!("here we go");
    let option_amp_mut: Option<&mut Box<AVLNode<T>>> = tree.as_mut();
    if option_amp_mut.is_some() {
        println!("option_amp_mut is Some.");
        let mut_box: &mut Box<AVLNode<T>> = option_amp_mut.unwrap();
        let this_label: &T = &mut_box.label;
        let child: &mut AVLTree<T> = if new_label < *this_label {
            &mut mut_box.left
        } else {
            &mut mut_box.right
        };
        insert_into_option(child, new_label);
    } else {
        println!("option_amp_mut is None.");
        let new_tree: AVLTree<T> = singleton(new_label);
        *tree = new_tree;
    }
}

fn singleton<T: Ord>(new_label: T) -> AVLTree<T> {
    let new_node = AVLNode {
        label: new_label,
        height: 1,
        left: None,
        right: None,
    };
    Some(Box::new(new_node))
}

fn contains<T: Eq + Ord + Debug>(tree: &AVLTree<T>, target: T) -> bool {
    let option_amp_box: Option<&Box<AVLNode<T>>> = tree.as_ref();
    if option_amp_box.is_some() {
        let amp_box: &Box<AVLNode<T>> = option_amp_box.unwrap();
        let this_label: &T = &amp_box.label;
        println!("Checking for {0:?} on node {1:?}", target, this_label);
        if target == *this_label {
            return true;
        }
        let child: &AVLTree<T> = if target < *this_label {
            &amp_box.left
        } else {
            &amp_box.right
        };
        return contains(child, target);
    } else {
        return false;
    }
}

fn main() {
    println!("why is this necessary");
    let mut my_tree: AVLTree<char> = None;
    insert_into_option(&mut my_tree, 'a');
    insert_into_option(&mut my_tree, 'b');
    insert_into_option(&mut my_tree, 'c');
    insert_into_option(&mut my_tree, 'd');
    println!("Does the tree contain c? {0}", contains(&my_tree, 'c'));
    println!("Does the tree contain z? {0}", contains(&my_tree, 'z'));
}

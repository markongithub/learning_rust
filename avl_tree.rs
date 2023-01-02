use std::cmp::max;
use std::fmt::Debug;

type AVLTree<T> = Option<Box<AVLNode<T>>>;

// #[derive(Eq, PartialEq)]
struct AVLNode<T: Eq + Ord> {
    label: T,
    height: i64,
    left: AVLTree<T>,
    right: AVLTree<T>,
}

fn get_height<T: Ord>(tree: &AVLTree<T>) -> i64 {
    match tree {
        &Some(ref node) => node.height,
        &None => 0,
    }
}

fn balance_factor<T: Ord>(tree: &AVLTree<T>) -> i64 {
    match tree {
        &Some(ref node) => get_height(&node.right) - get_height(&node.left),
        &None => 0,
    }
}

fn insert<T: Ord>(tree: &mut AVLTree<T>, new_label: T) {
    println!("here we go");
    let option_amp_mut: Option<&mut Box<AVLNode<T>>> = tree.as_mut();
    if option_amp_mut.is_some() {
        println!("option_amp_mut is Some.");
        let mut_box: &mut Box<AVLNode<T>> = option_amp_mut.unwrap();
        let this_label: &T = &mut_box.label;
        let child: &mut AVLTree<T> = if new_label < *this_label {
            &mut mut_box.left
        } else if new_label > *this_label {
            &mut mut_box.right
        } else {
            panic!("What if we insert the same key twice?")
        };
        insert(child, new_label);
        mut_box.height = max(get_height(&mut_box.left), get_height(&mut_box.right)) + 1;
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

/*
node *rotate_Left(node *X, node *Z) {
    // Z is by 2 higher than its sibling
    t23 = left_child(Z); // Inner child of Z
    right_child(X) = t23;
    if (t23 != null)
        parent(t23) = X;
    left_child(Z) = X;
    parent(X) = Z;
    // 1st case, BF(Z) == 0,
    //   only happens with deletion, not insertion:
    if (BF(Z) == 0) { // t23 has been of same height as t4
        BF(X) = +1;   // t23 now higher
        BF(Z) = –1;   // t4 now lower than X
    } else
    { // 2nd case happens with insertion or deletion:
        BF(X) = 0;
        BF(Z) = 0;
    }
    return Z; // return new root of rotated subtree
}
*/

/*
fn rotate_left<T: Ord>(tree: &mut AVLTree<T>) {
    let option_amp_mut: Option<&mut Box<AVLNode<T>>> = tree.as_mut();
    if option_amp_mut.is_none() {
        return;
    }
    let old_root_x: &mut Box<AVLNode<T>> = option_amp_mut.unwrap();
    let new_root_z: &mut Box<AVLNode<T>> = &mut old_root_x.right.unwrap();
    old_root_x.right = new_root_z.left;
    new_root_z.left = *tree;
    let immutable_z: &Box<AVLNode<T>> = &new_root_z;
    *tree = Some(*immutable_z);
    // mut_box.right will be the new root;
    // mut_box will keep its old left child.
    // mut_box's new right child will be the former left_child of mut_box.right
}
*/

fn main() {
    println!("why is this necessary");
    let mut my_tree: AVLTree<char> = None;
    insert(&mut my_tree, 'a');
    insert(&mut my_tree, 'b');
    insert(&mut my_tree, 'c');
    insert(&mut my_tree, 'd');
    println!("The tree is now of height {0},", get_height(&my_tree));
    println!("Does the tree contain c? {0}", contains(&my_tree, 'c'));
    println!("Does the tree contain z? {0}", contains(&my_tree, 'z'));
}

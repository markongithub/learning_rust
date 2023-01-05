use std::cmp::max;
use std::fmt::Debug;
use std::mem::swap;

type AVLTree<T> = Option<Box<AVLNode<T>>>;

#[derive(Debug)]
struct AVLNode<T: Eq + Ord + Debug> {
    label: T,
    height: i64,
    left: AVLTree<T>,
    right: AVLTree<T>,
}

fn get_height<T: Ord + Debug>(tree: &AVLTree<T>) -> i64 {
    match tree {
        &Some(ref node) => node.height,
        &None => 0,
    }
}

fn balance_factor<T: Ord + Debug>(tree: &AVLTree<T>) -> i64 {
    match tree {
        &Some(ref node) => get_height(&node.left) - get_height(&node.right),
        &None => 0,
    }
}

fn rebalance<T: Ord + Debug>(tree: &mut AVLTree<T>) {
    let bf = balance_factor(tree);
    match bf {
        -1 | 0 | 1 => (),
        -2 => rotate_left_maybe_double(tree),
        2 => rotate_right_maybe_double(tree),
        x => panic!("Bad balance factor: {0:?}", x),
    }
}

fn insert<T: Ord + Debug>(tree: &mut AVLTree<T>, new_label: T) {
    let option_amp_mut: Option<&mut Box<AVLNode<T>>> = tree.as_mut();
    if option_amp_mut.is_some() {
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
        let left_height = get_height(&mut_box.left);
        let right_height = get_height(&mut_box.right);
        println!(
            "After inserting, left height is {0:?} and right height is {1}",
            left_height, right_height
        );
        mut_box.height = max(get_height(&mut_box.left), get_height(&mut_box.right)) + 1;
        rebalance(tree);
    } else {
        println!("option_amp_mut is None.");
        let new_tree: AVLTree<T> = singleton(new_label);
        *tree = new_tree;
    }
}

fn singleton<T: Ord + Debug>(new_label: T) -> AVLTree<T> {
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

fn rotate_left<T: Ord + Debug>(old_root_x: &mut AVLTree<T>) {
    if old_root_x.as_mut().is_none() {
        return;
    }
    if old_root_x.as_mut().unwrap().right.as_mut().is_none() {
        panic!("I am rotatingleft but my right child is None.");
    }
    let right_then_left_child: &mut AVLTree<T> = &mut old_root_x
        .as_mut()
        .unwrap()
        .right
        .as_mut()
        .unwrap()
        .left
        .take();
    let old_right_child: &mut AVLTree<T> = &mut old_root_x.as_mut().unwrap().right.take();
    let right_height = get_height(right_then_left_child);
    let left_height = get_height(&old_root_x.as_mut().unwrap().left);
    println!(
        "New left height is {0:?} and right height is {1}",
        left_height, right_height
    );
    let new_height: i64 = 1 + max(left_height, right_height);
    old_root_x.as_mut().unwrap().right = right_then_left_child.take();
    fix_height(old_root_x);
    old_right_child.as_mut().unwrap().left = old_root_x.take();
    old_right_child.as_mut().unwrap().height = new_height;
    *old_root_x = old_right_child.take();
    fix_height(old_root_x);
}

fn rotate_right<T: Ord + Debug>(old_root_x: &mut AVLTree<T>) {
    if old_root_x.as_mut().is_none() {
        return;
    }
    if old_root_x.as_mut().unwrap().left.as_mut().is_none() {
        panic!("I am rotating right but my left child is None.");
    }
    let left_then_right_child: &mut AVLTree<T> = &mut old_root_x
        .as_mut()
        .unwrap()
        .left
        .as_mut()
        .unwrap()
        .right
        .take();
    let old_left_child: &mut AVLTree<T> = &mut old_root_x.as_mut().unwrap().left.take();
    let left_height = get_height(left_then_right_child);
    let right_height = get_height(&old_root_x.as_mut().unwrap().right);
    println!(
        "New right height is {0:?} and left height is {1}",
        right_height, left_height
    );
    let new_height: i64 = 1 + max(right_height, left_height);
    old_root_x.as_mut().unwrap().left = left_then_right_child.take();
    fix_height(old_root_x);
    old_left_child.as_mut().unwrap().right = old_root_x.take();
    old_left_child.as_mut().unwrap().height = new_height;
    *old_root_x = old_left_child.take();
    fix_height(old_root_x);
}

fn rotate_left_maybe_double<T: Ord + Debug>(tree: &mut AVLTree<T>) {
    if tree.as_ref().is_none() {
        return;
    }
    let node = tree.as_mut().unwrap();
    println!("I am going to rotate left from {0:?}", node.label);
    if balance_factor(&node.right) > 0 {
        println!(
            "But first I must rotate right from {0:?}",
            node.right.as_ref().unwrap().label
        );
        rotate_right(&mut node.right);
        fix_height(tree);
    }
    rotate_left(tree);
}

fn rotate_right_maybe_double<T: Ord + Debug>(tree: &mut AVLTree<T>) {
    if tree.as_ref().is_none() {
        return;
    }
    let node = tree.as_mut().unwrap();
    println!("I am going to rotate right from {0:?}", node.label);
    if balance_factor(&node.left) < 0 {
        println!(
            "But first I must rotate left from {0:?}",
            node.left.as_ref().unwrap().label
        );
        rotate_left(&mut node.left);
        fix_height(tree);
    }
    rotate_right(tree);
}

fn in_order<T: Ord + Copy + Debug>(tree: &AVLTree<T>) -> Vec<T> {
    if tree.as_ref().is_none() {
        let empty: Vec<T> = vec![];
        return empty;
    }
    let node = tree.as_ref().unwrap();
    let mut from_left = in_order(&node.left);
    let from_right = in_order(&node.right);
    from_left.push(node.label);
    from_left.extend(from_right);
    from_left
}

fn fix_height<T: Ord + Debug>(tree: &mut AVLTree<T>) {
    // This only works if your two subtrees have accurate height. So fix them
    // first.
    let option_amp_mut: Option<&mut Box<AVLNode<T>>> = tree.as_mut();
    if option_amp_mut.is_some() {
        let mut_box: &mut Box<AVLNode<T>> = option_amp_mut.unwrap();
        let left_height = get_height(&mut_box.left);
        let right_height = get_height(&mut_box.right);
        println!(
            "Before fixing the height, left height is {0:?} and right height is {1}",
            left_height, right_height
        );
        let new_height = 1 + max(left_height, right_height);
        mut_box.height = new_height;
    }
}

fn find_min_and_delete<T: Ord + Debug>(tree: &mut AVLTree<T>) -> T {
    if tree.as_ref().is_none() {
        panic!("Don't call find_min_and_delete on an empty tree.");
    }
    let node = tree.as_mut().unwrap();
    if node.left.as_ref().is_some() {
        return find_min_and_delete(&mut node.left);
    }
    // we are at the minimum node. we promote our right child (empty or
    // not) and return the label.
    let right_child = node.right.take();
    let this_one = tree.take();
    *tree = right_child;
    return this_one.unwrap().label;
}

fn main() {
    println!("why is this necessary");
    let mut my_tree: AVLTree<char> = None;
    insert(&mut my_tree, 'a');
    insert(&mut my_tree, 'b');
    insert(&mut my_tree, 'c');
    insert(&mut my_tree, 'd');
    println!("The tree is now of height {0:?},", get_height(&my_tree));
    println!("Does the tree contain c? {0:?}", contains(&my_tree, 'c'));
    println!("Does the tree contain z? {0:?}", contains(&my_tree, 'z'));
    println!("All elements in order: {0:?}", in_order(&my_tree));

    println!(
        "Before rotating the label is {0:?} and the height is {1}",
        my_tree.as_ref().unwrap().label,
        get_height(&my_tree)
    );
    rotate_left(&mut my_tree);
    println!(
        "After rotating the label is {0:?} and the height is {1}",
        my_tree.as_ref().unwrap().label,
        get_height(&my_tree)
    );
    println!("All elements in order: {0:?}", in_order(&my_tree));
    println!("Can I Debug the tree? {0:?}", my_tree.as_ref().unwrap());
}

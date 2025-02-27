mod merkle_tree;

use merkle_tree::hash;

fn main() {
    let x = "qwertyu";
    let mut tree = merkle_tree::MerkleTree::new();
    let data ="qwerty";
    println!("{:?}",tree);
    tree.construct(data.as_bytes());
    println!("{:?}",tree);
}

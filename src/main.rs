#![allow(unused)]
mod merkle_tree;

use merkle_tree::hash;

fn main() {
    let x = "qwertyu";
    let tree =merkle_tree::MerkleTree::new(x.as_bytes());
    println!("{:?}",tree);
    let mut vec =vec![1,2,3,4,5];
    println!("{:?},", vec.split_at_mut(3));
    let r =tree.get_proof(3).unwrap();
    println!("{:?}",tree.verify_proof(3, r));
}

#![allow(unused)]
mod merkle_tree;
mod trees;

use std::hash::{DefaultHasher, Hash, Hasher};

use merkle_tree::hash;

struct H;

impl trees::HasherFunction<8> for H {
    fn hash<T:AsRef<[u8]>>(data:&T)->[u8;8] {
        let mut hasher = DefaultHasher::new();
        data.as_ref().hash(&mut hasher);
        unsafe { std::mem::transmute(hasher.finish()) }
    }
}

fn main() {
    let x = "qwertyu";
    let tree =merkle_tree::MerkleTree::new(x.as_bytes());
    println!("{:?}",tree);
    let mut vec =vec![1,2,3,4,5];
    println!("{:?},", vec.split_at_mut(3));
    let r =tree.get_proof(3).unwrap();
    println!("{:?}",tree.verify_proof(3, r));
}

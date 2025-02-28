#![allow(unused)]
mod merkle_tree;
mod trees;

use std::hash::{DefaultHasher, Hash, Hasher};

use merkle_tree::hash;
use trees::ToBytes;

#[derive(Debug)]
struct H;

impl trees::HasherFunction<8> for H {
    fn hash(left:&[u8;8],right:&[u8;8]) -> [u8; 8] {
        let mut hasher = DefaultHasher::new();
        let r =[*left,*right];
        r.as_flattened().hash(&mut hasher);
        // todo!()
        hasher.finish().to_ne_bytes()
    }
}

impl ToBytes for str {
    fn to_bytes(&self) -> &[u8] {
        self.as_bytes()
    }
}

fn main() {
    let x = &[&[4u8;8], &[9u8;8]]; 
    let hasher = H; 
    let tree = trees::MekleTree::new(x.as_slice(), hasher);
    println!("{:?}", tree);
    
    let mut vec = vec![1, 2, 3, 4, 5];
    println!("{:?},", vec.split_at_mut(1));
    
    let proof = tree.get_proof(1).unwrap();
    println!("{:?}", tree.verify_proof(1, proof));
}

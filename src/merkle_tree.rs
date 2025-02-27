use std::hash::{DefaultHasher, Hash, Hasher};


pub fn hash<T:Hash>(val:&T)->u64{
    let mut hasher = DefaultHasher::new();
    val.hash(&mut hasher);
    hasher.finish()
}

#[derive(Debug)]
pub struct MerkleTree{
    nodes:Vec<Vec<u64>>
}


impl MerkleTree {
    pub fn new()->Self{
        MerkleTree{
            nodes:vec![vec![]]
        }
    }

    pub fn construct<'a,T:Hash >(&mut self,data:&'a [T]){
        let mut level = 0;
        for val in data{
            let hash_op = hash(&data);
            self.nodes[level].push(hash_op);
        }
        let mut top_len = self.nodes[0].len();
        while top_len > 1 {
            self.nodes.push(vec![]);
            let mut range = (0..level/2);
            if level % 2 != 0{
                range = (0..level/2 + 1);
            }
            level += 1;
            let mut push_idx = 0usize;
            let mut idx = 0usize;
            let mut prev = [0,0];
            let (iter_part,append_part) = self.nodes.split_at_mut(level);
            for val in iter_part[level - 1].iter(){
                prev[idx&1] = *val;
                if idx & 1 == 1{
                    push_idx += 1;
                    let h_op = hash(&prev);
                    append_part[0].push(h_op);
                }
                idx += 1;
            }
            if idx&1 == 1 {
                prev[1] = prev[0];
                self.nodes[level].push(hash(&prev));
            }
            top_len = self.nodes[level].len();
        }
    }
}
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
    pub fn new<T:Hash>(data:& [T])->Self{
        let mut tree = MerkleTree{
            nodes:vec![vec![]]
        };
        tree.construct(data);
        tree
    }

    pub fn construct<'a,T:Hash >(&mut self,data:&'a [T]){
        let mut level = 0;
        for val in data{
            self.nodes[level].push(hash(val));
        }
        let mut peek_len = self.nodes[level].len();
        while peek_len > 1 {
            let mut cup = [0,0];
            self.nodes.push(vec![]);
            level += 1;
            let (iter_ref,append_ref) = self.nodes.split_at_mut(level);
            let mut idx = 0;
            for val in iter_ref[level - 1].iter(){
                cup[idx&1] = *val;
                if idx&1 == 1{
                    append_ref[0].push(hash(&cup));
                }
                idx += 1;
            }
            if idx&1 == 1 {
                cup[idx&1] = cup[0];
                append_ref[0].push(hash(&cup));
            }
            peek_len = self.nodes[level].len();
        }
    }
    pub fn get_proof(&self,mut idx: usize)->Option<Vec<u64>>{
        if idx < self.nodes.len(){
            return unsafe {Some(self.get_proof_unchecked(idx))} ;
        }
        None
    }
    pub unsafe fn get_proof_unchecked(&self,mut idx: usize)->Vec<u64>{
        let mut res = Vec::new();
        let mut level = 0;
        for levels in self.nodes.iter(){
            if levels.len() == 1{
                res.push(levels[0]);
                return res;
            }
            if idx&1 == 0{res.push(levels[idx + 1]);}else{res.push(levels[idx - 1])}
            idx /=  2;
        }
        res
    }
    
    pub fn verify_proof(&self,mut idx: usize,proof:Vec<u64>)->bool{
        // unimplemented!()
        match self.get_proof(idx){
            Some(res)=>res == proof,
            None=>false
        }
    }
}
use std::hash::Hash;
use std::marker::PhantomData;


pub trait HasherFunction<const N:usize>{
    fn hash(left:&[u8;N],right:&[u8;N])->[u8;N];
}
pub trait ToBytes {
    fn to_bytes(&self)->&[u8];
}


#[derive(Debug)]
pub struct MekleTree<const N:usize,F>
where F:HasherFunction<N>{
    nodes:Vec<Vec<[u8;N]>>,
    hash_fn:F,
}


impl<const N:usize,F> MekleTree<N,F>
where F:HasherFunction<N>{
    pub fn new(data:&[&[u8;N]],hasher_fn:F)->Self{
        let mut tree = MekleTree{
            nodes:vec![vec![]],
            hash_fn:hasher_fn,
        };
        tree.construct(data);
        tree
        // todo!()
    }
    pub fn construct(&mut self,data:&[&[u8;N]]){
        let hash = &self.hash_fn;
        let mut level = 0;
        for val in data{
            self.nodes[level].push(**val);
        }
        let mut peek_len = self.nodes[level].len();
        while peek_len > 1 {
            let mut cup = [[0;N];2];
            self.nodes.push(vec![]);
            level += 1;
            let (iter_ref,append_ref) = self.nodes.split_at_mut(level);
            let mut idx = 0;
            for val in iter_ref[level - 1].iter(){
                cup[idx&1] = *val;
                if idx&1 == 1{
                    append_ref[0].push(F::hash(&cup[0],&cup[1]));
                }
                idx += 1;
            }
            if idx&1 == 1 {
                cup[idx&1] = cup[0];
                append_ref[0].push(F::hash(&cup[0],&cup[1]));
            }
            peek_len = self.nodes[level].len();
        }
    }
    pub fn get_proof(&self,mut idx: usize)->Option<Vec<[u8;N]>>{
        if idx < self.nodes.len(){
            return unsafe {Some(self.get_proof_unchecked(idx))} ;
        }
        None
    }
    pub unsafe fn get_proof_unchecked(&self,mut idx: usize)->Vec<[u8;N]>{
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
    pub fn verify_proof(&self,mut idx: usize,proof:Vec<[u8;N]>)->bool{
        match self.get_proof(idx){
            Some(res)=>res == proof,
            None=>false
        }
    }
    pub fn get_root(&self)->Option<[u8;N]>{
        let last = self.nodes.last().unwrap();
        match last.len() > 0{
            true =>{Some(last[0])},
            false =>{None}
        }
    }

}
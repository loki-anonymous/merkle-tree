use std::hash::Hash;
use std::marker::PhantomData;


pub trait HasherFunction<const N:usize>{
    fn hash<T:AsRef<[u8]>>(data:&T)->[u8;N];
}


pub struct MekleTree<const N:usize,F,T:AsRef<[u8]>>
where F:Fn(&[u8])->[u8;N]{
    nodes:Vec<Vec<[u8;N]>>,
    hash_fn:F,
    _d:PhantomData<T>
}


impl<const N:usize,F,T:AsRef<[u8]>> MekleTree<N,F,T>
where F:Fn(&[u8])->[u8;N]{
    pub fn new(data:&[T],hasher_fn:F)->Self{
        let mut tree = MekleTree{
            nodes:vec![vec![]],
            hash_fn:hasher_fn,
            _d:PhantomData::<T>
        };
        tree.construct(data);
        tree
        // todo!()
    }
    pub fn construct(&mut self,data:&[T]){
        let hash = &self.hash_fn;
        let mut level = 0;
        for val in data{
            self.nodes[level].push(hash(val.as_ref()));
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
                    append_ref[0].push(hash(&cup.as_flattened()));
                }
                idx += 1;
            }
            if idx&1 == 1 {
                cup[idx&1] = cup[0];
                append_ref[0].push(hash(&cup.as_flattened()));
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
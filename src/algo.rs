use crate::uberblock::Uberblock;
use crate::object_type::ObjectType;
use crate::object_pointer::ObjectPointer;
use crate::context::Context;
use crate::tree::NodeEntry;

//fn insert(ctx: &mut Context, op: ObjectPointer, entry: NodeEntry<u64, u64>) -> (ObjectPointer, Option<u64>) {
pub fn test(){

    std::process::exit(0);
}

struct Test {
    raw: Box<[u8]>,
    u1: &'static u64,
    s: &'static str,
    u2: &'static u64,

}

impl Test {
    pub fn u1(&self) -> &u64 { &self.u1}
    pub fn s(&self) -> &str { &self.s}
    pub fn u2(&self) -> &u64 { &self.u2}
}


//trait AllTraits<'a> = serde::Serialize + serde::de::Deserialize<'a> + std::fmt::Debug;
/*
fn bla<'a, K: 'a+serde::Serialize + serde::de::Deserialize<'a> + std::fmt::Debug>(t: Opaque<K>) {
    println!("{:?}", t);
    let b = bincode::serialize(&t).unwrap();
    println!("{:?}", b);
    //let b = [0,1,2];
    blu::<Opaque<K>>(&b);
    
}

fn blu<'a, K: serde::de::Deserialize<'a> + std::fmt::Debug>(v: &'a[u8]) {
    let o: K = bincode::deserialize(&v).unwrap();
    println!("{:?}", o);
}
*/
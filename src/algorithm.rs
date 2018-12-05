

/*
pub fn merge(buffer: &BTreeMap<u64, u64>, trp: &ObjectPointer, sm: &mut SpaceManager)  {
    let any_node = sm.retrieve::<AnyNode<u64,u64>>(trp);
    match any_node {
        AnyNode::LeafNode(node) => {
            unimplemented!()

            //let it_buf = buffer.into_iter();
            //let it_node = node.entries.into_iter();


        },
        AnyNode::InternalNode(node) => {
            unimplemented!()
        }
    }

}
*/

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

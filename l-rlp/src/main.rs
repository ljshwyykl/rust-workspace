use std::mem::size_of;

fn main() {

    let animal = "dog";
    let out = rlp::encode(&animal);
    assert_eq!(out, vec![0x83, b'd', b'o', b'g']);

    println!("{:#?}", out);

    let str =  "Lorem ipsum dolor sit amet, consectetur adipisicing elit";
    // println!("{}", str.chars().count()); // 56
    // println!("{}", str.chars().count().to_string().chars().count());

    println!("{:#?}", str.as_bytes());
    println!("{:#?}", str.as_bytes().len());

    println!("{:#b}!", str.as_bytes().len());

    // println!("{:#?}", hex_string.as_bytes());
    //
    // println!("{:#?}", 0x38.len());



    // let animal1 = "Lorem ipsum dolor sit amet, consectetur adipisicing elit";
    // let out1 = rlp::encode(&animal1);
    //
    // println!("out1: {:#?}", out1);

}

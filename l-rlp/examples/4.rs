

fn main() {
    let animals = vec!["dog"];
    let encoded = rlp::encode_list::<&str,&str>(&animals);

    // let a = vec![vec![25], vec![10, 11, 12]];

    // let a = vec![25];
    // let encoded =  rlp::encode_list::<u8, u8>(&a);
    println!("{:#?}", encoded);
}
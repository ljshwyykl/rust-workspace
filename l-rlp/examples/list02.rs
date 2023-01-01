

fn main() {
    let test = vec!["Lorem ipsum dolor sit amet, consectetur adipisicing elit"];
    let encoded = rlp::encode_list::<&str,&str>(&test);
    println!("{:#?}", encoded); //b"\xf8:\xb88Lorem ipsum dolor sit amet, consectetur adipisicing elit"
}



/*
rlp::encode("Lorem ipsum dolor sit amet, consectetur adipisicing elit"); //

0xc0 + 0x37 + 0x01, 0xb88, 'L', 'o', ... , ' ', 'e', 'l', 'i', 't'

0xf8,0xb88, 'L', 'o', ... , ' ', 'e', 'l', 'i', 't'
 */


fn main() {
    let animals = vec!["dog","dog"];
    let encoded = rlp::encode_list::<&str,&str>(&animals);
    println!("{:#?}", encoded); // b"\xc8\x83dog\x83dog"
}

/*
 ## 列表编码

  * 首先需要对列表中的字符串和列表均进行编码 ( 字符串编码使用上一节的规则 ) 得到的列表是内部无嵌套的字节数组。
      * 如果列表长度(字节数)list_len在 0 到 55 字节，在列表中内容的 RLP 编码结果前添加一个字节，其值为0xc0 + list_len
      * 如果列表长度list_len大于 55 字节，则需要：
        * 首先得到列表长度十六进制表示的字节数len_of_list_len。
        * 在每个字节编码结束后的结果前添加两个部分，一个是 0xc0 + 0x37 + len_of_list_len, 另一个是list_len的字节编码形式结果


例如 vec!["dog","dog"]
``
rlp::encode("dog"); // [0x83, 'd', 'o', 'g']
rlp::encode(["dog", "dog"]) [ 0x83, 'd', 'o', 'g',0x83, 'd', 'o', 'g' ] len = 8 < 55

0xc0 + 0x08 , 0x83, 'd', 'o', 'g',0x83, 'd', 'o', 'g'
0xc8, 0x83, 'd', 'o', 'g',0x83, 'd', 'o', 'g'

``

```
fn main() {
    let animals = vec!["dog","dog"];
    let encoded = rlp::encode_list::<&str,&str>(&animals);
    println!("{:#?}", encoded); // b"\xc8\x83dog\x83dog"
}
```

例如 "Lorem ipsum dolor sit amet, consectetur adipisicing elit"

fn main() {
    let test = vec!["Lorem ipsum dolor sit amet, consectetur adipisicing elit"];
    let encoded = rlp::encode_list::<&str,&str>(&test);
    println!("{:#?}", encoded); //b"\xf8:\xb88Lorem ipsum dolor sit amet, consectetur adipisicing elit"
}


rlp::encode("Lorem ipsum dolor sit amet, consectetur adipisicing elit"); // 0xb88, 'L', 'o', ... , ' ', 'e', 'l', 'i', 't'

0xc0 + 0x37 + 0x01, 0xb88, 'L', 'o', ... , ' ', 'e', 'l', 'i', 't'

0xf8,0xb88, 'L', 'o', ... , ' ', 'e', 'l', 'i', 't'

 */

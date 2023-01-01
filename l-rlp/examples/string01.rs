fn main() {
    let animal = "dog";
    let encoded = rlp::encode(&animal);
    assert_eq!(encoded, vec![0x83, b'd', b'o', b'g']);
    println!("{:#?}", encoded); // b"\x83dog"


    print!("animal len: {}",animal.len()); // 3
}

/*
对于字符串中的单个字符，如果其值在[0x00, 0x7f]之间 ( 即[0, 127] )，其 RLP 编码值为其本身。然后再考虑下面字符串长度的情况
    a.如果一个字符串长度str_len在 0 到 55 字节，
      还需要在字符串每个字符编码结束后的结果前添加一个字节，
      这个字节的值为0x80 + str_len( 即128 + str_len，128 为偏移值 )。

例如，字符串 "dog"，长度为 3 ( 0x03 )，对于每个字符编码得到的结果为 ['d', 'o', 'g']，
由于其长度为 3 个字节 小于 55 字节，在其结果前添加 0x80 + str_len = 0x80 + 0x03 = 0x83，
所以有 ( len 函数用来输出输入的字节数 )：

```
animal.len()  // 3
0x80 + 0x03 , 'd', 'o', 'g'
0x83, 'd', 'o', 'g'
```

*/
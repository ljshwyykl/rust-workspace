fn main() {
    let test = "Lorem ipsum dolor sit amet, consectetur adipisicing elit";
    let encoded = rlp::encode(&test);

    println!("{:#?}", encoded); // b"\xb88Lorem ipsum dolor sit amet, consectetur adipisicing elit"


    print!("test len: {}",test.len()); // 56
}

/*
对于字符串中的单个字符，如果其值在[0x00, 0x7f]之间 ( 即[0, 127] )，其 RLP 编码值为其本身。然后再考虑下面字符串长度的情况
    a.如果一个字符串长度str_len在 0 到 55 字节，
      还需要在字符串每个字符编码结束后的结果前添加一个字节，
      这个字节的值为0x80 + str_len( 即128 + str_len，128 为偏移值 )。
    b.如果一个字符串长度**str_len**大于 55 ( 0x37 ) 字节，则需要
        1.首先得到字符串长度十六进制表示的字节数**len_of_str_len**。
        2.在每个字节编码结束后的结果前添加两个部分，一个是 0x80 + 0x37 + len_of_str_len, 另一个是 str_len 的字节编码形式结果。

a. 例如，字符串 "dog"，长度为 3 ( 0x03 )，对于每个字符编码得到的结果为 ['d', 'o', 'g']，
由于其长度为 3 个字节 小于 55 字节，在其结果前添加 0x80 + str_len = 0x80 + 0x03 = 0x83，
所以有 ( len 函数用来输出输入的字节数 )：

```
animal.len()  // 3
0x80 + 0x03 , 'd', 'o', 'g'
0x83, 'd', 'o', 'g'
```
b. 例如，字符串 "Lorem ipsum dolor sit amet, consectetur adipisicing elit"，
其长度为 56 ( 0x38，一个字节 ) 大于 55，首先计算长度值 56 的字节数为 1:

```
test.len()  // 56 0x38
0x80 + 0x37 + 0x01 , 0x38, 'L', 'o', ... , ' ', 'e', 'l', 'i', 't'
0xb88 ,'L', 'o', ... , ' ', 'e', 'l', 'i', 't'
```

*/
fn main() {
    let mut num = 32_u32;

    let a = &mut num;
    let b: &mut _ = a;
    *b += 1;
    *a += 1;

    println!("b {}", a);

    // let mut i = 1_u32;
    // mutate_twice(&mut i);
}


fn mutate(i: &mut u32) -> &mut u32 {
    *i += 1;
    i
}

fn mutate_twice(i: &mut u32) -> &mut u32 {
    mutate(i);
    mutate(i)

    // println!("{}", i);
}
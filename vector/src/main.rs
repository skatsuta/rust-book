fn main() {
    let v = vec![1, 2, 3];
    let mut v2 = Vec::new();
    v2.push(5);
    v2.push(6);

    let third = &v[2];
    let third2 = v.get(2);
    println!("third: {}, third2: {:?}", third, third2);

    let first = v2[1];
    v2.push(7);
}

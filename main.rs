fn main() {
    let mut a = vec![5556, 554];
    println!("{:?}", a);

    for i in 0..666 + a.len() {
        a[0] = i;
    }
    for i in (0..666 + a.len()).rev() {
        a[0] = i;
    }
    println!("{:?}", a);
}

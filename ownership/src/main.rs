fn main() {
    let s1 = String::from("hello");
    let s2 = s1.clone();
    let s3 = s2;
    let s4 = s3.clone();

    println!("s3 = {}, s4 = {}", s3, s4);
}

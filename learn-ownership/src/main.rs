fn main() {
    let num1: i32 = 88;
    let num2: f64 = 3.14;
    let s = String::from("hello");

    println!("s_length: {},s_capacity: {}", s.len(), s.capacity());
    println!(
        "num1:{}, num2:{}, s:{},",
        std::mem::size_of_val(&num1),
        std::mem::size_of_val(&num2),
        std::mem::size_of_val(&s)
    )
}

fn main() {
    let a = b'Z'; //90
    let b = b'a'; //97
    println!("{},{}", a, b);
    println!("----");
    for item in b'a'..=b'Z' {
        println!("{}", char::from(item));
    }
    println!("----");

    for item in b'Z'..=b'a' {
        println!("{}", char::from(item));
    }
}

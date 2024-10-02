fn main() {
    println!("a");

    #[cfg(feature = "ax")]
    println!("ax");

    #[cfg(feature = "ay")]
    println!("ay");
}

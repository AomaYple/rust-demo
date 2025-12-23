pub fn hello() {
    println!("Hello, rust-demo!");
}

#[cfg(test)]
mod test {
    #[test]
    fn test() {
        super::hello();
    }
}

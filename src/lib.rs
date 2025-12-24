pub fn hello() {
    println!("Hello, rust-demo!");
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        super::hello();
    }
}

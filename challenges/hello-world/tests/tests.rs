#[cfg(test)]
mod tests {
    use hello_world::*;

    #[test]
    fn should_return_hello_world() {
        assert_eq!(hello_world(), "Hello, World!");
    }
}

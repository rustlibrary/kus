#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
	assert_eq!(4, add_one(3));
    }
    fn add_one(x: i32) -> i32 {
        x + 1
    }
}

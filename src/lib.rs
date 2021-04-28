pub mod first;
pub mod second;
pub mod third;
pub mod tree;

#[cfg(test)]
mod tests {
    fn plus(x: i32, y: i32) ->i32 {
        x + y
    }

    #[test]
    fn it_works() {
        assert_eq!(plus(2, 2), 4);
    }
}

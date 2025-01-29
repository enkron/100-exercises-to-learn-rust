// TODO: based on what you learned in this section, replace `todo!()` with
//  the correct **stack size** for the respective type.
#[cfg(test)]
mod tests {
    use std::mem::size_of;

    #[test]
    fn u16_size() {
        println!("{}", size_of::<u16>());
        assert_eq!(size_of::<u16>(), 2);
    }

    #[test]
    fn i32_size() {
        println!("{}", size_of::<i32>());
        assert_eq!(size_of::<i32>(), 4);
    }

    #[test]
    fn bool_size() {
        println!("{}", size_of::<bool>());
        assert_eq!(size_of::<bool>(), 1);
    }
}

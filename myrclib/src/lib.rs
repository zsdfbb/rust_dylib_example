#[unsafe(no_mangle)]
pub extern "C" fn rc_add(left: u64, right: u64) -> u64 {
    println!("[myrclib] rc_add called");
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = rc_add(2, 2);
        assert_eq!(result, 4);
    }
}

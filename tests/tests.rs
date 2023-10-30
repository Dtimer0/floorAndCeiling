#[cfg(test)]
mod tests {
    use floorAndCeiling::*;

    #[test]
    fn ff32() {
        assert_eq!(floor32(2.4), 2);
    }
    #[test]
    fn ff64() {
        assert_eq!(floor64(9.8), 9);
    }
    #[test]
    fn cf32() {
        assert_eq!(ceiling32(2.2), 3);
    }
    #[test]fn cf64() {
        assert_eq!(ceiling32(9.8), 10);
    }
}

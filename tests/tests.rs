#[cfg(test)]
mod tests {
    use floorAndCeiling::*;
    #[test]
    /// Takes an f32 
    fn ff32() {
        assert_eq!(floor32(2.4), 2);
        assert_eq!(floor32(-3.2), -4);
        assert_eq!(floor32(0.0), 0);
    }
    #[test]
    fn ff64() {
        assert_eq!(floor64(9.8), 9);
        assert_eq!(floor64(-9.2), -10);
        assert_eq!(floor64(0.0), 0);
    }
    #[test]
    fn cf32() {
        assert_eq!(ceiling32(2.2), 3);
        assert_eq!(ceiling32(-2.2), -2);
        assert_eq!(ceiling32(0.0), 0);
    }
    #[test]fn cf64() {
        assert_eq!(ceiling64(9.8), 10);
        assert_eq!(ceiling64(-15.7), -15);
        assert_eq!(ceiling64(0.0), 0);
    }
}

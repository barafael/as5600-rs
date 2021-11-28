use crate::configuration::Configuration;
use proptest::prelude::*;

proptest! {
    #![proptest_config(ProptestConfig::with_cases(10000))]
    #[test]
    fn bytes_to_config_roundtrip(bytes in any::<u16>()) {
        let top_most_mask = 0b0011_1111_1111_1111;
        if let Ok(config) = Configuration::try_from(bytes) {
            let original = u16::from(config);
            assert_eq!(bytes & top_most_mask, original & top_most_mask);
        }
    }

    #[test]
    fn config_to_bytes_roundtrip(config in any::<Configuration>()) {
        let bytes = u16::from(config);
        assert_eq!(config, Configuration::try_from(bytes).unwrap());
    }
}

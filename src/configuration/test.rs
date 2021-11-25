use proptest::prelude::*;

use crate::configuration::Configuration;

proptest! {
    #![proptest_config(ProptestConfig::with_cases(10000))]
    #[test]
    fn bytes_to_config_roundtrip(bytes in any::<u16>()) {
        if let Ok(config) = Configuration::try_from(bytes) {
            if let Ok(original) = config.try_into() {
                assert_eq!(bytes, original);
            }
        }
    }

    #[test]
    fn config_to_bytes_roundtrip(config in any::<Configuration>()) {
        let bytes = u16::from(config);
        assert_eq!(config, Configuration::try_from(bytes).unwrap());
    }

    #[test]
    fn partial_eq(first in any::<u16>(), second in any::<u16>()) {
        if let Ok(first_config) = Configuration::try_from(first) {
            if let Ok(second_config) = Configuration::try_from(second) {
                if first != second {
                    assert_ne!(first_config, second_config);
                } else {
                    assert_eq!(first_config, second_config);
                }
            }
        }
    }
}

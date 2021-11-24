use proptest::prelude::*;

use crate::configuration::Configuration;

proptest! {
    #[test]
    fn config_roundtrip(bytes in any::<u16>()) {
        if let Ok(config) = Configuration::try_from(bytes) {
            if let Ok(original) = config.try_into() {
                assert_eq!(bytes, original);
            }
        }
    }

    #[test]
    fn configuration(config in any::<Configuration>()) {
        let bytes = u16::from(config);
        assert_eq!( config, Configuration::try_from(bytes).unwrap());
    }
}

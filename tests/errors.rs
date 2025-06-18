#[cfg(feature = "std")]
mod std_tests {
    use slip10::Error;

    #[test]
    fn test_std_error_trait() {
        let err = Error::InvalidIndex;

        // Test std::error::Error
        let error_trait: &dyn std::error::Error = &err;
        assert!(error_trait.source().is_none());

        // Test Display implementation
        assert_eq!(format!("{}", err), "Invalid index provided");

        // Test Debug implementation
        assert_eq!(format!("{:?}", err), "InvalidIndex");
    }

    #[test]
    fn test_error_chain_compatibility() {
        let err = Error::InvalidIndex;

        let returns_error = || -> Result<(), Box<dyn std::error::Error>> {
            Err(Box::new(err))
        };

        assert!(returns_error().is_err());
    }
}

#[cfg(all(not(feature = "std"), feature = "error"))]
mod core_tests {
    extern crate alloc;
    use slip10::Error;

    #[test]
    fn test_core_error_trait() {
        let err = Error::InvalidIndex;

        // Test core::error::Error
        let error_trait: &dyn core::error::Error = &err;
        assert!(error_trait.source().is_none());

        // Test Display implementation
        assert_eq!(format!("{}", err), "Invalid index provided");

        // Test Debug implementation
        assert_eq!(format!("{:?}", err), "InvalidIndex");
    }

    #[test]
    fn test_no_std_compatibility() {
        let err = Error::InvalidIndex;

        // Test that basic error functionality works in no_std
        let _formatted = alloc::format!("{}", err);
        let _debug = alloc::format!("{:?}", err);
    }
}

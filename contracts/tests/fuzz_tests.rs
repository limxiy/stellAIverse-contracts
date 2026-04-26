use proptest::prelude::*;
use crate::msg::KYCData;
use crate::validation::validate_kyc;

proptest! {
    #[test]
    fn fuzz_kyc(
        name in ".*",
        dob in 0u64..u64::MAX,
        country in ".*",
        doc in ".*"
    ) {
        let data = KYCData {
            full_name: name.clone(),
            date_of_birth: dob,
            country: country.clone(),
            document_hash: doc.clone(),
        };

        let now = 1_700_000_000;

        let result = validate_kyc(&data, now);

        let should_fail =
            name.trim().len() < 3 ||
            name.len() > 100 ||
            dob == 0 || dob > now ||
            country.len() != 2 ||
            doc.trim().is_empty() ||
            doc.len() > 128;

        if should_fail {
            prop_assert!(result.is_err());
        } else {
            prop_assert!(result.is_ok());
        }
    }
}
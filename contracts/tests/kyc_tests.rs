use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
use crate::contract::execute_submit_kyc;
use crate::msg::KYCData;

#[test]
fn rejects_invalid_name() {
    let mut deps = mock_dependencies();

    let data = KYCData {
        full_name: "".into(),
        date_of_birth: 1000,
        country: "US".into(),
        document_hash: "hash".into(),
    };

    let res = execute_submit_kyc(
        deps.as_mut(),
        mock_env(),
        mock_info("user", &[]),
        data,
    );

    assert!(res.is_err());
}

#[test]
fn rejects_future_dob() {
    let mut deps = mock_dependencies();

    let data = KYCData {
        full_name: "Alice".into(),
        date_of_birth: 9999999999,
        country: "US".into(),
        document_hash: "hash".into(),
    };

    let res = execute_submit_kyc(
        deps.as_mut(),
        mock_env(),
        mock_info("user", &[]),
        data,
    );

    assert!(res.is_err());
}

#[test]
fn rejects_duplicate_submission() {
    let mut deps = mock_dependencies();
    let env = mock_env();
    let info = mock_info("user", &[]);

    let data = KYCData {
        full_name: "Alice".into(),
        date_of_birth: 1000,
        country: "US".into(),
        document_hash: "hash".into(),
    };

    // First submission
    execute_submit_kyc(deps.as_mut(), env.clone(), info.clone(), data.clone()).unwrap();

    // Second should fail
    let res = execute_submit_kyc(deps.as_mut(), env, info, data);

    assert!(res.is_err());
}
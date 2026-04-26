use crate::msg::KYCData;
use crate::error::ContractError;

pub fn validate_kyc(data: &KYCData, current_time: u64) -> Result<(), ContractError> {
    if data.full_name.trim().len() < 3 {
        return Err(ContractError::InvalidKYC("Name too short".into()));
    }

    if data.full_name.len() > 100 {
        return Err(ContractError::InvalidKYC("Name too long".into()));
    }

    if data.date_of_birth == 0 || data.date_of_birth > current_time {
        return Err(ContractError::InvalidKYC("Invalid date of birth".into()));
    }

    if data.country.len() != 2 {
        return Err(ContractError::InvalidKYC("Invalid country code".into()));
    }

    if data.document_hash.trim().is_empty() {
        return Err(ContractError::InvalidKYC("Missing document hash".into()));
    }

    if data.document_hash.len() > 128 {
        return Err(ContractError::InvalidKYC("Document hash too long".into()));
    }

    Ok(())
}
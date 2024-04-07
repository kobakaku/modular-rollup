use jsonrpsee::types::ErrorObjectOwned;

pub fn to_jsonrpsee_error_object(message: &str, data: impl ToString) -> ErrorObjectOwned {
    tracing::warn!("Received invalid query.");
    ErrorObjectOwned::owned(
        jsonrpsee::types::error::UNKNOWN_ERROR_CODE,
        message,
        Some(data.to_string()),
    )
}

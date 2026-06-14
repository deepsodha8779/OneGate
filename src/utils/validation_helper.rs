use validator::ValidationErrors;

pub fn _collect_errors(error: ValidationErrors) -> Vec<String> {
    error
        .field_errors()
        .into_iter()
        .map(|error| {
            let default_error = format!("{} is required", error.0);
            error.1[0]
                .message
                .as_ref()
                .unwrap_or(&std::borrow::Cow::Owned(default_error))
                .to_string()
        })
        .collect()
}

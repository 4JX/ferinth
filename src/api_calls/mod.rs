use crate::{Error, Result};

pub mod project_calls;
pub mod tag_calls;
pub mod user_calls;
pub mod version_calls;

/// Verify that a given string `input` is base62 compliant
pub(crate) fn check_id_slug(input: &str) -> Result<()> {
    // This regex checks if there is any character that isn't valid in base62 e.g. '/'
    // It doesn't explicitly check for base62 because this might also be a project slug
    match lazy_regex::regex_is_match!("[^a-zA-Z0-9-]", input) {
        true => Err(Error::NotBase62),
        false => Ok(()),
    }
}

/// Verify that a given string `input` is SHA1 compliant
pub(crate) fn check_sha1_hash(input: &str) -> Result<()> {
    // This regex checks that all 40 character contain SHA1 compliant characters
    match lazy_regex::regex_is_match!("^[a-f0-9]{40}$", input) {
        true => Ok(()),
        false => Err(Error::NotSHA1),
    }
}

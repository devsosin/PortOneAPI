//! Parts shared by the B2B features (`tax_invoice`, `counterparty`): the [`B2bError`] enum, the
//! `test` query flag and the paging/sorting types. Feature specific types live in the feature.

mod errors;

pub use crate::features::receipt::dto::{
    request::{PageInput, SortOrder},
    response::PageInfo,
};
pub use errors::B2bError;

/// Query for the `test` flag every B2B endpoint takes (`test=true` runs in test mode).
pub(crate) fn test_query(test: bool) -> Vec<(&'static str, String)> {
    if test {
        vec![("test", "true".to_string())]
    } else {
        Vec::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flag_only_sent_when_true() {
        assert!(test_query(false).is_empty());
        assert_eq!(test_query(true), vec![("test", "true".to_string())]);
    }
}

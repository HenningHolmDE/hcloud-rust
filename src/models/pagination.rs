/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.14.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// Pagination : Information about the current pagination. The keys previous_page, next_page, last_page, and total_entries may be null when on the first page, last page, or when the total number of entries is unknown

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Pagination {
    /// ID of the last page available. Can be null if the current page is the last one. | The last page number
    #[serde(rename = "last_page", deserialize_with = "Option::deserialize")]
    pub last_page: Option<i32>,
    /// ID of the next page. Can be null if the current page is the last one. | The next page number
    #[serde(rename = "next_page", deserialize_with = "Option::deserialize")]
    pub next_page: Option<i32>,
    /// Current page number | The current page number
    #[serde(rename = "page")]
    pub page: i32,
    /// Maximum number of items shown per page in the response | The number of entries per page
    #[serde(rename = "per_page")]
    pub per_page: i32,
    /// ID of the previous page. Can be null if the current page is the first one. | The previous page number
    #[serde(rename = "previous_page", deserialize_with = "Option::deserialize")]
    pub previous_page: Option<i32>,
    /// The total number of entries that exist in the database for this query. Nullable if unknown. | The total number of entries
    #[serde(rename = "total_entries", deserialize_with = "Option::deserialize")]
    pub total_entries: Option<i32>,
}

impl Pagination {
    #![allow(clippy::too_many_arguments)]
    /// Information about the current pagination. The keys previous_page, next_page, last_page, and total_entries may be null when on the first page, last page, or when the total number of entries is unknown
    pub fn new(
        last_page: Option<i32>,
        next_page: Option<i32>,
        page: i32,
        per_page: i32,
        previous_page: Option<i32>,
        total_entries: Option<i32>,
    ) -> Pagination {
        Pagination {
            last_page,
            next_page,
            page,
            per_page,
            previous_page,
            total_entries,
        }
    }
}

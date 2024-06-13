//! # Rust Nested Value
//! 
//! Nested structures stores the following values ​​inside:
//! 
//! - ConstValue - constant values
//! - MutValue - mutable values
//! - FetchValue - values fetched from API
//! - MultiValue - collection, nested structire
//! 
//! Any value present in the structure can be accessible by string path  
//! such as 'constants/const-1' or 'api/api-value-1'
//! 
//! **Installation**
//! 
//! ```toml
//! nested-value = { git = "https://github.com/a-givertzman/rust-nested-value.git", tag = "0.0.4" }
//! ```
//! 
//! **For example constants & mutable value:**
//! 
//! ```ignore
//! use crate::{const_value::ConstValue, multi_value::MultiValue, mut_value::MutValue, nested_value::NestedValue};
//! 
//! let mut flags = MultiValue::new([
//!     ("bool-flags", Box::new(MultiValue::new([
//!         ("flag-true", Box::new(ConstValue::new(true))),
//!         ("flag-false", Box::new(MutValue::new(false))),
//!     ]))),
//! ]);
//! let key = "bool-flags/flag-true";
//! println!("multi value {}: {:?}", key, flags.get(key));
//! ```
//! 
//! **Example with fetched values:**
//! 
//! ```ignore
//! pub fn parse_value(reply: &[u8]) -> Result<serde_json::Value, String> {
//!     match serde_json::from_slice(reply) {
//!         Ok(reply) => {
//!             let reply: ApiReply = reply;
//!             match reply.data.first() {
//!                 Some(row) => {
//!                     match row.values().next() {
//!                         Some(value) => Ok(value.to_owned()),
//!                         None => Err(format!("request_value | value not present in the reply: {:?}", reply)),
//!                     }
//!                 }
//!                 None => Err(format!("request_value | value not present in the reply: {:?}", reply)),
//!             }
//!         },
//!         Err(err) => Err(format!("parse array error: {:?}", err)),
//!     }
//! }
//! let mut value = FetchValue::new(
//!     ApiRequest::new(
//!         self_id, address, auth_token,
//!         ApiQuery::new(ApiQueryKind::Sql(ApiQuerySql::new(database, "select 222;")), false),
//!         false, false,
//!     ),
//!     Box::new(|reply| {
//!         parse_value(reply)
//!     }),
//! ),
//! println!("multi value {}: {:?}", key, value.get(""));
//! ```
//! 
mod tests;
pub mod const_value;
pub mod fetch_value;
pub mod multi_value;
pub mod mut_value;
pub mod nested_value;
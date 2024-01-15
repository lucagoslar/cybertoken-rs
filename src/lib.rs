//! # cybertoken
//!
//! A Rust implementation of the [cybertoken](https://github.com/nikeee/cybertoken), a token format inspired by the GitHub API token format.
//!
//! ## Examples
//!
//! ### Generate Cybertoken
//!
//! ```no_run
//! use cybertoken::Cybertoken;
//!
//! fn main() {
//!    let cybertoken = Cybertoken::new("zugriff");
//!    let token = cybertoken.generate_token();
//!    println!("{}", token); // zugriff_6Jyot35CwMvmxtv9BvumECX9zbdOtFPfJ6Wj
//! }
//! ```
//!
//! ### Validate Cybertoken
//!
//! ```no_run
//! use cybertoken::Cybertoken;
//!
//! fn main() {
//!    let cybertoken = Cybertoken::new("zugriff");
//!    println!("valid {}", cybertoken.is_token_string("zugriff_6Jyot35CwMvmxtv9BvumECX9zbdOtFPfJ6Wj")); // valid true
//! }
//! ```
//! ### Parse Cybertoken
//!
//! ```no_run
//! use cybertoken::Cybertoken;
//!
//! fn main() {
//!    let cybertoken = Cybertoken::new("zugriff");
//!    println!("contents {:?}", cybertoken.parse_token_data("zugriff_6Jyot35CwMvmxtv9BvumECX9zbdOtFPfJ6Wj"));
//! }
//! ```

pub mod cybertoken;
pub mod error;

pub use cybertoken::Cybertoken;
pub use cybertoken::CybertokenContents;
pub use error::CybertokenError;

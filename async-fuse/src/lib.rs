#![deny(
    single_use_lifetimes,
    trivial_casts,
    trivial_numeric_casts,
    unstable_features,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    variant_size_differences,

    clippy::all,
    clippy::restriction,
    clippy::pedantic,
    clippy::nursery,
    // clippy::cargo
)]
#![allow(
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs,
    clippy::missing_docs_in_private_items,
    clippy::missing_errors_doc,
    clippy::blanket_clippy_restriction_lints,
    clippy::implicit_return,
    clippy::panic_in_result_fn,
    clippy::unwrap_in_result,
    clippy::unwrap_used,
    clippy::panic,
    clippy::indexing_slicing,
    clippy::wildcard_imports,
    clippy::module_name_repetitions,
    clippy::unreachable
)]

#[macro_use]
mod internel_macros;

mod abi_marker;
mod de;
mod encode;
mod fd;

mod ctx;
mod errno;
mod fs;
mod io;
mod server;

pub mod kernel;
pub mod ops;
pub mod types;
pub mod utils;

pub use self::ctx::FuseContext;
pub use self::errno::Errno;
pub use self::fd::FuseDesc;
pub use self::fs::FileSystem;
pub use self::io::FuseWrite;
pub use self::ops::Operation;
pub use self::server::Server;

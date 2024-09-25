#![allow(unused)]

#[cfg(feature = "io_mapping_test")]
mod io_mapping_test;
#[cfg(feature = "io_mapping_test")]
pub use io_mapping_test::*;

#[cfg(feature = "io_mapping_v1")]
mod io_mapping_v1;
#[cfg(feature = "io_mapping_v1")]
pub use io_mapping_v1::IOMapping;

#[cfg(feature = "io_mapping_v2")]
mod io_mapping_v2;
#[cfg(feature = "io_mapping_v2")]
pub use io_mapping_v2::IOMapping;

#[cfg(feature = "io_mapping_v3")]
mod io_mapping_v3;
#[cfg(feature = "io_mapping_v3")]
pub use io_mapping_v3::IOMapping;

#[cfg(feature = "io_mapping_v4")]
mod io_mapping_v4;
#[cfg(feature = "io_mapping_v4")]
pub use io_mapping_v4::*;

#[cfg(feature = "io_mapping_v5")]
mod io_mapping_v5;
#[cfg(feature = "io_mapping_v5")]
pub use io_mapping_v5::*;
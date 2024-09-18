#[allow(unused)]
mod io_mapping_v1;
#[allow(unused)]
mod io_mapping_v2;
#[allow(unused)]
mod io_mapping_v3;

#[cfg(feature = "io_mapping_v1")]
pub use io_mapping_v1::IOMapping;

#[cfg(feature = "io_mapping_v2")]
pub use io_mapping_v2::IOMapping;

#[cfg(feature = "io_mapping_v3")]
pub use io_mapping_v3::IOMapping;
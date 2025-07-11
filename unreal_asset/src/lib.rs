#![deny(missing_docs)]
#![allow(non_upper_case_globals)]
#![allow(elided_named_lifetimes)]
#![allow(clippy::needless_lifetimes)]

//! This crate is used for parsing Unreal Engine uasset files
//!
//! Supports both traditional UE4/UE5 package formats and the new UE5.3+ IoStore/ZenPackage formats.
//! Format detection is automatic and transparent to the user.
//!
//! # Examples
//!
//! ## Reading an asset that doesn't use bulk data
//!
//! ```no_run
//! use std::fs::File;
//!
//! use unreal_asset::{
//!     Asset,
//!     engine_version::EngineVersion,
//! };
//!
//! let mut file = File::open("asset.uasset").unwrap();
//! let mut asset = Asset::new(file, None, EngineVersion::VER_UE4_23, None).unwrap();
//!
//! println!("{:#?}", asset);
//! ```
//!
//! ## Reading an asset that uses bulk data
//!
//! ```no_run
//! use std::fs::File;
//!
//! use unreal_asset::{
//!     Asset,
//!     engine_version::EngineVersion,
//! };
//!
//! let mut file = File::open("asset.uasset").unwrap();
//! let mut bulk_file = File::open("asset.uexp").unwrap();
//! let mut asset = Asset::new(file, Some(bulk_file), EngineVersion::VER_UE4_23, None).unwrap();
//!
//! println!("{:#?}", asset);
//! ```
//!
//! ## Working with UE5.3+ ZenPackage format
//!
//! ```no_run
//! use std::fs::File;
//!
//! use unreal_asset::{
//!     Asset, AssetFormat,
//!     engine_version::EngineVersion,
//! };
//!
//! let mut file = File::open("ue53_asset.uasset").unwrap();
//! let mut asset = Asset::new(file, None, EngineVersion::VER_UE5_3, None).unwrap();
//!
//! match asset.format {
//!     AssetFormat::ZenPackage => println!("UE5.3+ ZenPackage format detected!"),
//!     AssetFormat::Traditional => println!("Traditional UE4/UE5 format"),
//!     AssetFormat::IoStore => println!("IoStore container format"),
//! }
//! ```

// sub crate reexports
// base
pub use unreal_asset_base as base;

pub use base::compression;
pub use base::containers;
pub use base::crc;
pub use base::custom_version;
pub use base::engine_version;
pub use base::enums;
pub use base::error;
pub use base::flags;
pub use base::import;
pub use base::object_version;
pub use base::reader;
pub use base::types;
pub use base::unversioned;

pub use base::cast;
pub use base::Guid;
pub use error::Error;
pub use import::Import;

// properties
pub use unreal_asset_properties as properties;

// kismet
pub use unreal_asset_kismet as kismet;

pub use kismet::KismetExpression;

// exports
pub use unreal_asset_exports as exports;

pub use exports::properties::fproperty;
pub use exports::properties::uproperty;

pub use exports::Export;

// registry
pub use unreal_asset_registry as registry;

// modules
pub mod ac7;
pub mod asset;
pub mod asset_archive_writer;
pub mod asset_data;
pub mod fengineversion;
pub mod io_store;
pub mod package_file_summary;

pub use asset::Asset;
pub use io_store::{AssetFormat, FZenPackageSummary, FormatDetector};

const UE4_ASSET_MAGIC: u32 = u32::from_be_bytes([0xc1, 0x83, 0x2a, 0x9e]);

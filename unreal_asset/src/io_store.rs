//! IoStore package format support for UE5.3+

use std::io::{Read, Seek, SeekFrom};

use byteorder::{ReadBytesExt, LE};

use unreal_asset_base::{
    error::Error, flags::EPackageFlags, reader::ArchiveTrait, types::PackageIndex,
};

/// IoStore package format detection and basic support
#[derive(Debug, Clone, PartialEq)]
pub enum AssetFormat {
    /// Traditional UE4/UE5 package format
    Traditional,
    /// UE5.3+ ZenPackage format
    ZenPackage,
    /// IoStore container format
    IoStore,
}

/// ZenPackage summary for UE5.3+ files
#[derive(Debug, Clone, Default)]
pub struct FZenPackageSummary {
    /// Whether the package has versioning info
    pub has_versioning_info: u32,
    /// Size of the header
    pub header_size: u32,
    /// Package name as mapped name
    pub name: FMappedName,
    /// Package flags
    pub package_flags: EPackageFlags,
    /// Cooked header size
    pub cooked_header_size: u32,
    /// Imported public export hashes offset
    pub imported_public_export_hashes_offset: i32,
    /// Import map offset
    pub import_map_offset: i32,
    /// Export map offset
    pub export_map_offset: i32,
    /// Export bundle entries offset
    pub export_bundle_entries_offset: i32,
    /// Graph data offset (pre-UE5.3)
    pub graph_data_offset: i32,
    /// Dependency bundle headers offset (UE5.3+)
    pub dependency_bundle_headers_offset: i32,
    /// Dependency bundle entries offset (UE5.3+)
    pub dependency_bundle_entries_offset: i32,
    /// Imported package names offset (UE5.3+)
    pub imported_package_names_offset: i32,
}

/// Mapped name for IoStore format
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct FMappedName {
    /// Name index
    pub index: u32,
    /// Name number  
    pub number: u32,
}

impl FMappedName {
    /// Create a new mapped name
    pub fn new(index: u32, number: u32) -> Self {
        Self { index, number }
    }

    /// Check if this is a global name
    pub fn is_global(&self) -> bool {
        (self.index & 0x80000000) != 0
    }

    /// Get the actual index without the global flag
    pub fn get_index(&self) -> u32 {
        self.index & 0x7FFFFFFF
    }
}

/// Export bundle header for IoStore format
#[derive(Debug, Clone, Default)]
pub struct FExportBundleHeader {
    /// First entry index in the bundle entries array
    pub first_entry_index: u32,
    /// Number of entries in this bundle
    pub entry_count: u32,
}

/// Export bundle entry for IoStore format
#[derive(Debug, Clone, Default)]
pub struct FExportBundleEntry {
    /// Local export index
    pub local_export_index: u32,
    /// Command type for this entry
    pub command_type: EExportCommandType,
}

/// Export command type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EExportCommandType {
    /// Create the export object
    Create,
    /// Serialize the export object
    Serialize,
}

impl Default for EExportCommandType {
    fn default() -> Self {
        Self::Create
    }
}

impl From<u32> for EExportCommandType {
    fn from(value: u32) -> Self {
        match value {
            0 => Self::Create,
            1 => Self::Serialize,
            _ => Self::Create, // Default fallback
        }
    }
}

/// Dependency bundle header for UE5.3+
#[derive(Debug, Clone, Default)]
pub struct FDependencyBundleHeader {
    /// First entry index
    pub first_entry_index: u32,
    /// Entry count
    pub entry_count: u32,
}

/// Dependency bundle entry for UE5.3+
#[derive(Debug, Clone, Default)]
pub struct FDependencyBundleEntry {
    /// Local import or export index
    pub local_index: u32,
}

impl FZenPackageSummary {
    /// Read ZenPackage summary from archive
    pub fn read<R: Read + Seek + ArchiveTrait<PackageIndex>>(
        archive: &mut R,
    ) -> Result<Self, Error> {
        let has_versioning_info = archive.read_u32::<LE>()?;
        let header_size = archive.read_u32::<LE>()?;

        // Read mapped name (simplified for now)
        let name_index = archive.read_u32::<LE>()?;
        let name_number = archive.read_u32::<LE>()?;
        let name = FMappedName::new(name_index, name_number);

        let package_flags = EPackageFlags::from_bits(archive.read_u32::<LE>()?)
            .ok_or_else(|| Error::invalid_file("Invalid package flags".to_string()))?;

        let cooked_header_size = archive.read_u32::<LE>()?;
        let imported_public_export_hashes_offset = archive.read_i32::<LE>()?;
        let import_map_offset = archive.read_i32::<LE>()?;
        let export_map_offset = archive.read_i32::<LE>()?;
        let export_bundle_entries_offset = archive.read_i32::<LE>()?;

        // Check if this is UE5.3+ based on engine version or other detection logic
        let is_ue53_plus = Self::is_ue53_plus(archive)?;

        let (
            graph_data_offset,
            dependency_bundle_headers_offset,
            dependency_bundle_entries_offset,
            imported_package_names_offset,
        ) = if is_ue53_plus {
            (
                0,
                archive.read_i32::<LE>()?,
                archive.read_i32::<LE>()?,
                archive.read_i32::<LE>()?,
            )
        } else {
            (archive.read_i32::<LE>()?, 0, 0, 0)
        };

        Ok(Self {
            has_versioning_info,
            header_size,
            name,
            package_flags,
            cooked_header_size,
            imported_public_export_hashes_offset,
            import_map_offset,
            export_map_offset,
            export_bundle_entries_offset,
            graph_data_offset,
            dependency_bundle_headers_offset,
            dependency_bundle_entries_offset,
            imported_package_names_offset,
        })
    }

    /// Detect if this is UE5.3+ format
    /// This is a simplified implementation - in practice would check engine version
    fn is_ue53_plus<R: Read + Seek + ArchiveTrait<PackageIndex>>(
        _archive: &mut R,
    ) -> Result<bool, Error> {
        // For now, assume it's UE5.3+ if we detect ZenPackage format
        // Real implementation would check the engine version from the archive
        Ok(true) // Placeholder
    }
}

impl FExportBundleHeader {
    /// Read export bundle header from archive
    pub fn read<R: Read + Seek>(archive: &mut R) -> Result<Self, Error> {
        let first_entry_index = archive.read_u32::<LE>()?;
        let entry_count = archive.read_u32::<LE>()?;

        Ok(Self {
            first_entry_index,
            entry_count,
        })
    }
}

impl FExportBundleEntry {
    /// Read export bundle entry from archive
    pub fn read<R: Read + Seek>(archive: &mut R) -> Result<Self, Error> {
        let raw_value = archive.read_u32::<LE>()?;

        // Extract command type and local export index from the raw value
        // Based on CUE4Parse implementation
        let command_type = EExportCommandType::from(raw_value >> 30);
        let local_export_index = raw_value & 0x3FFFFFFF;

        Ok(Self {
            local_export_index,
            command_type,
        })
    }
}

/// Format detection utilities
pub struct FormatDetector;

impl FormatDetector {
    /// Detect the asset format from the beginning of a file
    pub fn detect_format<R: Read + Seek>(mut reader: R) -> Result<AssetFormat, Error> {
        // Save current position
        let original_pos = reader.stream_position()?;

        // Read magic number
        reader.seek(SeekFrom::Start(0))?;
        let magic = reader.read_u32::<byteorder::BE>()?;

        // Check for traditional UE4/UE5 magic
        if magic == crate::UE4_ASSET_MAGIC {
            // Read legacy file version to determine format type
            let _legacy_version = reader.read_i32::<LE>()?;

            // For now, assume traditional format
            // Real implementation would check for ZenPackage indicators
            reader.seek(SeekFrom::Start(original_pos))?;
            return Ok(AssetFormat::Traditional);
        }

        // Check for other known magic numbers
        // TODO: Add IoStore container magic numbers

        // Restore position
        reader.seek(SeekFrom::Start(original_pos))?;

        Err(Error::invalid_file("Unknown asset format".to_string()))
    }

    /// Check if a traditional UE asset is actually using ZenPackage format
    pub fn is_zen_package<R: Read + Seek + ArchiveTrait<PackageIndex>>(
        _reader: &mut R,
    ) -> Result<bool, Error> {
        // This would involve more sophisticated detection logic
        // For now, return false to maintain compatibility
        Ok(false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_format_detection() {
        // Test traditional UE4 magic detection
        let ue4_magic_bytes = vec![0xc1, 0x83, 0x2a, 0x9e, 0x00, 0x00, 0x00, 0x00];
        let cursor = Cursor::new(ue4_magic_bytes);

        let format = FormatDetector::detect_format(cursor).unwrap();
        assert_eq!(format, AssetFormat::Traditional);
    }

    #[test]
    fn test_mapped_name() {
        let name = FMappedName::new(0x80000001, 0);
        assert!(name.is_global());
        assert_eq!(name.get_index(), 1);

        let local_name = FMappedName::new(42, 0);
        assert!(!local_name.is_global());
        assert_eq!(local_name.get_index(), 42);
    }

    #[test]
    fn test_export_command_type() {
        assert_eq!(EExportCommandType::from(0), EExportCommandType::Create);
        assert_eq!(EExportCommandType::from(1), EExportCommandType::Serialize);
        assert_eq!(EExportCommandType::from(999), EExportCommandType::Create); // fallback
    }
}

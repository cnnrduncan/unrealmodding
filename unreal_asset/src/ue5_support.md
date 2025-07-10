# UE5.3+ IoStore Support Implementation Plan

## Overview
Implement support for Unreal Engine 5.3+ IoStore file formats based on CUE4Parse implementation.

## Key Differences in UE5.3+ Files

### 1. IoStore Container Format
- Files are organized in IoStore containers (.utoc/.ucas)
- Individual assets are IoPackages instead of traditional Package format
- Header structure is completely different (FZenPackageSummary vs FPackageFileSummary)

### 2. Version Detection Changes
UE5.3+ introduces a new summary format that diverges from traditional UE4/UE5 packages:
- `FZenPackageSummary` instead of `FPackageFileSummary`
- Different magic numbers and version checks
- New versioning info structure (`FZenPackageVersioningInfo`)

### 3. Export/Import Organization
- Export bundles instead of traditional export maps
- Dependency bundles for UE5.3+ (replaces graph data)
- Different serialization offset handling

### 4. Name Map Changes
- Name map can be shared globally across containers
- Different name map serialization format
- Support for mapped names (`FMappedName`)

## Implementation Steps

### Phase 1: Detection and Format Identification
1. **Enhance Magic Number Detection**
   - Add detection for IoStore format markers
   - Detect ZenPackage vs traditional Package format
   - Add version range checks for UE5.3+

2. **Create IoStore Types**
   - `IoPackage` struct for IoStore packages
   - `FZenPackageSummary` for new summary format
   - `FZenPackageVersioningInfo` for version data

### Phase 2: Core IoStore Support
1. **ZenPackage Summary Parsing**
   - Implement `FZenPackageSummary::read()`
   - Handle version-specific field differences (UE5.3+ dependency bundles)
   - Parse versioning info when present

2. **Export Bundle System**
   - `FExportBundleHeader` and `FExportBundleEntry` types
   - Bundle-based export loading instead of traditional export map
   - Handle serialization offset calculations

3. **Dependency System for UE5.3+**
   - Dependency bundle headers/entries for UE5.3+
   - Replace graph data parsing with dependency bundles
   - Import package name handling

### Phase 3: Name Map and Global Data
1. **Mapped Name Support**
   - `FMappedName` type for IoStore name references
   - Global vs local name map handling
   - Name map batch loading

2. **Global Data Management**
   - Container-level shared data (name maps, script imports)
   - Cross-package reference resolution

### Phase 4: Integration and Testing
1. **Unified Asset Interface**
   - Extend current `Asset` type to handle both formats
   - Provide transparent API regardless of underlying format
   - Maintain backwards compatibility

2. **Comprehensive Testing**
   - Test with UE5.3+ .uasset/.umap files
   - Validate export/import parsing
   - Ensure property serialization works correctly

## Key Structures to Implement

```rust
// Core IoStore types
pub struct IoPackage<C: Read + Seek> {
    // ZenPackage-specific fields
    pub summary: FZenPackageSummary,
    pub export_bundles: Vec<FExportBundleHeader>,
    pub dependency_bundles: Option<Vec<FDependencyBundleHeader>>, // UE5.3+
    // ... other fields
}

pub struct FZenPackageSummary {
    pub has_versioning_info: u32,
    pub header_size: u32,
    pub name: FMappedName,
    pub package_flags: EPackageFlags,
    pub cooked_header_size: u32,
    pub import_map_offset: i32,
    pub export_map_offset: i32,
    pub export_bundle_entries_offset: i32,
    pub graph_data_offset: i32, // pre-UE5.3
    pub dependency_bundle_headers_offset: i32, // UE5.3+
    pub dependency_bundle_entries_offset: i32, // UE5.3+
    pub imported_package_names_offset: i32, // UE5.3+
}

pub struct FExportBundleHeader {
    pub first_entry_index: u32,
    pub entry_count: u32,
}

pub struct FExportBundleEntry {
    pub local_export_index: u32,
    pub command_type: EExportCommandType,
}

pub enum EExportCommandType {
    Create,
    Serialize,
}
```

## Detection Logic

```rust
impl<C: Read + Seek> Asset<C> {
    pub fn detect_format(&mut self) -> Result<AssetFormat, Error> {
        // Check magic number
        let magic = self.read_u32::<BE>()?;
        
        match magic {
            UE4_ASSET_MAGIC => {
                // Traditional UE4/UE5 package format
                // Check legacy version to determine if ZenPackage
                let legacy_version = self.read_i32::<LE>()?;
                
                if self.is_zen_package_format()? {
                    Ok(AssetFormat::ZenPackage)
                } else {
                    Ok(AssetFormat::Traditional)
                }
            }
            // Other magic numbers for specific formats
            _ => Err(Error::invalid_file("Unknown file format"))
        }
    }
    
    fn is_zen_package_format(&mut self) -> Result<bool, Error> {
        // Logic to detect ZenPackage vs traditional format
        // Based on version checks and header structure
        // ...
    }
}
```

## Benefits
- Full support for UE5.3+ asset files
- Maintains compatibility with existing UE4/UE5 files  
- Enables modding of newer Unreal Engine games
- Prepares for future UE versions using IoStore format

## Testing Strategy
- Create test suite with UE5.3+ sample files
- Validate against known-good CUE4Parse results
- Test export/import parsing and property serialization
- Ensure no regressions in existing UE4/UE5 support

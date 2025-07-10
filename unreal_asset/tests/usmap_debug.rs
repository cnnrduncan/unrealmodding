use byteorder::{ReadBytesExt, LE};
use std::fs;
use std::io::Cursor;
use unreal_asset_base::unversioned::{EUsmapCompressionMethod, EUsmapVersion};

#[test]
fn debug_oblivion_remastered_usmap() {
    // Read the usmap file
    let usmap_data = fs::read("tests/OblivionRemastered-5.3.2-.usmap")
        .expect("Failed to read OblivionRemastered-5.3.2-.usmap file");

    println!("Read usmap file, size: {} bytes", usmap_data.len());

    // Print first 32 bytes as hex for analysis
    println!("First 32 bytes:");
    for chunk in usmap_data[..std::cmp::min(32, usmap_data.len())].chunks(8) {
        for byte in chunk {
            print!("{:02X} ", byte);
        }
        println!();
    }

    let mut cursor = Cursor::new(&usmap_data);

    // Manually parse the header to debug
    let magic = cursor.read_u16::<LE>().expect("Failed to read magic");
    println!("Magic: 0x{:04X} (position: {})", magic, cursor.position());

    let usmap_version_byte = cursor.read_u8().expect("Failed to read version");
    println!(
        "Version byte: {} (position: {})",
        usmap_version_byte,
        cursor.position()
    );

    let usmap_version = EUsmapVersion::try_from(usmap_version_byte);
    println!("Version enum: {:?}", usmap_version);

    let is_ue4ss_format = usmap_version_byte == 0;
    println!("Is UE4SS format: {}", is_ue4ss_format);

    if is_ue4ss_format {
        println!("Using UE4SS format parsing");
        let compression = cursor.read_u8().expect("Failed to read compression");
        println!("Compression method: {}", compression);
    } else {
        println!("Using official format parsing");

        let usmap_version = usmap_version.expect("Invalid version");
        let mut has_versioning = usmap_version >= EUsmapVersion::PackageVersioning;
        println!("Initial has_versioning: {}", has_versioning);

        if has_versioning {
            let versioning_flag = cursor.read_u8().expect("Failed to read versioning flag");
            has_versioning = versioning_flag != 0;
            println!(
                "Versioning flag byte: 0x{:02X} -> has_versioning: {} (position: {})",
                versioning_flag,
                has_versioning,
                cursor.position()
            );
        }

        if has_versioning {
            let object_version = cursor
                .read_i32::<LE>()
                .expect("Failed to read object version");
            println!(
                "Object version: {} (position: {})",
                object_version,
                cursor.position()
            );

            let object_version_ue5 = cursor.read_i32::<LE>().expect("Failed to read UE5 version");
            println!(
                "Object version UE5: {} (position: {})",
                object_version_ue5,
                cursor.position()
            );

            // Skip custom versions array for now - this is complex
            let custom_versions_len = cursor
                .read_i32::<LE>()
                .expect("Failed to read custom versions length");
            println!(
                "Custom versions length: {} (position: {})",
                custom_versions_len,
                cursor.position()
            );

            // For debugging, let's skip the custom versions array
            println!("Skipping custom versions parsing for now...");
            return;
        }

        let compression = cursor.read_u8().expect("Failed to read compression");
        println!(
            "Compression method: {} (position: {})",
            compression,
            cursor.position()
        );
        let compression_method = EUsmapCompressionMethod::try_from(compression);
        println!("Compression enum: {:?}", compression_method);
    }

    let pos_before_sizes = cursor.position();
    let compressed_size = cursor
        .read_u32::<LE>()
        .expect("Failed to read compressed size");
    println!(
        "Compressed size: {} (position: {}, read from: {})",
        compressed_size,
        cursor.position(),
        pos_before_sizes
    );

    let decompressed_size = cursor
        .read_u32::<LE>()
        .expect("Failed to read decompressed size");
    println!(
        "Decompressed size: {} (position: {})",
        decompressed_size,
        cursor.position()
    );

    let remaining = usmap_data.len() as u64 - cursor.position();
    println!("Remaining bytes in file: {}", remaining);
    println!("Expected compressed data size: {}", compressed_size);

    // Print the bytes that were interpreted as the sizes
    println!(
        "Bytes interpreted as compressed size (starting at position {}):",
        pos_before_sizes
    );
    for i in 0..8 {
        if pos_before_sizes as usize + i < usmap_data.len() {
            print!("{:02X} ", usmap_data[pos_before_sizes as usize + i]);
        }
    }
    println!();

    if remaining < compressed_size as u64 {
        println!("❌ Not enough data remaining in file!");
        println!("This indicates the file might be truncated or the header parsing is incorrect.");
    } else {
        println!("✅ File has enough data for compressed payload");
    }
}

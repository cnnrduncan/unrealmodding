use byteorder::{ReadBytesExt, LE};
use std::fs;
use std::io::Cursor;

#[test]
fn test_oblivion_as_ue4ss_format() {
    // Read the usmap file
    let usmap_data = fs::read("tests/OblivionRemastered-5.3.2-.usmap")
        .expect("Failed to read OblivionRemastered-5.3.2-.usmap file");

    println!("Read usmap file, size: {} bytes", usmap_data.len());

    let mut cursor = Cursor::new(&usmap_data);

    // Try parsing as UE4SS format:
    // magic (2) + version (1) + compression (1) + compressed_size (4) + decompressed_size (4) + data

    let magic = cursor.read_u16::<LE>().expect("Failed to read magic");
    println!("Magic: 0x{:04X}", magic);

    let version = cursor.read_u8().expect("Failed to read version");
    println!("Version: {}", version);

    let compression = cursor.read_u8().expect("Failed to read compression");
    println!("Compression: {}", compression);

    let compressed_size = cursor
        .read_u32::<LE>()
        .expect("Failed to read compressed size");
    println!("Compressed size: {}", compressed_size);

    let decompressed_size = cursor
        .read_u32::<LE>()
        .expect("Failed to read decompressed size");
    println!("Decompressed size: {}", decompressed_size);

    let remaining = usmap_data.len() as u64 - cursor.position();
    println!("Remaining bytes: {}", remaining);

    if compressed_size as u64 <= remaining {
        println!("✅ UE4SS format interpretation looks correct!");
        println!("Compressed data size matches remaining bytes in file.");
    } else {
        println!("❌ UE4SS format interpretation also doesn't work.");
    }

    // Let's also try a different interpretation - maybe there are extra bytes
    cursor.set_position(2); // Back to after magic
    let _version = cursor.read_u8().expect("Failed to read version");

    // Skip some bytes and try reading sizes at different positions
    for skip_bytes in 1..8 {
        cursor.set_position(3 + skip_bytes);
        if cursor.position() + 8 < usmap_data.len() as u64 {
            let compressed_size = cursor.read_u32::<LE>().unwrap_or(0);
            let decompressed_size = cursor.read_u32::<LE>().unwrap_or(0);
            let remaining = usmap_data.len() as u64 - cursor.position();

            println!(
                "Try skipping {} bytes: compressed={}, decompressed={}, remaining={}",
                skip_bytes, compressed_size, decompressed_size, remaining
            );

            if compressed_size as u64 <= remaining
                && compressed_size > 0
                && compressed_size < 1_000_000
            {
                println!("  ✅ This looks reasonable!");
            }
        }
    }
}

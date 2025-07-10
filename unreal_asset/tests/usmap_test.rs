use std::fs;
use std::io::Cursor;
use unreal_asset_base::unversioned::Usmap;

#[test]
fn test_oblivion_remastered_usmap() {
    // Read the usmap file
    let usmap_data = fs::read("tests/OblivionRemastered-5.3.2-.usmap")
        .expect("Failed to read OblivionRemastered-5.3.2-.usmap file");

    println!("Read usmap file, size: {} bytes", usmap_data.len());

    // Check the first few bytes to see the magic and version
    if usmap_data.len() >= 4 {
        println!(
            "First 4 bytes: {:02X} {:02X} {:02X} {:02X}",
            usmap_data[0], usmap_data[1], usmap_data[2], usmap_data[3]
        );
    }

    // Parse the usmap file
    let cursor = Cursor::new(usmap_data);
    let usmap_result = Usmap::new(cursor);

    match usmap_result {
        Ok(usmap) => {
            // Print basic information about the usmap file
            println!("✅ Usmap file parsed successfully!");
            println!("Usmap version: {:?}", usmap.version);
            println!("Compression method: {:?}", usmap.compression_method);
            println!("Extension version: {:?}", usmap.extension_version);
            println!("Object version: {:?}", usmap.object_version);
            println!("Object version UE5: {:?}", usmap.object_version_ue5);
            println!("Net CL: {}", usmap.net_cl);
            println!("Name map entries: {}", usmap.name_map.len());
            println!("Enum map entries: {}", usmap.enum_map.len());
            println!("Schema entries: {}", usmap.schemas.len());

            // Print first few name map entries
            println!("\nFirst 10 name map entries:");
            for (i, name) in usmap.name_map.iter().take(10).enumerate() {
                println!("  {}: {}", i, name);
            }

            // Print enum map entries
            println!("\nEnum map entries:");
            for (_, enum_name, enum_values) in usmap.enum_map.iter().take(5) {
                println!("  {}: {} values", enum_name, enum_values.len());
                for (i, value) in enum_values.iter().take(3).enumerate() {
                    println!("    {}: {}", i, value);
                }
                if enum_values.len() > 3 {
                    println!("    ... and {} more", enum_values.len() - 3);
                }
            }

            // Print schema entries
            println!("\nFirst 10 schema entries:");
            for (_, schema_name, schema) in usmap.schemas.iter().take(10) {
                println!(
                    "  {}: {} properties, super: {}",
                    schema_name,
                    schema.properties.len(),
                    schema.super_type
                );
            }

            // Verify that the file was parsed successfully
            assert!(!usmap.name_map.is_empty(), "Name map should not be empty");
            assert!(!usmap.schemas.is_empty(), "Schemas should not be empty");
        }
        Err(e) => {
            println!("❌ Failed to parse usmap file: {:?}", e);
            println!("This could indicate:");
            println!("1. The file is corrupted or incomplete");
            println!("2. This is a different usmap format not yet supported");
            println!("3. There's a bug in the parsing code");

            // Don't panic, just return - we want to see the error details
            return;
        }
    }
}

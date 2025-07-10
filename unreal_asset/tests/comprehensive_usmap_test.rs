use std::fs;
use std::io::Cursor;
use unreal_asset_base::unversioned::Usmap;

#[test]
fn comprehensive_oblivion_usmap_test() {
    // Read the usmap file
    let usmap_data = fs::read("tests/OblivionRemastered-5.3.2-.usmap")
        .expect("Failed to read OblivionRemastered-5.3.2-.usmap file");

    println!("📁 File size: {} bytes", usmap_data.len());

    // Parse the usmap file
    let cursor = Cursor::new(usmap_data);
    let usmap = Usmap::new(cursor).expect("Failed to parse OblivionRemastered-5.3.2-.usmap file");

    println!("✅ Successfully parsed usmap file");
    println!("📊 Basic stats:");
    println!("   Version: {:?}", usmap.version);
    println!("   Compression: {:?}", usmap.compression_method);
    println!("   Names: {}", usmap.name_map.len());
    println!("   Enums: {}", usmap.enum_map.len());
    println!("   Schemas: {}", usmap.schemas.len());

    // Test 1: Validate name map integrity
    println!("\n🔍 Testing name map integrity...");
    let mut empty_names = 0;
    let mut very_long_names = 0;
    let mut total_name_chars = 0;

    for (i, name) in usmap.name_map.iter().enumerate() {
        if name.is_empty() {
            empty_names += 1;
            if empty_names <= 5 {
                println!("   ⚠️  Empty name at index {}", i);
            }
        }
        if name.len() > 200 {
            very_long_names += 1;
            if very_long_names <= 3 {
                println!("   ⚠️  Very long name at {}: {} chars", i, name.len());
            }
        }
        total_name_chars += name.len();
    }

    println!("   Names statistics:");
    println!("     Empty names: {}", empty_names);
    println!("     Very long names (>200 chars): {}", very_long_names);
    println!(
        "     Average name length: {:.1} chars",
        total_name_chars as f64 / usmap.name_map.len() as f64
    );

    // Test 2: Validate enum map integrity
    println!("\n🔍 Testing enum map integrity...");
    let mut empty_enums = 0;
    let mut large_enums = 0;
    let mut total_enum_values = 0;

    for (_, enum_name, enum_values) in usmap.enum_map.iter() {
        if enum_values.is_empty() {
            empty_enums += 1;
            if empty_enums <= 5 {
                println!("   ⚠️  Empty enum: {}", enum_name);
            }
        }
        if enum_values.len() > 100 {
            large_enums += 1;
            if large_enums <= 3 {
                println!(
                    "   📊 Large enum {}: {} values",
                    enum_name,
                    enum_values.len()
                );
            }
        }
        total_enum_values += enum_values.len();

        // Validate enum value names
        for enum_value in enum_values {
            if enum_value.is_empty() {
                println!("   ⚠️  Empty enum value in {}", enum_name);
            }
        }
    }

    println!("   Enum statistics:");
    println!("     Empty enums: {}", empty_enums);
    println!("     Large enums (>100 values): {}", large_enums);
    println!("     Total enum values: {}", total_enum_values);
    println!(
        "     Average values per enum: {:.1}",
        total_enum_values as f64 / usmap.enum_map.len() as f64
    );

    // Test 3: Validate schema integrity
    println!("\n🔍 Testing schema integrity...");
    let mut empty_schemas = 0;
    let mut large_schemas = 0;
    let mut schemas_with_super = 0;
    let mut total_properties = 0;
    let mut orphaned_schemas = 0;

    for (_, schema_name, schema) in usmap.schemas.iter() {
        if schema.properties.is_empty() {
            empty_schemas += 1;
            if empty_schemas <= 5 {
                println!("   📝 Empty schema: {}", schema_name);
            }
        }

        if schema.properties.len() > 50 {
            large_schemas += 1;
            if large_schemas <= 3 {
                println!(
                    "   📊 Large schema {}: {} properties",
                    schema_name,
                    schema.properties.len()
                );
            }
        }

        if !schema.super_type.is_empty() {
            schemas_with_super += 1;
            // Check if super type exists
            if !usmap.schemas.get_by_key(&schema.super_type).is_some() {
                orphaned_schemas += 1;
                if orphaned_schemas <= 5 {
                    println!(
                        "   ⚠️  Schema {} has non-existent super type: {}",
                        schema_name, schema.super_type
                    );
                }
            }
        }

        total_properties += schema.properties.len();

        // Validate property count matches actual properties
        if schema.prop_count as usize != schema.properties.len() {
            println!(
                "   ⚠️  Schema {} prop_count mismatch: declared={}, actual={}",
                schema_name,
                schema.prop_count,
                schema.properties.len()
            );
        }

        // Check for duplicate property names (same name, different duplication_index)
        let mut property_names = std::collections::HashMap::new();
        for (_, (prop_name, dup_index), _property) in &schema.properties {
            let entry = property_names
                .entry(prop_name.clone())
                .or_insert(Vec::new());
            entry.push(*dup_index);
        }

        for (prop_name, dup_indices) in property_names {
            if dup_indices.len() > 1 {
                println!(
                    "   📊 Schema {} has property {} with {} duplicates: {:?}",
                    schema_name,
                    prop_name,
                    dup_indices.len(),
                    dup_indices
                );
            }
        }
    }

    println!("   Schema statistics:");
    println!("     Empty schemas: {}", empty_schemas);
    println!("     Large schemas (>50 props): {}", large_schemas);
    println!("     Schemas with super types: {}", schemas_with_super);
    println!(
        "     Orphaned schemas (invalid super): {}",
        orphaned_schemas
    );
    println!("     Total properties: {}", total_properties);
    println!(
        "     Average properties per schema: {:.1}",
        total_properties as f64 / usmap.schemas.len() as f64
    );

    // Test 4: Test property lookup functionality
    println!("\n🔍 Testing property lookup functionality...");
    let test_schemas = usmap.schemas.iter().take(10).collect::<Vec<_>>();
    let mut successful_lookups = 0;
    let mut failed_lookups = 0;

    for (_, schema_name, schema) in &test_schemas {
        // Test getting properties by name
        for (_, (prop_name, dup_index), _) in schema.properties.iter().take(3) {
            match schema.get_property(prop_name, *dup_index) {
                Some(_) => successful_lookups += 1,
                None => {
                    failed_lookups += 1;
                    println!(
                        "   ⚠️  Failed to lookup property {} in schema {}",
                        prop_name, schema_name
                    );
                }
            }
        }
    }

    println!("   Property lookup test:");
    println!("     Successful lookups: {}", successful_lookups);
    println!("     Failed lookups: {}", failed_lookups);

    // Test 5: Test get_all_properties functionality
    println!("\n🔍 Testing inheritance chain functionality...");
    let mut inheritance_tests = 0;
    let mut inheritance_chains_found = 0;

    for (_, schema_name, schema) in usmap.schemas.iter().take(20) {
        inheritance_tests += 1;
        let all_properties = usmap.get_all_properties(schema_name);

        if all_properties.len() > schema.properties.len() {
            inheritance_chains_found += 1;
            if inheritance_chains_found <= 5 {
                println!(
                    "   📊 Schema {} inherits: own={}, total={}",
                    schema_name,
                    schema.properties.len(),
                    all_properties.len()
                );
            }
        }
    }

    println!("   Inheritance test:");
    println!("     Schemas tested: {}", inheritance_tests);
    println!(
        "     Inheritance chains found: {}",
        inheritance_chains_found
    );

    // Test 6: Test some common UE property types
    println!("\n🔍 Testing for common UE property types...");
    let common_types = [
        "Vector",
        "Vector2D",
        "Rotator",
        "Transform",
        "Color",
        "LinearColor",
        "FString",
        "FName",
        "TSoftObjectPtr",
        "TArray",
        "TMap",
        "TSet",
        "UObject",
        "AActor",
        "UComponent",
        "UStaticMesh",
        "UTexture",
    ];

    let mut found_types = 0;
    for common_type in &common_types {
        if usmap.schemas.get_by_key(*common_type).is_some() {
            found_types += 1;
            println!("   ✅ Found common type: {}", common_type);
        }
    }

    println!(
        "   Common types found: {}/{}",
        found_types,
        common_types.len()
    );

    // Test 7: Memory usage estimation
    println!("\n📊 Memory usage estimation:");
    let name_map_size = usmap.name_map.iter().map(|s| s.len()).sum::<usize>();
    let enum_map_size = usmap
        .enum_map
        .iter()
        .map(|(_, name, values)| name.len() + values.iter().map(|v| v.len()).sum::<usize>())
        .sum::<usize>();
    let schema_map_size = usmap
        .schemas
        .iter()
        .map(|(_, name, schema)| {
            name.len()
                + schema.super_type.len()
                + schema
                    .properties
                    .iter()
                    .map(|(_, (prop_name, _), _)| prop_name.len())
                    .sum::<usize>()
        })
        .sum::<usize>();

    println!("   Name map: ~{} KB", name_map_size / 1024);
    println!("   Enum map: ~{} KB", enum_map_size / 1024);
    println!("   Schema map: ~{} KB", schema_map_size / 1024);
    println!(
        "   Total string data: ~{} KB",
        (name_map_size + enum_map_size + schema_map_size) / 1024
    );

    // Final validation
    println!("\n✅ Comprehensive test completed!");
    println!("🎯 Summary:");
    println!("   • File parsing: SUCCESS");
    println!(
        "   • Name map: {} entries, {} empty",
        usmap.name_map.len(),
        empty_names
    );
    println!(
        "   • Enum map: {} enums, {} values total",
        usmap.enum_map.len(),
        total_enum_values
    );
    println!(
        "   • Schema map: {} schemas, {} properties total",
        usmap.schemas.len(),
        total_properties
    );
    println!(
        "   • Property lookups: {}/{} successful",
        successful_lookups,
        successful_lookups + failed_lookups
    );
    println!(
        "   • Memory usage: ~{} KB",
        (name_map_size + enum_map_size + schema_map_size) / 1024
    );

    // Assert critical conditions
    assert!(
        usmap.name_map.len() > 1000,
        "Should have substantial name map"
    );
    assert!(
        usmap.enum_map.len() > 100,
        "Should have substantial enum map"
    );
    assert!(
        usmap.schemas.len() > 1000,
        "Should have substantial schema map"
    );
    assert!(failed_lookups == 0, "All property lookups should succeed");
    assert!(
        empty_names < usmap.name_map.len() / 10,
        "Too many empty names"
    );

    println!("\n🎉 All validation checks passed!");
}

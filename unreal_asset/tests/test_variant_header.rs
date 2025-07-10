use std::io::Cursor;
use unreal_asset_base::unversioned::Usmap;

#[test]
fn test_schema_and_struct_analysis() {
    // Use the test data from comprehensive test
    let test_file_path = "C:/Users/Connor/unreal_rust_tools/unrealmodding/unreal_asset/tests/OblivionRemastered-5.3.2-.usmap";

    if !std::path::Path::new(test_file_path).exists() {
        println!("⚠️ OblivionRemastered usmap file not found, skipping detailed analysis");
        return;
    }

    let usmap_data = std::fs::read(test_file_path).expect("Failed to read usmap file");
    let cursor = Cursor::new(usmap_data);

    let usmap = Usmap::new(cursor).expect("Failed to parse usmap file");

    println!("🔍 Analyzing Schema and Struct Format");
    println!("Total schemas: {}", usmap.schemas.len());

    // Analyze different types of schemas
    let mut struct_schemas = Vec::new();
    let mut class_schemas = Vec::new();
    let mut empty_schemas = Vec::new();
    let mut small_schemas = Vec::new();
    let mut large_schemas = Vec::new();

    for (_, name, schema) in usmap.schemas.iter() {
        if schema.properties.is_empty() {
            empty_schemas.push((name, schema));
        } else if schema.prop_count <= 5 {
            small_schemas.push((name, schema));
        } else if schema.prop_count > 50 {
            large_schemas.push((name, schema));
        }

        // Try to classify as struct vs class based on naming patterns
        if name.starts_with("F")
            || name.contains("Struct")
            || matches!(
                name.as_str(),
                "Vector" | "Vector2D" | "Rotator" | "Transform" | "Color" | "LinearColor"
            )
        {
            struct_schemas.push((name, schema));
        } else if name.starts_with("U")
            || name.starts_with("A")
            || name.contains("Component")
            || name.contains("Actor")
        {
            class_schemas.push((name, schema));
        }
    }

    println!("\n📊 Schema Classification:");
    println!("  Empty schemas: {}", empty_schemas.len());
    println!("  Small schemas (≤5 props): {}", small_schemas.len());
    println!("  Large schemas (>50 props): {}", large_schemas.len());
    println!("  Likely structs: {}", struct_schemas.len());
    println!("  Likely classes: {}", class_schemas.len());

    // Analyze common struct patterns
    println!("\n🏗️ Common Struct Examples:");
    let common_structs = [
        "Vector",
        "Vector2D",
        "Rotator",
        "Transform",
        "Color",
        "LinearColor",
    ];
    for struct_name in &common_structs {
        if let Some(schema) = usmap.schemas.get_by_key(&struct_name.to_string()) {
            println!(
                "  {}: {} properties, super: '{}'",
                struct_name, schema.prop_count, schema.super_type
            );

            for (i, (_, _key, prop)) in schema.properties.iter().enumerate().take(10) {
                println!(
                    "    [{}] {} (idx:{}, array_idx:{}, array_size:{})",
                    i, prop.name, prop.schema_index, prop.array_index, prop.array_size
                );
            }
        }
    }

    // Detailed analysis of Vector struct as example
    if let Some(vector_schema) = usmap.schemas.get_by_key(&"Vector".to_string()) {
        println!("\n� Detailed Vector Struct Analysis:");
        println!("  Name: Vector");
        println!("  Super type: '{}'", vector_schema.super_type);
        println!("  Property count: {}", vector_schema.prop_count);
        println!("  Total properties: {}", vector_schema.properties.len());

        for (_, key, prop) in vector_schema.properties.iter() {
            println!("    Key: {:?}", key);
            println!(
                "    Property: {} (schema_idx:{}, array_idx:{}, array_size:{})",
                prop.name, prop.schema_index, prop.array_index, prop.array_size
            );
            println!(
                "    Property size in memory: {} bytes",
                std::mem::size_of_val(prop)
            );
        }
    }

    // Check some interesting struct patterns for the 9-byte hypothesis
    println!("\n🎯 Struct Size Analysis (9-byte hypothesis):");
    let interesting_structs = [
        "Vector",
        "Vector2D",
        "Vector4",
        "Rotator",
        "Quat",
        "Transform",
        "Color",
        "LinearColor",
        "IntPoint",
        "IntVector",
    ];

    for struct_name in &interesting_structs {
        if let Some(schema) = usmap.schemas.get_by_key(&struct_name.to_string()) {
            let total_props = schema.properties.len();
            println!(
                "  {}: {} props, estimated {} bytes ({}×9)",
                struct_name,
                total_props,
                total_props * 9,
                total_props
            );

            // Show property layout
            let mut props: Vec<_> = schema.properties.values().collect();
            props.sort_by_key(|p| p.schema_index);
            for prop in props.iter().take(5) {
                println!("    {} @ index {}", prop.name, prop.schema_index);
            }
        }
    }

    // Analyze property types and sizes
    println!("\n🔬 Property Type Analysis:");
    let test_structs = ["Vector", "Transform", "Color"];
    for struct_name in &test_structs {
        if let Some(schema) = usmap.schemas.get_by_key(&struct_name.to_string()) {
            println!("\n  {} Property Details:", struct_name);
            let mut props: Vec<_> = schema.properties.values().collect();
            props.sort_by_key(|p| p.schema_index);

            for prop in props {
                println!("    Property '{}' details:", prop.name);
                println!("      Schema index: {}", prop.schema_index);
                println!("      Array index: {}", prop.array_index);
                println!("      Array size: {}", prop.array_size);
                println!("      Property data: {:?}", prop.property_data);
            }
        }
    }

    // Look for arrays and complex properties
    println!("\n📦 Array Property Analysis:");
    let mut array_examples = Vec::new();
    let mut struct_property_examples = Vec::new();
    let mut property_type_counts = std::collections::HashMap::new();

    for (_, name, schema) in usmap.schemas.iter().take(1000) {
        for (_, _, prop) in schema.properties.iter() {
            if prop.array_size > 1 && array_examples.len() < 5 {
                array_examples.push((name, prop));
            }

            // Count property types
            match &prop.property_data {
                unreal_asset_base::unversioned::properties::UsmapPropertyData::UsmapShallowPropertyData(data) => {
                    let type_name = format!("{:?}", data.property_type);
                    *property_type_counts.entry(type_name).or_insert(0) += 1;
                }
                unreal_asset_base::unversioned::properties::UsmapPropertyData::UsmapStructPropertyData(data) => {
                    let type_name = format!("StructProperty({})", data.struct_type);
                    *property_type_counts.entry(type_name).or_insert(0) += 1;

                    if struct_property_examples.len() < 5 {
                        struct_property_examples.push((name, prop, &data.struct_type));
                    }
                }
                _ => {
                    *property_type_counts.entry("Other".to_string()).or_insert(0) += 1;
                }
            }
        }
    }

    for (schema_name, prop) in array_examples {
        println!(
            "  Schema '{}' has array property '{}' with {} elements",
            schema_name, prop.name, prop.array_size
        );
        println!(
            "    Array indices: {} to {}",
            prop.schema_index - prop.array_index,
            prop.schema_index - prop.array_index + prop.array_size as u16 - 1
        );
    }

    println!("\n🏗️ Struct Property Examples:");
    for (schema_name, prop, struct_type) in struct_property_examples {
        println!(
            "  Schema '{}' has struct property '{}' of type '{}'",
            schema_name, prop.name, struct_type
        );
    }

    println!("\n📊 Property Type Distribution (top 10):");
    let mut type_counts: Vec<_> = property_type_counts.iter().collect();
    type_counts.sort_by(|a, b| b.1.cmp(a.1));
    for (type_name, count) in type_counts.iter().take(10) {
        println!("  {}: {} occurrences", type_name, count);
    }
}

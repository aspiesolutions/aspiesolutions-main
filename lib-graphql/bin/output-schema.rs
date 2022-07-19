use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

fn main() -> std::io::Result<()> {
    println!("Building schema and writing file...");
    let schema = lib_graphql::Schema::new(lib_graphql::Query,lib_graphql::Mutation,lib_graphql::EmptySubscription::<lib_graphql::Context>::default());
    let schema_string = schema.as_schema_language().to_string();
    let schema_output_directory = std::env::var("SCHEMA_OUTPUT_DIRECTORY").expect("Environment Variable SCHEMA_OUTPUT_DIRECTORY should be defined and point to a valid file path");
    let schema_output_name = std::env::var("SCHEMA_OUTPUT_NAME").unwrap_or(String::from("schema.graphql"));
    let path = PathBuf::from(schema_output_directory);
    std::fs::create_dir_all(&path)?;
    let mut schema_file = File::create(path.join(schema_output_name))?;
    schema_file.write(schema_string.as_bytes())?;
    schema_file.flush()?;
    schema_file.sync_all()?;
    println!("Done");
    Ok(())
}
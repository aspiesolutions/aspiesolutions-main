use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    println!("Building schema");
    let schema = lib_graphql::Schema::new(lib_graphql::Query,lib_graphql::Mutation,lib_graphql::EmptySubscription::<lib_graphql::Context>::default());
    let schema_string = schema.as_schema_language().to_string();
    let schema_output_path = std::env::var("SCHEMA_OUTPUT_PATH").expect("Environment Variable SCHEMA_OUTPUT_PATH should be defined and point to a valid file path");
    println!("Writing schema file to \"{}\"",schema_output_path);
    let mut schema_file = File::create(schema_output_path)?;
    schema_file.write(schema_string.as_bytes())?;
    schema_file.flush()?;
    schema_file.sync_all()?;
    println!("File Written");
    Ok(())
}
use migration::{Migrator,MigratorTrait};
use sea_orm::{Database, ConnectOptions};
#[tokio::main]
async fn main() -> Result<(),Box<dyn std::error::Error>> {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL");
    let opts = ConnectOptions::try_from(database_url).unwrap();
    println!("Connecting");
    let db = Database::connect(opts).await?;
    println!("running migrations");
    Migrator::up(&db, None).await?;
    println!("done");
    Ok(())
}
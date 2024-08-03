use datafusion::prelude::*;
use datafusion::error::Result;

pub(crate) async fn read_csv() -> Result<()> {
    let ctx = SessionContext::new();

    // Register the "example" table
    ctx.register_csv("example", "data/sample.csv", CsvReadOptions::new()).await?;

    // Verify the schema by retrieving the DataFrame and printing the schema
    let df = ctx.table("example").await?;
    let schema = df.schema();
    println!("Schema: {:?}", schema);

    // Plan to run SQL
    let df = ctx.sql("SELECT a, MIN(b) FROM example WHERE a <= b GROUP BY a LIMIT 100").await?;

    // Output result
    df.show().await?;

    Ok(())
}

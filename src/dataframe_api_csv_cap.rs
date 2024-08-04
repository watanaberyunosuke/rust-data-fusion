use datafusion::error::Result;
use datafusion::prelude::*;

pub(crate) async fn dataframe_api_csv_cap() -> Result<()> {
    let ctx = SessionContext::new();
    let df = ctx
        .read_csv("data/capitalized_example.csv", CsvReadOptions::new())
        .await?;

    let df = df
        .filter(col("\"A\"").lt_eq(col("c")))?
        // alternatively use ident to pass in an unqualified column name directly without parsing
        .aggregate(vec![ident("A")], vec![min(col("b"))])?
        .limit(0, Some(100))?;
    df.show().await?;

    Ok(())
}

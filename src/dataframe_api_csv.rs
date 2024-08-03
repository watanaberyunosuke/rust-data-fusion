use datafusion::prelude::*;
use datafusion::error::Result;


pub(crate) async fn dataframe_api_csv() -> Result<()> {
    
    let ctx = SessionContext::new();
    let df = ctx.read_csv("data/sample.csv", CsvReadOptions::new()).await?;

    let df = df.filter(col("a").lt_eq(col("b")))?
                        .aggregate(vec![col("a")], vec![min(col("b"))])?
                        .limit(0, Some(100))?;
    
    df.show().await?;
    Ok(())

}
use datafusion::error::Result;
use datafusion::prelude::*;

pub(crate) async fn sql_api_csv() -> Result<()> {
    let ctx = SessionContext::new();
    ctx.register_csv(
        "example",
        "data/capitalized_example.csv",
        CsvReadOptions::new(),
    )
    .await?;

    let df = ctx
        .sql("SELECT \"A\", MIN(b) FROM example WHERE \"A\" <= c GROUP BY \"A\" LIMIT 100")
        .await?;

    df.show().await?;

    Ok(())
}

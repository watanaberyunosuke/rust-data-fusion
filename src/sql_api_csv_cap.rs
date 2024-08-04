use datafusion::error::Result;
use datafusion::prelude::*;

/**
 * All identifiers are effectively made lower-case in SQL,
 * so if csv file has capital letters (ex: Name)
 * you must put your column name in double quotes or the examples won’t work.
 **/

pub(crate) async fn sql_api_csv_cap() -> Result<()> {
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

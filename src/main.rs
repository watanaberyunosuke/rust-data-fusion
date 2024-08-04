mod r#dataframe_api_csv;
mod dataframe_api_csv_cap;
mod r#read_csv;
mod sql_api_csv_cap;

// Allocate memory
#[global_allocator]
static ALLOC: snmalloc_rs::SnMalloc = snmalloc_rs::SnMalloc;

#[tokio::main]
async fn main() {
    println!("read_csv result");
    read_csv::read_csv().await.expect("Error");
    println!("read_csv using dataframe api result");
    dataframe_api_csv::dataframe_api_csv().await.expect("Error");
    println!("sql api csv cap result");
    sql_api_csv_cap::sql_api_csv_cap().await.expect("Error");
    println!("dataframe api csv cap result");
    dataframe_api_csv_cap::dataframe_api_csv_cap()
        .await
        .expect("Error");
}

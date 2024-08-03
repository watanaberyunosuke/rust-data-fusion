mod r#read_csv;
mod r#dataframe_api_csv;
mod sql_api_csv;

#[tokio::main]
async fn main() {
    println!("read_csv result");
    read_csv::read_csv().await.expect("Error");
    println!("read_csv using dataframe api result");
    dataframe_api_csv::dataframe_api_csv().await.expect("Error");
    println!("sql api csv result");
    sql_api_csv::sql_api_csv().await.expect("Error");
}

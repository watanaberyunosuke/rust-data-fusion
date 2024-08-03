mod r#read_csv;

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    read_csv::read_csv().await.expect("Error");
}

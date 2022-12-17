use polars::prelude::*;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let df = df![
        "Model" => ["iphonexs","iphone 12"],
        "Company" => ["Apple","Apple"],
        "Sales" => [80,170],
        "Comment" => [None,Some("sold out")]
    ]?;

    println!("{}", &df);

    println!("df describe {}", &df.describe(None));

    df.dtypes();
    println!("df.get_column_names() {:?}", df.get_column_names());
    println!("df.get_row(0) {:?}", df.get_row(0));

    // read csv
    let csv_data= CsvReader::from_path("csv/test.csv")?
        .has_header(true)
        .finish()?;

    println!("csv_data {}", &csv_data);
    println!("csv_data head {}", &csv_data.head(Some(1)));
    println!("csv_data tail {}", &csv_data.tail(Some(1)));

    Ok(())
}

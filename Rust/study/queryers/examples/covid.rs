use anyhow::Result;
use polars::prelude::*;
use queryer::query;
use std::fmt::Debug;
use std::io::Cursor;
use std::path::PathBuf;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let mut config_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    config_path.push("examples");
    let path = config_path.into_os_string().into_string().unwrap();

    // let source = "http://raw.githubusercontent.com/owid/covid-19-data/master/public/data/latest/owid-covid-latest.csv";
    // let data = reqwest::get(source).await?.text().await?;
    // print!("csv:{}", data);

    let source = format!("file://{}/test.csv", path);

    // 使用 sql 从 URL 里获取数据
    let sql = format!(
        "select iso_code, continent, new_deaths, new_deaths_per_million \
        from {} \
        where  new_deaths > 50 and new_deaths_per_million > 0.2
        ",
        source
    );
    let df = query(sql).await?;
    println!("{:?}", df);

    Ok(())
}

// #[tokio::main]
async fn mains() -> Result<()> {
    tracing_subscriber::fmt::init();

    let url = "https://raw.githubusercontent.com/owid/covid-19-data/master/public/data/latest/owid-covid-latest.csv";
    let data = reqwest::get(url).await?.text().await?;

    // 使用 polars 直接请求
    let df = CsvReader::new(Cursor::new(data))
        .infer_schema(Some(16))
        .finish()?;

    let filtered = df.filter(&df["new_deaths"].gt(2000))?;
    println!(
        "{:?}",
        filtered.select((
            "location",
            "total_cases",
            "new_cases",
            "total_deaths",
            "new_deaths"
        ))
    );

    Ok(())
}

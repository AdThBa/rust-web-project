//use std::{fs::OpenOptions, io::Write, str::FromStr}; //removed numbuffer
use calamine::{open_workbook, Reader, Xlsx};
use clap::Parser;
use csv::Writer;
use serde::Deserialize;
use std::io::Write;

const AUDITOR_URL: &str =
    "https://www.hamiltoncountyauditor.org/download/revalue/bldginfo.xlsx";

#[derive(Parser)]
#[command(name = "building-code-tool", about = "Cincinnati building code scraper")]
struct Cli {
    #[arg(short, long, default_value = "data/bldginfo.xlsx")]
    input: String,
    #[arg(short, long, default_value = "data/building_info.csv")]
    output: String,
}

#[derive(Debug, Deserialize)]
struct Home {
    #[serde(rename = "PARCELID")]
    parcel_id: String,
    #[serde(rename = "ATTIC_SQFT")]
    attic_sqft: i64,
    #[serde(rename = "BSMT_SQFT")]
    bsmt_sqft: i64,
    #[serde(rename = "LIVE_FSQFT")]
    live_fsqft: f64,
    #[serde(rename = "SQFT")]
    sqft: i64,
    #[serde(rename = "SQFT_FLR1")]
    sqft_flr1: i64,
    #[serde(rename = "SQFT_FLR2")]
    sqft_flr2: i64,
    #[serde(rename = "SQFT_FLRH")]
    sqft_flrh: i64,
    #[serde(rename = "STORYHT")]
    storyht: f64,
    #[serde(rename = "YEARBUILT")]
    year_built: i32,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    std::fs::create_dir_all("data")?;

    // Always download. Same path we're about to read from.
    download_file(&cli.input).await?;

    println!("Reading:  {}", cli.input);
    println!("Writing:  {}", cli.output);
    //To highlight that we are actually reading the correct one (this time)

    let mut workbook: Xlsx<_> = open_workbook(&cli.input).map_err(|e| {
        anyhow::anyhow!("Cannot open '{}' ({}).", cli.input, e)
    })?;

    let sheet_name = workbook.sheet_names().first().cloned().ok_or_else(|| anyhow::anyhow!("Workbook has no sheets"))?;

    let range = workbook.worksheet_range(&sheet_name)?;

    println!("Sheet: {} ({} rows)", sheet_name, range.height());

    let file = std::fs::File::create(&cli.output)?;
    let mut writer = Writer::from_writer(file);

    for row in range.rows() {
        let record: Vec<String> = row.iter().map(|cell| cell.to_string()).collect();
        writer.write_record(&record)?;
    }

    println!("Completed. Wrote {} rows to {}", range.height(), cli.output);
    Ok(())
}

async fn download_file(path: &str) -> anyhow::Result<()> {
    println!("Fetching {}", AUDITOR_URL);

    let response = reqwest::get(AUDITOR_URL)
        .await
        .map_err(|e| anyhow::anyhow!("Request to {} failed: {}", AUDITOR_URL, e))?
        .error_for_status()
        .map_err(|e| anyhow::anyhow!("Server returned an error for {}: {}", AUDITOR_URL, e))?;

    let bytes = response.bytes().await?;
    println!("Downloaded {} bytes", bytes.len());

    let mut file = std::fs::File::create(path)
        .map_err(|e| anyhow::anyhow!("Cannot create '{}': {}", path, e))?;
    file.write_all(&bytes)?;

    Ok(())
}
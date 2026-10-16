use clap::Parser;
use serde::Deserialize;

#[derive(Parser)]
#[command(name = "building-code-tool", about = "Cincinnati building code scraper")]
struct Cli {
    #[arg(short, long)]
    source: Option<String>,
}

#[derive(Debug, Deserialize)]
struct Home {
    parcel_id: String,
    address: String,
    year_built: i32,
}

fn main() -> anyhow::Result<()> {
    //This comment in a reminder that its just parsing terminal to get the source such as hcauditor (used in test)
    let cli = Cli::parse();
    println!("Running with source: {:?}", cli.source);

    // Smoke-test the CSV reader with an inline string
    // This line should give a 'Home' of 123-45 | 123 Main St | built in 1968 
    let sample = "parcel_id,address,year_built\n123-45,123 Main St,1968\n";
    let mut reader = csv::Reader::from_reader(std::io::Cursor::new(sample));
    // Creates the Vector of homes (Honestly more like a list - This is a reminder for myself)
    let homes: Vec<Home> = reader.deserialize().collect::<Result<Vec<_>, _>>()?;

    for h in &homes {
        println!("Loaded: {} | {} | built in {}", h.parcel_id ,h.address, h.year_built);
    }

    println!("It works.");
    Ok(())
}

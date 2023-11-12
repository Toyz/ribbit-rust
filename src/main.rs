use std::io::{self, Read, Write};
use std::net::TcpStream;
use natord::compare;
use ribbit::{BgdlObject, CdnObject, FromRow, SummaryObject, VersionObject};

fn parser<T: FromRow>(reader: impl io::BufRead) -> io::Result<(i32, Vec<T>)> {
    let mut keys = Vec::new();
    let mut seqn = 0;
    let mut records = Vec::new();

    for (index, line) in reader.lines().enumerate() {
        let line = line?;

        if index == 0 {
            keys = line.split('|').map(|pair| {
                let parts: Vec<&str> = pair.split('!').collect();
                (parts[0].to_string(), parts[1].to_string())
            }).collect();
        } else if line.starts_with("#") {
            let parts: Vec<&str> = line.split('=').collect();
            seqn = parts[1].trim().parse().unwrap();
        } else {
            let row_values: Vec<&str> = line.split('|').collect();
            let mut parsed_row = Vec::new();
            for (i, val) in row_values.iter().enumerate() {
                parsed_row.push((keys[i].0.clone(), keys[i].1.clone(), val.to_string()));
            }
            records.push(T::from_row(parsed_row));
        }
    }

    Ok((seqn, records))
}

enum RibbitCommand {
    Summary,
    // Versions data
    Versions(&'static str),
    // CDN paths
    Cdns(&'static str),
    // Background Download
    Bgdl(&'static str),
}

// into impl
impl Into<String> for RibbitCommand {
    fn into(self) -> String {
        match self {
            RibbitCommand::Summary => "v2/summary".to_string(),
            RibbitCommand::Versions(product) => format!("v2/products/{}/versions", product),
            RibbitCommand::Cdns(product) => format!("v2/products/{}/cdns", product),
            RibbitCommand::Bgdl(product) => format!("v2/products/{}/bgdl", product),
        }
    }
}

impl std::fmt::Display for RibbitCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RibbitCommand::Summary => write!(f, "v2/summary"),
            RibbitCommand::Versions(product) => write!(f, "v2/products/{}/versions", product),
            RibbitCommand::Cdns(product) => write!(f, "v2/products/{}/cdns", product),
            RibbitCommand::Bgdl(product) => write!(f, "v2/products/{}/bgdl", product),
        }
    }
}

fn call(command: RibbitCommand) -> anyhow::Result<String> {
    let mut stream = TcpStream::connect("us.version.battle.net:1119")?;

    stream.write(format!("{}\r\n", <RibbitCommand as Into<String>>::into(command)).as_bytes())?;

    let mut buffer = Vec::new();
    stream.read_to_end(&mut buffer)?;

    Ok(String::from_utf8(buffer)?)
}

fn main() -> anyhow::Result<()> {
    let start = std::time::Instant::now();
    const GAME: &str = "pro";
    
    println!("---- summary -----");
    let data = call(RibbitCommand::Summary)?;
    let (seqn, mut records) = parser::<SummaryObject>(io::BufReader::new(data.as_bytes()))?;
    records.sort_by(|a, b| compare(&a.product, &b.product));
    println!("CMD: {}", RibbitCommand::Summary);
    println!("SEQN: {}", seqn);
    println!("RECORDS: {:?}", records) ;

    println!("---- CDNS -----");
    let data = call(RibbitCommand::Cdns(GAME))?;
    let (seqn, records) = parser::<CdnObject>(io::BufReader::new(data.as_bytes()))?;
    println!("CMD: {}", RibbitCommand::Cdns(GAME));
    println!("SEQN: {}", seqn);
    println!("RECORDS: {:?}", records);

    println!("---- Versions -----");
    let data = call(RibbitCommand::Versions(GAME))?;
    let (seqn, records) = parser::<VersionObject>(io::BufReader::new(data.as_bytes()))?;
    println!("CMD: {}", RibbitCommand::Versions(GAME));
    println!("SEQN: {}", seqn);
    println!("RECORDS: {:?}", records);

    println!("---- Bgdl -----");
    let data = call(RibbitCommand::Bgdl(GAME))?;
    let (seqn, records) = parser::<BgdlObject>(io::BufReader::new(data.as_bytes()))?;
    println!("CMD: {}", RibbitCommand::Bgdl(GAME));
    println!("SEQN: {}", seqn);
    println!("RECORDS: {:?}", records);

    println!("\n\nTime: {:?}", start.elapsed());

    Ok(())
}

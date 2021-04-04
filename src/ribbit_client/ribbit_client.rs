use std::net::TcpStream;
use std::io::{Read, Write};
use mailparse::*;
use std::fmt;
use std::fs::File;

pub enum Region {
    US
}

pub enum FileType {
    Version,
    BGDL,
    CDN,
}

pub fn get(region: Region, file: FileType, code: &str) -> Option<String> {
    let region_string = match region {
        Region::US => "us"
    };

    let mut stream= match TcpStream::connect(format!("{}.version.battle.net:1119", region_string)) {
        Ok(d) => d,
        Err(_) => return None
    };

    let file_type = match file {
        FileType::Version => "versions",
        FileType::BGDL => "bgdl",
        FileType::CDN => "cdns",
    };

    let msg = format!("v1/products/{}/{}\r\n", code, file_type);

    let _ =  stream.write(msg.as_bytes());

    let mut buffer = Vec::new();
    let _ = stream.read_to_end(&mut buffer);

    let parsed = match parse_mail(&mut buffer)  {
        Ok(x) => x,
        Err(_) => return None
    };

    if parsed.subparts.is_empty() {
        return None
    }

    match parsed.subparts[0].get_body() {
        Ok(x) => Some(x),
        Err(_) => None,
    }
}
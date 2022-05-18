use base64;
use clap::Parser;
use http::StatusCode;
use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue, AUTHORIZATION};
use std::fs::File;
use std::path::Path;
use std::str;

// curl \
//  -H 'Authorization: OAuth abc' \
// 	-H 'X-entity-name: test.blob' \
// 	-H 'offset: 0' \
// 	-H 'X-entity-lenght: 1' \
// 	-H 'X-Entity-type: application/octet-stream' \
// 	-H 'x-custom-metadata: blah' \
// 	--data-binary '@test.blob' \
// 	https:jsonplaceholder.typicode.com/posts

const URI: &str = "https://jsonplaceholder.typicode.com/posts";

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    // Path of binary data to load
    #[clap(short, long)]
    filename: String,
    header_data: String,
}

fn construct_headers(header_data: &String) -> HeaderMap {
    let mut headers = HeaderMap::new();

    headers.insert(AUTHORIZATION, HeaderValue::from_static("OAuth abc"));
    headers.insert(
        HeaderName::from_lowercase(b"x-entity-name").unwrap(),
        HeaderValue::from_static("test.blob"),
    );
    headers.insert(
        HeaderName::from_lowercase(b"offset").unwrap(),
        HeaderValue::from_static("0"),
    );
    headers.insert(
        HeaderName::from_lowercase(b"x-entity-lenght").unwrap(),
        HeaderValue::from_static("1"),
    );
    headers.insert(
        HeaderName::from_lowercase(b"x-entity-type").unwrap(),
        HeaderValue::from_static("application/octet-stream"),
    );
    headers.insert(
        HeaderName::from_lowercase(b"x-custom-metadata").unwrap(),
        HeaderValue::from_str(header_data.as_str()).unwrap(),
    );
    println!("Header map created:");
    for (key, value) in headers.iter() {
        println!("\t{:?}: {:?}", key, value);
    }
    headers
}

fn upload(path: &Path, header_data: &String) -> Result<StatusCode, Box<dyn std::error::Error>> {
    let filename: &str = path.to_str().unwrap();
    println!("POST request: uploading {}", filename);
    let file = File::open(filename)?;
    let client = Client::new();
    let res = client
        .post(URI)
        .headers(construct_headers(header_data))
        .body(file)
        .send()?;
    Ok(res.status())
}

fn main() {
    let args = Args::parse();
    let path: &Path = Path::new(&args.filename);
    match upload(path, &args.header_data) {
        Ok(StatusCode::OK) => println!("Successfully posted data."),
        Ok(StatusCode::PAYLOAD_TOO_LARGE) => {
            println!("Payload is too large.");
        }
        Ok(s) => println!("Received response status: {:?}", s),
        Err(e) => println!("Undexpected error: {}", e),
    };
}

//! Renders several ZPL files in one multipart request and handles API errors.
//!
//! ```sh
//! CARETLY_API_KEY=zpl_test_... cargo run --example render_files -- a.zpl b.zpl
//! ```

use caretly::{Client, Error, RenderOptions};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let paths: Vec<String> = std::env::args().skip(1).collect();
    let files = if paths.is_empty() {
        vec![
            (
                "first.zpl".to_owned(),
                b"^XA^FO50,50^A0N,40,40^FDFirst^FS^XZ".to_vec(),
            ),
            (
                "second.zpl".to_owned(),
                b"^XA^FO50,50^A0N,40,40^FDSecond^FS^XZ".to_vec(),
            ),
        ]
    } else {
        paths
            .iter()
            .map(|p| Ok((p.clone(), std::fs::read(p)?)))
            .collect::<std::io::Result<_>>()?
    };

    let client = Client::new()?;
    match client.render_files(files, RenderOptions::new()).await {
        Ok(result) => {
            result.save("labels.pdf")?;
            println!("labels.pdf: pages per file {:?}", result.file_page_counts);
        }
        Err(Error::BadRequest(e)) => eprintln!("the ZPL was rejected: {}", e.message),
        Err(Error::Authentication(_)) => eprintln!("set CARETLY_API_KEY to a valid key"),
        Err(e) => return Err(e.into()),
    }
    Ok(())
}

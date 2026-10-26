//! Renders a label with the blocking client: no async runtime needed.
//!
//! ```sh
//! CARETLY_API_KEY=zpl_test_... cargo run --example blocking --features blocking
//! ```

use std::time::Duration;

use caretly::blocking::Client;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::builder()
        .timeout(Duration::from_secs(10))
        .max_retries(3)
        .build()?;
    println!("healthy: {}", client.health()?);

    let png = client.render_png("^XA^FO50,50^A0N,40,40^FDHello^FS^XZ", 1)?;
    png.save("label.png")?;
    println!("label.png: {} bytes", png.bytes.len());
    Ok(())
}

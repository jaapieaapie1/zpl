//! Renders a label to PDF and PNG with the async client.
//!
//! ```sh
//! CARETLY_API_KEY=zpl_test_... cargo run --example render_pdf
//! ```

use caretly::{Client, Dpmm, Format, RenderOptions};

const LABEL: &str =
    "^XA^FO50,50^A0N,40,40^FDHello from Caretly^FS^FO50,120^BCN,80,Y,N,N^FD123456^FS^XZ";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new()?;

    let pdf = client.render_pdf(LABEL).await?;
    pdf.save("label.pdf")?;
    println!(
        "label.pdf: {} page(s), {} bytes",
        pdf.page_count,
        pdf.bytes.len()
    );
    if let Some(billed) = pdf.labels_billed {
        println!("labels billed: {billed}");
    }

    let options = RenderOptions::new()
        .format(Format::Png)
        .page(1)
        .dpmm(Dpmm::D12)
        .timings(true);
    let png = client.render(LABEL, options).await?;
    png.save("label.png")?;
    println!(
        "label.png: server timing {}",
        png.server_timing.as_deref().unwrap_or("-")
    );
    Ok(())
}

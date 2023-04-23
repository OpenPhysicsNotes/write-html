use std::error::Error;

use write_html::{html, Doctype, DefaultMeta, ToHtmlString};

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello There!");

    let html_page = html!(
        (Doctype)
        html lang="en" {
            head {
                (DefaultMeta)
                title { "Website!" }
            }
            body {
                h1 #some-id { "H1" }
                h2 { "H2" }
                h3 { "H3" }
                p { "Paragraph" }
                ol {
                    li { "Item 1" }
                    li { "Item 2" }
                    li style="color: red" { "Item 3" }
                }
                footer;
            }
        }
    ).to_html_string()?;

    {
        use std::io::Write;
        let file = "test.html";
        let mut file = std::fs::File::create(file)?;
        file.write_all(html_page.as_bytes())?;
    }

    Ok(())
}
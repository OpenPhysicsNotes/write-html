# write-html
 A simple and fast html generator

> :warning: This project is still in development and is not ready for production use, it is also not so fast, yet.

# Example
```rust
use write_html::*;

let page = html!(
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
).to_html_string().unwrap();
```
use std::{error::Error, fmt::Write};

use write_html::{HtmlEnv, DefaultMeta, html, Doctype};

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello There!");

    let mut page = String::new();
    page.reserve(1000);

    page.with(html!(
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
            }
        }
    ))?;

    struct MyWriter<'a>(&'a mut String);
    impl Write for MyWriter<'_> {
        fn write_str(&mut self, s: &str) -> std::fmt::Result {
            self.0.write_str(s).unwrap();
            Ok(())
        }
    }

    let mut page2 = MyWriter(&mut page);

    let begin = std::time::Instant::now();
    for i in 0..1000000*1 {
        page2.0.clear();
        page2.with(html!(
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
                }
            }
        ))?;
    }
    //let page2 = include_str!("page.html");
    let mut c = 0;
    for _ in 0..1000000*0 {
        page.clear();
        for _ in 0..372 {
            c += 1;
            let c = c % 26 + 65;
            let c = c as u8 as char;
            page.write_char(c).unwrap();
        }
    }
    let end = std::time::Instant::now();
    println!("Time: {:?}", end - begin);
    println!("len: {}", page.len());

    {
        use std::io::Write;
        let file = "test.html";
        let mut file = std::fs::File::create(file)?;
        file.write_all(page.as_bytes())?;
    }

    Ok(())
}
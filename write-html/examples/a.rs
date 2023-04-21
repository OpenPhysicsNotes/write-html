use std::error::Error;

use write_html::{HtmlEnv, tags, Empty, AsHtml, DefaultMeta};



fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello There!");

    let mut page = String::new();
    //page.reserve(1000);

    {
        let mut page = String::new();
        page.html5_default_meta()?;
        println!("{}", page);
    }

    page.doctype();
    page.html_root("en")?
        .with(tags::head(Empty, Empty)
            .child(DefaultMeta)
            .child(tags::title(Empty, "Website!".as_html()))
        )?
        .with(tags::body(Empty, Empty)
            .child(tags::h1([("id", "h1")], "H1".as_html()))
            .child(tags::h2(Empty, "H2".as_html()))
            .child(tags::h3(Empty, "H3".as_html()))
            .child(tags::p(Empty, "Paragraph".as_html()))
            .child(tags::ol(
                Empty,
                [
                    tags::li(Empty, "Item 1".as_html()),
                    tags::li(Empty, "Item 2".as_html()),
                ],
            ))
            .child(tags::ol(Empty, Empty)
                .attr("style", "color: red")
                .child(tags::li(Empty, "Item 1".as_html()))
                .child(tags::li(Empty, "Item 2".as_html()))
                .child(tags::li(Empty, "Item 3".as_html()))
            )
    )?;

    {
        use std::io::Write;
        let file = "test.html";
        let mut file = std::fs::File::create(file)?;
        file.write_all(page.as_bytes())?;
    }

    Ok(())
}
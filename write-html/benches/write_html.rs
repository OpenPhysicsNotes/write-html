
use criterion::{criterion_group, criterion_main, Criterion};
use write_html::*;
use std::fmt::Write;

pub fn benchmark(c: &mut Criterion) {
    c.bench_function("sample html", |b| b.iter(|| sample_html().unwrap()));
}

fn sample_html() -> Result<String, Box<dyn std::error::Error>> {
    
    let mut page = String::new();
    //page.reserve(1000);

    struct Links;
    impl Html for Links {
        fn write_html(self, env: &mut impl HtmlEnv) -> std::fmt::Result {
            struct Href(i32);
            impl AttributeValue for Href {
                fn write_attribute_value(self, w: &mut impl Write) -> std::fmt::Result {
                    w.write_fmt(format_args!("href='/page_{}.html'", self.0))
                }
            }
            impl Html for Href {
                fn write_html(self, env: &mut impl HtmlEnv) -> std::fmt::Result {
                    env.write_fmt(format_args!("Page {}", self.0))
                }
            }
            for i in 0..2 {
                env.with(tags::a(Empty, Empty)
                    .attr("href", Href(i))
                    .child(Href(i))
                )?;
            }
            Ok(())
        }
    }

    page.with(html!(
        (Doctype)
        html lang="en" {
            head {
                (DefaultMeta)
                title { "Website!" }
            }
            body {
                h1 { "It's a website!" }
                //li {
                //    ((0..2).map(|i| html!(
                //        a href=(format!("/page_{}.html", i)) {
                //            (format!("Page {}", i).as_html_text())
                //        }
                //    )))
                //}
                li {
                    (Links)
                }
                figure {
                    img src="img.jpg" alt="Awesome image" {} // TODO accept ; instead of {} as well for empty tags
                    figcaption { "Awesome image" }
                }
                footer {
                    "Last modified"
                    time { "2021-04-12" }
                }
                // TODO comment
            }
        }
    ))?;

    Ok(page)
}

criterion_group!(benches, benchmark);
criterion_main!(benches);

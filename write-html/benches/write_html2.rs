
use criterion::{criterion_group, criterion_main, Criterion};
use write_html::*;


pub fn benchmark(c: &mut Criterion) {
    let mut page = String::new();
    c.bench_function("sample html", |b| b.iter(|| sample_html(&mut page).unwrap()));
}

fn sample_html(page: &mut String) -> Result<(), Box<dyn std::error::Error>> {
    
    page.clear();
    //page.reserve(1000);

    page.write_html(html!(
        (Doctype)
        html lang="en" {
            head {
                (DefaultMeta)
                title { "Website!" }
            }
            body {
                h1 { "It's a website!" }
                li {
                    ((0..2).map(|i| html!(
                        a href=(format!("/page_{}.html", i)) {
                            (format!("Page {}", i).as_html_text())
                        }
                    )))
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

    Ok(())
}

criterion_group!(benches, benchmark);
criterion_main!(benches);


/*!
This crate provides a way to write HTML with as little overhead as possible.

> This crate is still in development, and the API is not stable.

# Example
```
use write_html::*;
use std::fmt::Write;

fn my_f() -> std::fmt::Result {
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
    ).to_html_string()?;

    Ok(())
}

my_f().unwrap();
```
*/

#![warn(missing_docs)]

use std::fmt::Write;

mod attributes;
mod tag;
mod html_trait;

pub use attributes::*;
use escapes::HtmlEscaper;
pub use tag::*;
pub use html_trait::*;

pub mod escapes;
pub mod tags;

pub use write_html_macro::html;

// TODO move
/// Represents something that is "empty"
pub struct Empty;

// TODO move
/// Represents a sum of two types.
pub struct Sum<A, B>(pub A, pub B);


/// Represents an environment that can write HTML.
///
/// This trait is implemented for everything that implements [`Write`], for example [`String`].
pub trait HtmlEnv: Write + Sized {

    /// Writes an [`Html`] into the environment.
    ///
    /// # Example
    /// ```
    /// use write_html::{HtmlEnv, Html, AsHtml};
    /// 
    /// let mut s = String::new();
    /// s.write_html("Hello, world!".as_html()).unwrap();
    /// assert_eq!(s, "Hello, world!");
    /// ```
    fn write_html(&mut self, html: impl Html) -> Result<&mut Self, std::fmt::Error> {
        html.write_html(self)?;
        Ok(self)
    }

    /// Writes the HTML5 doctype.
    ///
    /// # Example
    /// ```
    /// use write_html::HtmlEnv;
    /// 
    /// let mut s = String::new();
    /// s.doctype();
    /// assert_eq!(s, "<!DOCTYPE html>");
    /// ```
    fn doctype(&mut self) {
        write!(self, "<!DOCTYPE html>").unwrap();
    }

    /// Lets you write text into the HTML document, escaping it as necessary.
    ///
    /// # Example
    /// ```
    /// use write_html::HtmlEnv;
    /// use std::fmt::Write;
    /// 
    /// let mut s = String::new();
    /// s.write_html_text().write_str("Hello, <world>").unwrap();
    /// assert_eq!(s, "Hello, &lt;world&gt;");
    /// ```
    fn write_html_text<'s>(&'s mut self) -> HtmlEscaper<'s, Self> {
        HtmlEscaper::new(self)
    }

    /// Returns a tag opening, which lets you write attributes and inner HTML.
    ///
    /// # Arguments
    /// * `tag` - The tag name.
    /// * `compactability` - Whether the tag can be compacted.
    ///
    /// # Example
    /// ```
    /// use write_html::{HtmlEnv, Compactability};
    /// use std::fmt::Write;
    /// 
    /// let mut s = String::new();
    /// s.open_tag("h1", Compactability::No).unwrap()
    ///     .with_attr("id", "my-id").unwrap()
    ///     .inner_html().unwrap()
    ///     .write_str("Hello, world!").unwrap();
    /// assert_eq!(s, "<h1 id=\"my-id\">Hello, world!</h1>");
    /// ```
    fn open_tag<'s, 't>(
        &'s mut self,
        tag: &'t str, // TODO non-static lifetime
        compactability: Compactability
    ) -> Result<TagOpening<'s, 't, Self>, std::fmt::Error> {
        TagOpening::<'s, 't, Self>::new(tag, self, compactability)
    }
}

impl<W: Write> HtmlEnv for W {}

/// Writes the default HTML5 `<meta>` tags.
pub struct DefaultMeta;
impl Html for DefaultMeta {
    fn write_html(self, env: &mut impl HtmlEnv) -> std::fmt::Result {
        env
            .write_html(tags::meta(
                [
                    ("http-equiv", "X-UA-Compatible"),
                    ("content", "ie=edge")
                ],
                Empty,
            ))?
            .write_html(tags::meta(
                [("charset", "UTF-8")],
                Empty,
            ))?
            .write_html(tags::meta(
                [
                    ("name", "viewport"),
                    ("content", "width=device-width, initial-scale=1.0")
                ],
                Empty,
            )).map(|_| ())
    }
}

/// Writes the HTML5 doctype.
pub struct Doctype;
impl Html for Doctype {
    fn write_html(self, env: &mut impl HtmlEnv) -> std::fmt::Result {
        env.doctype();
        Ok(())
    }
}
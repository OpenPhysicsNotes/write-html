
/*!
This crate provides a way to write HTML with as little overhead as possible.

> This crate is still in development, and the API is not stable.

# Example
```
use write_html::*;
use std::fmt::Write;

fn my_f() -> std::fmt::Result {
    let mut page = String::new();
    //page.reserve(1000);

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
    /// s.with("Hello, world!".as_html()).unwrap();
    /// assert_eq!(s, "Hello, world!");
    /// ```
    fn with(&mut self, html: impl Html) -> Result<&mut Self, std::fmt::Error> {
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
    /// s.text().write_str("Hello, <world>").unwrap();
    /// assert_eq!(s, "Hello, &lt;world&gt;");
    /// ```
    fn text<'s>(&'s mut self) -> HtmlEscaper<'s, Self> {
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
    /// s.tag("h1", Compactability::No).unwrap()
    ///     .with_attr("id", "my-id").unwrap()
    ///     .inner_html().unwrap()
    ///     .write_str("Hello, world!").unwrap();
    /// assert_eq!(s, "<h1 id=\"my-id\">Hello, world!</h1>");
    /// ```
    fn tag<'s, 't>(
        &'s mut self,
        tag: &'t str, // TODO non-static lifetime
        compactability: Compactability
    ) -> Result<TagOpening<'s, 't, Self>, std::fmt::Error> {
        TagOpening::<'s, 't, Self>::new(tag, self, compactability)
    }

    /// Adds a new root `<html>` node to the HTML document.
    ///
    /// See [`HtmlEnv::tag`] for more information.
    fn html_root<'s>(
        &'s mut self,
        lang: &str
    ) -> Result<InsideTagHtml<'s, 'static, Self>, std::fmt::Error> {
        self.tag("html", Compactability::No)?
            .with_attr("lang", lang)?
            .inner_html()
    }

    /// Adds default `<meta>` tags to the HTML document.
    ///
    /// # Example
    /// ```
    /// use write_html::HtmlEnv;
    /// 
    /// let mut s = String::new();
    /// s.html5_default_meta().unwrap();
    /// assert_eq!(s, "<meta http-equiv=\"X-UA-Compatible\" content=\"ie=edge\"><meta charset=\"UTF-8\"><meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">");
    fn html5_default_meta(&mut self) -> Result<&mut Self, std::fmt::Error> {
        self
            .with(tags::meta(
                [
                    ("http-equiv", "X-UA-Compatible"),
                    ("content", "ie=edge")
                ],
                Empty,
            ))?
            .with(tags::meta(
                //[("charset", "utf-8")],
                [("charset", "UTF-8")],
                Empty,
            ))?
            .with(tags::meta(
                [
                    ("name", "viewport"),
                    ("content", "width=device-width, initial-scale=1.0")
                ],
                Empty,
            ))?;

        Ok(self)
    }
}

impl<W: Write> HtmlEnv for W {}

/// Writes the default HTML5 `<meta>` tags.
pub struct DefaultMeta;
impl Html for DefaultMeta {
    fn write_html(self, env: &mut impl HtmlEnv) -> std::fmt::Result {
        env.html5_default_meta().map(|_| ())
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
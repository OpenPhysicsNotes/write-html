use std::fmt::{Write};

use crate::{HtmlEnv, Empty, Sum};


/// Represents a content that can be written to a `Write` as HTML.
pub trait Html {
    /// Tells whether `self` is a unit value, meaning that it is not written.
    fn is_unit(&self) -> bool {
        false
    }
    /// Writes the HTML representation of `self` to `w`.
    ///
    /// # Arguments
    /// * `env` - The environment to write to.
    ///
    /// # Example
    /// ```
    /// use write_html::{Html, HtmlEnv, AsHtml};
    /// use std::fmt::Write;
    /// 
    /// let mut s = String::new();
    /// "<h1>H1</h1>".as_html().write_html(&mut s).unwrap();
    /// assert_eq!(s, "<h1>H1</h1>");
    /// ```
    fn write_html(self, env: &mut impl HtmlEnv) -> std::fmt::Result;
}

impl Html for Empty {
    fn is_unit(&self) -> bool {
        true
    }
    fn write_html(self, _env: &mut impl HtmlEnv) -> std::fmt::Result {
        Ok(())
    }
}

impl<A: Html, B: Html> Html for Sum<A, B> {
    fn write_html(self, env: &mut impl HtmlEnv) -> std::fmt::Result {
        self.0.write_html(env)?;
        self.1.write_html(env)?;
        Ok(())
    }
}

impl<I: IntoIterator> Html for I
where
    I::Item: Html,
{
    fn write_html(self, env: &mut impl HtmlEnv) -> std::fmt::Result {
        for h in self {
            h.write_html(env)?;
        }
        Ok(())
    }
}

/// Html string
///
/// TODO better doc
pub struct HtmlStr<S>(pub S);

impl<S> Html for HtmlStr<S>
where
    S: AsRef<str>,
{
    fn write_html(self, env: &mut impl HtmlEnv) -> std::fmt::Result {
        env.write_str(self.0.as_ref())
    }
}

/// Html text string
///
/// TODO better doc
pub struct HtmlTextStr<S>(pub S);

impl<S> Html for HtmlTextStr<S>
where
    S: AsRef<str>,
{
    fn write_html(self, env: &mut impl HtmlEnv) -> std::fmt::Result {
        env.write_html_text().write_str(self.0.as_ref())
    }
}

/// Something that can be converted into HTML.
///
/// TODO better doc
pub trait AsHtml {
    /// The HTML type.
    type Html: Html;
    /// The HTML text type.
    type HtmlText: Html;
    /// Converts `self` into HTML.
    fn as_html(self) -> Self::Html;
    /// Converts `self` into HTML text.
    fn as_html_text(self) -> Self::HtmlText;
}

impl<'a> AsHtml for &'a str {
    type Html = HtmlStr<&'a str>;
    type HtmlText = HtmlTextStr<&'a str>;
    fn as_html(self) -> Self::Html {
        HtmlStr(self)
    }
    fn as_html_text(self) -> Self::HtmlText {
        HtmlTextStr(self)
    }
}

impl AsHtml for String {
    type Html = HtmlStr<String>;
    type HtmlText = HtmlTextStr<String>;
    fn as_html(self) -> Self::Html {
        HtmlStr(self)
    }
    fn as_html_text(self) -> Self::HtmlText {
        HtmlTextStr(self)
    }
}

/// Something that can be converted into an HTML string.
pub trait ToHtmlString {
    /// Converts `self` into an HTML string.
    fn to_html_string(self) -> Result<String, std::fmt::Error>;
}

impl<H: Html> ToHtmlString for H {
    fn to_html_string(self) -> Result<String, std::fmt::Error> {
        let mut s = String::new();
        self.write_html(&mut s)?;
        Ok(s)
    }
}
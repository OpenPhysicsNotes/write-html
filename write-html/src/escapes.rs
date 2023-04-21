/*!
Provides `StringEscaper` and `HtmlEscaper` to escape strings for use in string literals and HTML elements respectively.
*/

use std::fmt::Write;



/// Escapes a string for the inside of a string literal.
///
/// # Examples
/// ```
/// use std::fmt::Write;
/// use write_html::escapes::StringEscaper;
/// 
/// let mut s = String::new();
/// let mut escaper = StringEscaper::new(&mut s);
/// escaper.write_str("Hello, \"world\"").unwrap();
/// assert_eq!(s, "Hello, \\\"world\\\"");
/// ```
pub struct StringEscaper<'a, W: Write> {
    w: &'a mut W,
}

impl<'a, W: Write> StringEscaper<'a, W> {
    /// Creates a new `StringEscaper` that will write to `w`.
    ///
    /// # Arguments
    /// * `w` - The `Write` to write to.
    pub fn new(w: &'a mut W) -> Self {
        Self { w }
    }
}

impl<'a, W: Write> Write for StringEscaper<'a, W> {
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        for c in s.chars() {
            match c {
                '"' => write!(self.w, "\\\"")?,
                '\'' => write!(self.w, "\\\'")?,
                '\\' => write!(self.w, "\\\\")?,
                '\r' => write!(self.w, "\\r")?,
                '\n' => write!(self.w, "\\n")?,
                '\t' => write!(self.w, "\\t")?,
                // TODO ...
                _ => write!(self.w, "{}", c)?,
            };
        }

        Ok(())
    }
}

/// Escapes a string for the inside of an HTML element.
///
/// # Examples
/// ```
/// use std::fmt::Write;
/// use write_html::escapes::HtmlEscaper;
/// 
/// let mut s = String::new();
/// let mut escaper = HtmlEscaper::new(&mut s);
/// escaper.write_str("<h1>H1</h1>").unwrap();
/// assert_eq!(s, "&lt;h1&gt;H1&lt;/h1&gt;");
/// ```
pub struct HtmlEscaper<'a, W: Write> {
    w: &'a mut W,
}

impl<'a, W: Write> HtmlEscaper<'a, W> {
    /// Creates a new `HtmlEscaper` that will write to `w`.
    ///
    /// # Arguments
    /// * `w` - The `Write` to write to.
    pub fn new(w: &'a mut W) -> Self {
        Self { w }
    }
}

impl<'a, W: Write> Write for HtmlEscaper<'a, W> {
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        for c in s.chars() {
            match c {
                '<' => write!(self.w, "&lt;")?,
                '>' => write!(self.w, "&gt;")?,
                '&' => write!(self.w, "&amp;")?,
                '\n' => write!(self.w, "<br>")?,
                // TODO ...
                _ => write!(self.w, "{}", c)?,
            };
        }

        Ok(())
    }
}
use std::fmt::Write;

use crate::{AttributeName, AttributeValue, escapes::StringEscaper, Attributes};

struct TagOpeningData<'a, 't, W: Write> {
    tag: &'t str,
    w: &'a mut W,
    compactability: Compactability,
}


/// Represents a tag that is being opened.
pub struct TagOpening<'a, 't, W: Write> {
    data: Option<TagOpeningData<'a, 't, W>>,
}

impl<'a, 't, W: Write> TagOpening<'a, 't, W> {
    /// Creates a new `TagOpening` that will write to `w`.
    pub fn new(tag: &'t str, w: &'a mut W, compactability: Compactability) -> Result<Self, std::fmt::Error> {
        w.write_str("<")?;
        w.write_str(tag)?;
        Ok(Self { data: Some(TagOpeningData { tag, w, compactability }) })
    }

    /// Adds an attribute to the tag.
    ///
    /// # Arguments
    /// * `name` - The name of the attribute.
    /// * `value` - The value of the attribute.
    pub fn attr<'s>(
        &'s mut self,
        name: impl AttributeName,
        value: impl AttributeValue
    ) -> Result<&'s mut Self, std::fmt::Error> {
        let data = self.data.as_mut().unwrap();
        data.w.write_str(" ").unwrap();

        assert!(name.is_valid_attribute_name());
        name.write_attribute_name(data.w)?;

        if value.is_unit() {
            return Ok(self);
        } else {
            data.w.write_str("=\"")?;
            value.write_attribute_value(&mut StringEscaper::new(data.w))?;
            data.w.write_str("\"")?;
        }

        Ok(self)
    }

    /// Adds an attribute to the tag.
    ///
    /// See [`attr`] for more information.
    pub fn with_attr(mut self, name: impl AttributeName, value: impl AttributeValue) -> Result<Self, std::fmt::Error> {
        self.attr(name, value)?;

        Ok(self)
    }

    /// Adds multiple attributes to the tag.
    pub fn with_attributes(mut self, attributes: impl Attributes) -> Result<Self, std::fmt::Error> {
        attributes.write_attributes(&mut self)?;
        Ok(self)
    }

    /// Finishes the opening of the tag and returns a [`InsideTagHtml`] that can be used to write the contents of the tag.
    pub fn inner_html(mut self) -> Result<InsideTagHtml<'a, 't, W>, std::fmt::Error> {
        // get the data out of self
        let data = self.data.take().unwrap();
        data.w.write_str(">")?;
        Ok(InsideTagHtml { tag: data.tag, w: data.w })
    }
}

impl<'a, 't, W: Write> Drop for TagOpening<'a, 't, W> {
    fn drop(&mut self) {
        if let Some(data) = self.data.take() {
            if let Compactability::Yes { final_slash } = data.compactability {
                let _ = data.w.write_str(if final_slash { "/>" } else { ">" });
            } else {
                let _ = data.w.write_fmt(format_args!("></{}>", data.tag));
            }
        }
    }
}

/// Represents the environment inside a tag.
pub struct InsideTagHtml<'a, 't, W: Write> {
    tag: &'t str,
    w: &'a mut W,
}

impl<'a, 't, W: Write> InsideTagHtml<'a, 't, W> {
    // TODO
}

impl<'a, 't, W: Write> Drop for InsideTagHtml<'a, 't, W> {
    fn drop(&mut self) {
        let _ = self.w.write_str("</");
        let _ = self.w.write_str(self.tag);
        let _ = self.w.write_str(">");
    }
}

impl<'a, 't, W: Write> Write for InsideTagHtml<'a, 't, W> {
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        self.w.write_str(s)
    }
}

/// Represents the compactability of a tag.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Compactability {
    /// The tag is not compactable.
    ///
    /// This means that the tag will always be written as `<tag></tag>`.
    No,

    /// The tag is compactable.
    ///
    /// This means that the tag will be written as `<tag/>` if it has no contents.
    ///
    /// The `final_slash` parameter determines whether the tag will be written as `<tag>` or `<tag/>`.
    Yes {
        /// Wether a compacted tag will be written as `<tag/>` or `<tag>`.
        final_slash: bool
    },
}

impl Compactability {
    /// Returns wether the tag is compactable.
    pub fn is_compactable(&self) -> bool {
        match self {
            Compactability::No => false,
            Compactability::Yes { .. } => true,
        }
    }
}

impl From<bool> for Compactability {
    fn from(b: bool) -> Self {
        if b {
            Compactability::Yes { final_slash: true }
        } else {
            Compactability::No
        }
    }
}

impl From<Compactability> for bool {
    fn from(c: Compactability) -> Self {
        match c {
            Compactability::No => false,
            Compactability::Yes { .. } => true,
        }
    }
}
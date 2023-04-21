use std::fmt::Write;

use crate::{TagOpening, Empty, Sum};


/// Represents a list of attributes.
pub trait Attributes {
    /// Writes the attributes to `w`.
    ///
    /// # Arguments
    /// * `w` - The tag opening to write to.
    fn write_attributes<'a, 't, W: Write>(self, w: &mut TagOpening<'a, 't, W>) -> std::fmt::Result;
} // TODO TagOpening into trait and rename to TagOpening struct into something else

impl<A: Attributes, B: Attributes> Attributes for Sum<A, B> {
    fn write_attributes<'a, 't, W: Write>(self, w: &mut TagOpening<'a, 't, W>) -> std::fmt::Result {
        self.0.write_attributes(w)?;
        self.1.write_attributes(w)?;
        Ok(())
    }
}

impl<I: IntoIterator<Item = (Name, Value)>, Name, Value> Attributes for I
where
    Name: AttributeName,
    Value: AttributeValue,
{
    fn write_attributes<'a, 't, W: Write>(self, w: &mut TagOpening<'a, 't, W>) -> std::fmt::Result {
        for (n, v) in self {
            w.attr(n, v)?;
        }
        Ok(())
    }
}

impl Attributes for Empty {
    fn write_attributes<'a, 't, W: Write>(self, _w: &mut TagOpening<'a, 't, W>) -> std::fmt::Result {
        Ok(())
    }
}


/// Tells whether the given string is a valid attribute name.
pub fn is_valid_attribute_name(name: &str) -> bool {
    if name.is_empty() {
        return false;
    }

    if name.trim() != name {
        return false;
    }

    let mut chars = name.chars();
    let first_char = chars.next().unwrap();

    if !first_char.is_ascii_alphabetic() {
        return false;
    }

    for c in chars {
        if !c.is_ascii_alphanumeric() && c != '-' && c != '_' {
            return false;
        }
    }

    true
}

/// Represents a name of an attribute.
pub trait AttributeName {
    /// Tells whether the attribute name is valid.
    fn is_valid_attribute_name(&self) -> bool;
    /// Writes the attribute name to `w`.
    ///
    /// # Arguments
    /// * `w` - The writer to write to.
    fn write_attribute_name(self, w: &mut impl Write) -> std::fmt::Result;
}

impl AttributeName for &str {
    fn is_valid_attribute_name(&self) -> bool {
        is_valid_attribute_name(self)
    }
    fn write_attribute_name(self, w: &mut impl Write) -> std::fmt::Result {
        w.write_str(self)
    }
}

impl AttributeName for &&str {
    fn is_valid_attribute_name(&self) -> bool {
        is_valid_attribute_name(self)
    }
    fn write_attribute_name(self, w: &mut impl Write) -> std::fmt::Result {
        w.write_str(*self)
    }
}

impl AttributeName for String {
    fn is_valid_attribute_name(&self) -> bool {
        is_valid_attribute_name(&self)
    }
    fn write_attribute_name(self, w: &mut impl Write) -> std::fmt::Result {
        w.write_str(&self)
    }
}

impl AttributeName for &String {
    fn is_valid_attribute_name(&self) -> bool {
        is_valid_attribute_name(self)
    }
    fn write_attribute_name(self, w: &mut impl Write) -> std::fmt::Result {
        w.write_str(self)
    }
}

/// Represents a value of an attribute.
pub trait AttributeValue {
    /// Tells whether the attribute value is a unit value, meaning that it is not written.
    fn is_unit(&self) -> bool {
        false
    }
    /// Writes the attribute value to `w`.
    ///
    /// # Arguments
    /// * `w` - The writer to write to.
    fn write_attribute_value(self, w: &mut impl Write) -> std::fmt::Result;
}

impl AttributeValue for &str {
    fn is_unit(&self) -> bool {
        false
    }
    fn write_attribute_value(self, w: &mut impl Write) -> std::fmt::Result {
        w.write_str(self)
    }
}

impl AttributeValue for &&str {
    fn is_unit(&self) -> bool {
        false
    }
    fn write_attribute_value(self, w: &mut impl Write) -> std::fmt::Result {
        w.write_str(*self)
    }
}

impl AttributeValue for String {
    fn is_unit(&self) -> bool {
        false
    }
    fn write_attribute_value(self, w: &mut impl Write) -> std::fmt::Result {
        w.write_str(&self)
    }
}

impl AttributeValue for &String {
    fn is_unit(&self) -> bool {
        false
    }
    fn write_attribute_value(self, w: &mut impl Write) -> std::fmt::Result {
        w.write_str(self)
    }
}

impl AttributeValue for () {
    fn is_unit(&self) -> bool {
        true
    }
    fn write_attribute_value(self, _w: &mut impl Write) -> std::fmt::Result {
        Ok(())
    }
}

/*impl<F: FnOnce(&mut dyn Write) -> std::fmt::Result> AttributeValue for F {
    fn is_unit(&self) -> bool {
        false
    }
    fn write_attribute_value(self, w: &mut impl Write) -> std::fmt::Result {
        self(w)
    }
}*/
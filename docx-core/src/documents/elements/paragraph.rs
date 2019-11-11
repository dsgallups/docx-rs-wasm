use super::{ParagraphProperty, ParagraphStyle, Run};
use crate::documents::BuildXML;
use crate::types::*;
use crate::xml_builder::*;

#[derive(Debug)]
pub struct Paragraph {
    runs: Vec<Run>,
    property: ParagraphProperty,
    style: ParagraphStyle,
}

impl Default for Paragraph {
    fn default() -> Self {
        let s: Option<&str> = None;
        Self {
            runs: Vec::new(),
            property: ParagraphProperty::new(),
            style: ParagraphStyle::new(s),
        }
    }
}

impl Paragraph {
    pub fn new() -> Paragraph {
        Default::default()
    }

    pub fn add_run(mut self, run: Run) -> Paragraph {
        self.runs.push(run);
        self
    }

    pub fn align(mut self, alignment_type: AlignmentType) -> Paragraph {
        self.property = self.property.align(alignment_type);
        self
    }
}

impl BuildXML for Paragraph {
    fn build(&self) -> Vec<u8> {
        XMLBuilder::new()
            .open_paragraph()
            .add_child(&self.property)
            .add_child(&self.style)
            .add_children(&self.runs)
            .close()
            .build()
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[cfg(test)]
    use pretty_assertions::assert_eq;
    use std::str;

    #[test]
    fn test_paragraph() {
        let b = Paragraph::new().add_run(Run::new("Hello")).build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<w:p><w:pPr /><w:pStyle w:val="Normal" /><w:r><w:rPr /><w:t xml:space="preserve">Hello</w:t></w:r></w:p>"#
        );
    }
}
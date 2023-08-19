use maud::{Markup, Render};

pub struct MarkupBuilder<'a> {
    tag: &'a str,
    attributes: String,
    inner_html: Option<Markup>,
}

impl<'a> MarkupBuilder<'a> {
    pub fn new(tag: &'a str) -> Self {
        Self {
            tag,
            attributes: String::new(),
            inner_html: None,
        }
    }

    fn push_equals_attribute_value(mut self, value: &str) -> Self {
        self.attributes.push('=');

        self.attributes.push('"');
        html_escape::encode_double_quoted_attribute_to_string(value, &mut self.attributes);
        self.attributes.push('"');

        self
    }

    pub fn attribute(mut self, key: &str, value: &str) -> Self {
        // + 4 for the leading space, equals sign, and quotes.
        self.attributes
            .reserve(key.as_bytes().len() + value.as_bytes().len() + 4);

        self.attributes.push(' ');

        for ch in key.chars() {
            if ch.is_ascii() && (ch.is_alphanumeric() || matches!(ch, '-' | '_' | ':' | '.' | '@'))
            {
                self.attributes.push(ch);
            }
        }

        self.push_equals_attribute_value(value)
    }

    fn attribute_preescaped_name(mut self, key: &str, value: &str) -> Self {
        // + 4 for the leading space, equals sign, and quotes.
        self.attributes
            .reserve(key.as_bytes().len() + value.as_bytes().len() + 4);

        self.attributes.push(' ');

        self.attributes.push_str(key);

        self.push_equals_attribute_value(value)
    }

    pub fn class(self, class: &str) -> Self {
        self.attribute_preescaped_name("class", class)
    }

    pub fn inner_html(mut self, inner_html: impl Render) -> Self {
        self.inner_html = Some(inner_html.render());
        self
    }
}

impl<'a> Render for MarkupBuilder<'a> {
    fn render_to(&self, buffer: &mut String) {
        buffer.push('<');
        buffer.push_str(self.tag);
        buffer.push_str(&self.attributes);
        buffer.push('>');

        if let Some(inner) = self.inner_html.as_ref() {
            inner.render_to(buffer);
        }

        buffer.push('<');
        buffer.push('/');
        buffer.push_str(self.tag);
        buffer.push('>');
    }
}

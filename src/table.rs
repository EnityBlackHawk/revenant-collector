use comfy_table::{Attribute, Cell, ContentArrangement, Table};
use comfy_table::presets::UTF8_FULL;
use comfy_table::modifiers::UTF8_SOLID_INNER_BORDERS;

pub trait Tabulated<const N: usize> {
    fn print(&self) -> [&str; N];
    fn headers<'a>() -> [&'a str; N];

}

fn elide_text(text: &str, max_len: usize) -> String {
    if text.len() <= max_len {
        text.to_string()
    } else if max_len > 3 {
        let mut s = String::from("...");
        s.push_str(&text[(text.len() - max_len + 3)..]);
        s
    } else {
        text[..max_len].to_string()
    }
}

pub fn print_table<T: Tabulated<N>, const N: usize>(entries: &[T], width: u16) {
    let headers = T::headers();
    let mut table = Table::new();
    table.set_width(width);
    table.set_truncation_indicator("...");
    table.set_content_arrangement(ContentArrangement::Dynamic);
    table.set_header(headers.map(|x| Cell::new(x).add_attribute(Attribute::Bold)));
    table.load_preset(UTF8_FULL);
    table.apply_modifier(UTF8_SOLID_INNER_BORDERS);
    println!();
    let max_col_width = table.width().unwrap() as usize / N;
    for entry in entries {
        table.add_row(entry.print().map(|x| elide_text(x, max_col_width)) );
    }
    println!("{table}");
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_elide() {
        let short_text = "Short text";
        let long_text = "This is a very long text that should be elided when the maximum length is exceeded.";

        assert_eq!(elide_text(short_text, 20), "Short text");
        assert_eq!(elide_text(long_text, 22), "...length is exceeded.");

    }

    #[test]
    fn test_print_table() {
        struct TestEntry {
            col1: String,
            col2: String,
            col3: String,
        }

        impl Tabulated<3> for TestEntry {
            fn print(&self) -> [&str; 3] {
                [&self.col1, &self.col2, &self.col3]
            }

            fn headers<'a>() -> [&'a str; 3] {
                ["Column 1", "Column 2", "Column 3"]
            }
        }

        let entries = vec![
            TestEntry { col1: "Row1Chhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhol1".to_string(), col2: "Row1Col2".to_string(), col3: "Row1Col3".to_string() },
            TestEntry { col1: "Row2Col1".to_string(), col2: "Row2Col2".to_string(), col3: "Row2Col3".to_string() },
        ];

        print_table(&entries, 128);
    }
}
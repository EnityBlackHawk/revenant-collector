use comfy_table::{Attribute, Cell, Table};
use comfy_table::presets::UTF8_FULL;
use comfy_table::modifiers::UTF8_SOLID_INNER_BORDERS;

pub trait Tabulated<const N: usize> {
    fn print(&self) -> [&str; N];
    fn headers<'a>() -> [&'a str; N];

}


pub fn print_table<T: Tabulated<N>, const N: usize>(entries: &[T]) {
    let headers = T::headers();
    let mut table = Table::new();
    table.set_header(headers.map(|x| Cell::new(x).add_attribute(Attribute::Bold)));
    table.load_preset(UTF8_FULL);
    table.apply_modifier(UTF8_SOLID_INNER_BORDERS);
    println!();
    for entry in entries {
        table.add_row(entry.print());
    }
    println!("{table}");
}
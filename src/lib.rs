
mod stack_entry;
mod table;

use backtrace::Backtrace;
use stack_entry::StackEntry;
use crate::table::print_table;


#[unsafe(no_mangle)]
pub extern "C" fn print_stacktrace() {
    let bt = Backtrace::new();
    let mut entries : Vec<StackEntry> = Vec::new();
    for frame in bt.frames() {
        for symbol in frame.symbols() {
            entries.push( StackEntry::from_backtrace_symbol(symbol) );
        }
    }

    print_table(entries.as_slice());

}

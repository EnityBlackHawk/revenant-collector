mod stack_entry;
mod table;
mod dump;

use libc::{signal, c_int};
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
    print_table(entries.as_slice(), 128);
}

extern "C" fn handler(sig: c_int) {
    eprintln!("Caught signal: {}", sig);
    print_stacktrace();
    std::process::exit(1);
}

#[unsafe(no_mangle)]
pub extern "C" fn setup_one_signal_handler(sig : c_int) -> bool {
    unsafe {
        if signal(sig, handler as usize) == libc::SIG_ERR {
            eprintln!("Error setting up signal handler for SIGSEGV");
            return false
        }
        true
    }
}

pub fn setup_signal_handlers(signals : Vec<c_int>) {
    for &sig in signals.iter() {
        if !setup_one_signal_handler(sig) {
            eprintln!("Failed to set up handler for signal: {}", sig);
        }
    }
}

use backtrace::BacktraceSymbol;
use std::fmt;
use serde::{Deserialize, Serialize};
use crate::table::Tabulated;

#[derive(Serialize, Deserialize, Debug)]
pub struct StackEntry {
    name : String,
    file: String,
    line : String,
}


impl StackEntry {

    pub fn new(name : String, file : String, line : String) -> StackEntry {
        StackEntry{name, file, line}
    }

    pub fn from_backtrace_symbol(symbol: &BacktraceSymbol) -> StackEntry {

        let name = match symbol.name() {
            Some(name) => match cpp_demangle::Symbol::new(&name.to_string()) {
                Ok(sym) => sym.demangle().unwrap_or_else(|_e| name.to_string()),
                _ => {name.to_string()}
            },
            _ => { String::new() }
        };

        let file = match symbol.filename() {
            Some(path) => path.display().to_string(),
            _ => { String::new() }
        };

        let line = match symbol.lineno() {
            Some(line) => line.to_string(),
            _ => { String::new() }
        };

        StackEntry::new(name, file, line)
    }
}

impl fmt::Display for StackEntry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} on {}:{}", self.name, self.file, self.line)
    }
}

impl Tabulated<3> for StackEntry {
    fn print(&self) -> [&str; 3] {
        [&self.name, &self.file, &self.line]
    }

    fn headers<'a>() -> [&'a str; 3] {
        ["Function Name", "File", "Line"]
    }
}
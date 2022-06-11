// Copyright (c) 2022 The Quantii Contributors
//
// This file is part of Quantii.
//
// Quantii is free software: you can redistribute
// it and/or modify it under the terms of the GNU
// Lesser General Public License as published by
// the Free Software Foundation, either version 3
// of the License, or (at your option) any later
// version.
//
// Quantii is distributed in the hope that it
// will be useful, but WITHOUT ANY WARRANTY;
// without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR
// PURPOSE. See the GNU Lesser General Public
// License for more details.
//
// You should have received a copy of the GNU
// Lesser General Public License along with
// Quantii. If not, see <https://www.gnu.org/licenses/>.

//! Argument parser for Quantii Shell (Qiish).

// section clippy
#![warn(
    clippy::all,
    clippy::restriction,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo
)]
#![allow(clippy::implicit_return)]
#![allow(clippy::missing_inline_in_public_items)]
#![allow(clippy::print_stdout)]
#![allow(clippy::blanket_clippy_restriction_lints)]
#![allow(clippy::unwrap_used)]
#![allow(clippy::let_underscore_drop)]
#![allow(clippy::indexing_slicing)]
#![allow(clippy::inline_always)]
#![allow(clippy::unwrap_in_result)]
#![allow(clippy::as_conversions)]
#![allow(clippy::integer_arithmetic)]
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::panic)]

use std::str::SplitWhitespace;

#[cfg(test)]
mod tests {

    #[test]
    fn no_options_flags() {
        let mut parser: crate::ArgParser = crate::ArgParser::new("".to_owned());
        parser.parse();
        assert_eq!(parser.flags.len(), 0);
    }

    #[test]
    fn no_options_args() {
        let mut parser: crate::ArgParser = crate::ArgParser::new("".to_owned());
        parser.parse();
        assert_eq!(parser.args.len(), 0);
    }

    #[test]
    fn short_flag_no_args_flags() {
        let mut parser: crate::ArgParser = crate::ArgParser::new("-a".to_owned());
        parser.parse();
        assert_eq!(parser.flags, vec!["a"]);
    }

    #[test]
    fn short_flag_no_args_args() {
        let mut parser: crate::ArgParser = crate::ArgParser::new("-a".to_owned());
        parser.parse();
        assert_eq!(parser.args.len(), 0);
    }

    #[test]
    fn short_flags_no_args_flags() {
        let mut parser: crate::ArgParser = crate::ArgParser::new("-a -b -cd".to_owned());
        parser.parse();
        assert_eq!(parser.flags, vec!["a", "b", "c", "d"]);
    }

    #[test]
    fn short_flags_no_args_args() {
        let mut parser: crate::ArgParser = crate::ArgParser::new("-a -b -cd".to_owned());
        parser.parse();
        assert_eq!(parser.args.len(), 0);
    }

    #[test]
    fn long_flag_no_args_flags() {
        let mut parser: crate::ArgParser = crate::ArgParser::new("--something".to_owned());
        parser.parse();
        assert_eq!(parser.flags, vec!["something"]);
    }

    #[test]
    fn long_flag_no_args_args() {
        let mut parser: crate::ArgParser = crate::ArgParser::new("--something".to_owned());
        parser.parse();
        assert_eq!(parser.args.len(), 0);
    }

    #[test]
    fn long_flags_no_args_flags() {
        let mut parser: crate::ArgParser = crate::ArgParser::new("--something --something-else".to_owned());
        parser.parse();
        assert_eq!(parser.flags, vec!["something", "something-else"]);
    }

    #[test]
    fn long_flags_no_args_args() {
        let mut parser: crate::ArgParser = crate::ArgParser::new("--something --something-else".to_owned());
        parser.parse();
        assert_eq!(parser.args.len(), 0);
    }

    #[test]
    fn short_flag_with_args_flags() {
        let mut parser: crate::ArgParser = crate::ArgParser::new("-a something".to_owned());
        parser.parse();
        assert_eq!(parser.flags, vec!["a"]);
    }

    #[test]
    fn short_flag_with_args_args() {
        let mut parser: crate::ArgParser = crate::ArgParser::new("-a something".to_owned());
        parser.parse();
        assert_eq!(parser.args, vec!["something"]);
    }

    #[test]
    fn short_flags_with_args_flags() {
        let mut parser: crate::ArgParser = crate::ArgParser::new("-ab something something-else".to_owned());
        parser.parse();
        assert_eq!(parser.flags, vec!["a", "b"]);
    }

    #[test]
    fn short_flags_with_args_args() {
        let mut parser: crate::ArgParser = crate::ArgParser::new("-ab something something-else".to_owned());
        parser.parse();
        assert_eq!(parser.args, vec!["something", "something-else"]);
    }

    #[test]
    fn long_flag_with_args_flags() {
        let mut parser: crate::ArgParser = crate::ArgParser::new("--something something_else".to_owned());
        parser.parse();
        assert_eq!(parser.flags, vec!["something"]);
    }

    #[test]
    fn long_flag_with_args_args() {
        let mut parser: crate::ArgParser = crate::ArgParser::new("--something something_else".to_owned());
        parser.parse();
        assert_eq!(parser.args, vec!["something_else"]);
    }

    #[test]
    fn long_flags_with_args_flags() {
        let mut parser: crate::ArgParser = crate::ArgParser::new("--something --something-else something_else_again".to_owned());
        parser.parse();
        assert_eq!(parser.flags, vec!["something", "something-else"]);
    }

    #[test]
    fn long_flags_with_args_args() {
        let mut parser: crate::ArgParser = crate::ArgParser::new("--something --something-else something_else_again".to_owned());
        parser.parse();
        assert_eq!(parser.args, vec!["something_else_again"]);
    }

    #[test]
    fn many_flags() {
        let mut parser: crate::ArgParser = crate::ArgParser::new("argument1 -o -p -sfr --long-option another_argument --another-long-option".to_owned());
        parser.parse();
        assert_eq!(parser.flags, vec!["o", "p", "s", "f", "r", "long-option", "another-long-option"]);
    }

    #[test]
    fn many_args() {
        let mut parser: crate::ArgParser = crate::ArgParser::new("argument1 -o -p -sfr --long-option another_argument --another-long-option".to_owned());
        parser.parse();
        assert_eq!(parser.args, vec!["argument1", "another_argument"]);
    }
}

/// The argument parser for Quantii Shell (Qiish).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArgParser {
    /// The command line arguments, as the input to the parser.
    input: String,
    /// The command line positional arguments, as a vector of strings.
    pub args: Vec<String>,
    /// The command line flags/options, as a vector of strings.
    pub flags: Vec<String>,
}

impl ArgParser {
    /// Creates a new argument parser.
    #[must_use]
    pub const fn new(input: String) -> Self {
        Self {
            input,
            args: vec![],
            flags: vec![],
        }
    }

    /// Parses the command line arguments.
    #[allow(clippy::let_underscore_must_use)]
    pub fn parse(&mut self) {
        // Split the input string into arguments. This does not take into account quotes.
        let mut args: SplitWhitespace = self.input.split_whitespace();
        while let Some(arg) = args.next().map(ToOwned::to_owned) {
            if arg.starts_with('-') {
                if arg.starts_with("--") {
                    // Long flag.
                    self.flags.push(arg.chars().skip(2).collect::<String>());
                } else {
                    // Short flag(s).
                    for c in arg.chars().skip(1) {
                        self.flags.push(c.to_string());
                    }
                }
            } else if arg.starts_with('"') {
                // Argument that resolves escape sequences.
                let _ = arg.chars().skip(1);
                let mut arg_string = String::new();
                while let Some(c) = args.next().map(ToOwned::to_owned) {
                    if c.contains('"') {
                        break;
                    }
                    arg_string.push_str(resolve_escapes(&*c).as_str());
                }
            } else if arg.starts_with('\'') {
                // Argument that does not resolve escape sequences.
                let _ = arg.chars().skip(1);
                let mut arg_string = String::new();
                while let Some(c) = args.next().map(ToOwned::to_owned) {
                    if c.contains('\'') {
                        break;
                    }
                    arg_string.push_str(c.as_str());
                }
            } else {
                // Positional argument.
                self.args.push(arg);
            }
        }
    }
}

/// Resolves escape sequences in a string.
fn resolve_escapes(in_string: &str) -> String {
    let mut out_string = String::new();
    for c in in_string.chars() {
        if c == '\\' {
            let next = in_string.chars().nth(1).unwrap();
            match next {
                // Newline
                'n' => out_string.push('\n'),
                // Tab
                't' => out_string.push('\t'),
                // Carrier return
                'r' => out_string.push('\r'),
                // Backslash
                '\\' => out_string.push('\\'),
                // Backtick
                '`' => out_string.push_str("\\`"),
                _ => out_string.push(next),
            }
        }
    }
    out_string
}

//! # Keymap
//!
//! Linux keycode to ASCII mapping for VirtIO input devices.
//!
//! TEAM_334: Extracted from kernel/src/input.rs as part of VirtIO driver refactor.
//!
//! ## Kernel SOP Alignment
//!
//! - **Rule 1 (Modular Scope):** This module handles exactly one task: key mapping.
//! - **Rule 13 (Representation):** Uses match for exhaustive keycode handling.

use super::{KEY_BACKSPACE, KEY_ENTER, KEY_SPACE, KEY_TAB};

/// Map Linux key codes to ASCII characters.
///
/// TEAM_334: Extracted from kernel/src/input.rs for reuse.
///
/// # Arguments
///
/// * `code` - Linux keycode
/// * `shift` - Whether shift key is pressed
///
/// # Returns
///
/// `Some(char)` if the keycode maps to an ASCII character, `None` otherwise.
pub fn linux_code_to_ascii(code: u16, shift: bool) -> Option<char> {
    match code {
        2..=11 => {
            // 1-9, 0
            let chars = if shift { ")!@#$%^&*(" } else { "1234567890" };
            chars.chars().nth(code as usize - 2)
        }
        16..=25 => {
            // q-p
            let chars = if shift { "QWERTYUIOP" } else { "qwertyuiop" };
            chars.chars().nth(code as usize - 16)
        }
        30..=38 => {
            // a-l
            let chars = if shift { "ASDFGHJKL" } else { "asdfghjkl" };
            chars.chars().nth(code as usize - 30)
        }
        44..=50 => {
            // z-m
            let chars = if shift { "ZXCVBNM" } else { "zxcvbnm" };
            chars.chars().nth(code as usize - 44)
        }
        // Symbols
        12 => Some(if shift { '_' } else { '-' }),
        13 => Some(if shift { '+' } else { '=' }),
        26 => Some(if shift { '{' } else { '[' }),
        27 => Some(if shift { '}' } else { ']' }),
        39 => Some(if shift { ':' } else { ';' }),
        40 => Some(if shift { '"' } else { '\'' }),
        41 => Some(if shift { '~' } else { '`' }),
        43 => Some(if shift { '|' } else { '\\' }),
        51 => Some(if shift { '<' } else { ',' }),
        52 => Some(if shift { '>' } else { '.' }),
        53 => Some(if shift { '?' } else { '/' }),

        KEY_SPACE => Some(' '),
        KEY_ENTER => Some('\n'),
        KEY_BACKSPACE => Some('\x08'),
        KEY_TAB => Some('\t'),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linux_code_to_ascii_symbols() {
        // [IN1] Regression test for shell input bug
        // Verify that dot and dash are correctly mapped
        assert_eq!(linux_code_to_ascii(52, false), Some('.')); // dot
        assert_eq!(linux_code_to_ascii(12, false), Some('-')); // dash

        // Verify shift variants
        assert_eq!(linux_code_to_ascii(52, true), Some('>')); // greater
        assert_eq!(linux_code_to_ascii(12, true), Some('_')); // underscore

        // Verify other symbols
        assert_eq!(linux_code_to_ascii(53, false), Some('/'));
        assert_eq!(linux_code_to_ascii(53, true), Some('?'));
    }

    #[test]
    fn test_numbers() {
        // Normal numbers
        assert_eq!(linux_code_to_ascii(2, false), Some('1'));
        assert_eq!(linux_code_to_ascii(11, false), Some('0'));

        // Shifted numbers (symbols)
        assert_eq!(linux_code_to_ascii(2, true), Some(')'));
        assert_eq!(linux_code_to_ascii(3, true), Some('!'));
    }

    #[test]
    fn test_letters() {
        // Lowercase
        assert_eq!(linux_code_to_ascii(16, false), Some('q'));
        assert_eq!(linux_code_to_ascii(30, false), Some('a'));
        assert_eq!(linux_code_to_ascii(44, false), Some('z'));

        // Uppercase
        assert_eq!(linux_code_to_ascii(16, true), Some('Q'));
        assert_eq!(linux_code_to_ascii(30, true), Some('A'));
        assert_eq!(linux_code_to_ascii(44, true), Some('Z'));
    }

    #[test]
    fn test_special_keys() {
        assert_eq!(linux_code_to_ascii(57, false), Some(' ')); // space
        assert_eq!(linux_code_to_ascii(28, false), Some('\n')); // enter
        assert_eq!(linux_code_to_ascii(14, false), Some('\x08')); // backspace
        assert_eq!(linux_code_to_ascii(15, false), Some('\t')); // tab
    }
}

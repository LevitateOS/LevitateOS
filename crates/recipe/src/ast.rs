//! Abstract syntax tree for S-expressions.

/// An S-expression is either an atom (string) or a list of expressions.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expr {
    /// An atom - either a symbol or a quoted string.
    /// Examples: `foo`, `"hello world"`, `14.1.0`
    Atom(String),

    /// A list of expressions.
    /// Example: `(package "name" "version" ...)`
    List(Vec<Expr>),
}

impl Expr {
    /// Returns the atom value if this is an Atom, None otherwise.
    pub fn as_atom(&self) -> Option<&str> {
        match self {
            Expr::Atom(s) => Some(s),
            Expr::List(_) => None,
        }
    }

    /// Returns the list if this is a List, None otherwise.
    pub fn as_list(&self) -> Option<&[Expr]> {
        match self {
            Expr::Atom(_) => None,
            Expr::List(items) => Some(items),
        }
    }

    /// Returns true if this is an atom with the given value.
    pub fn is_atom(&self, value: &str) -> bool {
        self.as_atom() == Some(value)
    }

    /// If this is a list, returns the first element (the "head" or command).
    pub fn head(&self) -> Option<&str> {
        self.as_list()?.first()?.as_atom()
    }

    /// If this is a list, returns all elements after the first.
    pub fn tail(&self) -> Option<&[Expr]> {
        let list = self.as_list()?;
        if list.is_empty() {
            None
        } else {
            Some(&list[1..])
        }
    }
}

impl std::fmt::Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Atom(s) => {
                if s.contains(' ') || s.contains('"') || s.is_empty() {
                    write!(f, "\"{}\"", s.replace('\\', "\\\\").replace('"', "\\\""))
                } else {
                    write!(f, "{}", s)
                }
            }
            Expr::List(items) => {
                write!(f, "(")?;
                for (i, item) in items.iter().enumerate() {
                    if i > 0 {
                        write!(f, " ")?;
                    }
                    write!(f, "{}", item)?;
                }
                write!(f, ")")
            }
        }
    }
}

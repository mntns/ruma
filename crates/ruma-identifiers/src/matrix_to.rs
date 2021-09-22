use std::fmt;

use percent_encoding::{percent_encode, NON_ALPHANUMERIC};

use crate::ServerName;

const BASE_URL: &str = "https://matrix.to/#/";

/// A reference to a user or room.
///
/// Turn it into a `matrix.to` URL through its `Display` implementation (i.e. by
/// interpolating it in a formatting macro or via `.to_string()`).
#[derive(Debug, PartialEq, Eq)]
pub struct MatrixToRef<'a> {
    id: &'a str,
    server_names: Vec<&'a ServerName>,
}

impl<'a> MatrixToRef<'a> {
    pub(crate) fn new(id: &'a str, server_names: Vec<&'a ServerName>) -> Self {
        Self { id, server_names }
    }
}

impl<'a> fmt::Display for MatrixToRef<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(BASE_URL)?;
        write!(f, "{}", percent_encode(self.id.as_bytes(), NON_ALPHANUMERIC))?;

        let mut first = true;
        for server_name in &self.server_names {
            f.write_str(if first { "?via=" } else { "&via=" })?;
            f.write_str(server_name.as_str())?;

            first = false;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::user_id;

    #[test]
    fn matrix_to_ref() {
        assert_eq!(
            user_id!("@jplatte:notareal.hs").matrix_to_url().to_string(),
            "https://matrix.to/#/%40jplatte%3Anotareal%2Ehs"
        );
    }
}

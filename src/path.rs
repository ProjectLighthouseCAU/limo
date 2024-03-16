use std::{borrow::Borrow, convert::Infallible, fmt, ops::Deref, str::FromStr};

use ref_cast::RefCast;

/// The separator for virtual paths.
pub const SEPARATOR: &str = "/";

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct VirtualPathBuf(Vec<String>);

impl VirtualPathBuf {
    pub fn root() -> Self {
        Self(vec![String::new()])
    }

    pub fn empty() -> Self {
        Self(Vec::new())
    }

    pub fn push(&mut self, path: impl AsRef<VirtualPath>) {
        let path = path.as_ref();
        let is_abs = path.is_absolute();
        let iter = path.0.into_iter().cloned();
        if is_abs {
            self.0 = iter.collect();
        } else {
            self.0.extend(iter);
        }
    }
}

impl FromIterator<String> for VirtualPathBuf {
    fn from_iter<T>(iter: T) -> Self where T: IntoIterator<Item = String> {
        Self(iter.into_iter().collect())
    }
}

impl From<Vec<String>> for VirtualPathBuf {
    fn from(value: Vec<String>) -> Self {
        Self(value)
    }
}

impl From<&str> for VirtualPathBuf {
    fn from(value: &str) -> Self {
        Self(value.split(SEPARATOR).map(|s| s.to_owned()).collect())
    }
}

impl FromStr for VirtualPathBuf {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

impl fmt::Display for VirtualPathBuf {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.deref())
    }
}

#[derive(Debug, Hash, PartialEq, Eq, RefCast)]
#[repr(transparent)]
pub struct VirtualPath([String]);

impl VirtualPath {
    pub fn parent(&self) -> &Self {
        Self::ref_cast(&self.0[..(self.0.len() - 1)])
    }

    pub fn join(&self, path: impl AsRef<VirtualPath>) -> VirtualPathBuf {
        let mut owned = self.to_owned();
        owned.push(path);
        owned
    }

    pub fn is_absolute(&self) -> bool {
        self.0.len() >= 1 && self.0[0].is_empty()
    }

    pub fn is_root(&self) -> bool {
        self.0.len() == 1 && self.is_absolute()
    }
}

impl fmt::Display for VirtualPath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_root() {
            write!(f, "/")
        } else {
            write!(f, "{}", self.0.join(SEPARATOR))
        }
    }
}

impl ToOwned for VirtualPath {
    type Owned = VirtualPathBuf;

    fn to_owned(&self) -> Self::Owned {
        self.0.into_iter().map(|s| s.to_string()).collect()
    }
}

impl Deref for VirtualPathBuf {
    type Target = VirtualPath;

    fn deref(&self) -> &VirtualPath {
        VirtualPath::ref_cast(&self.0[..])
    }
}

impl AsRef<VirtualPath> for VirtualPathBuf {
    fn as_ref(&self) -> &VirtualPath {
        self.deref()
    }
}

impl Borrow<VirtualPath> for VirtualPathBuf {
    fn borrow(&self) -> &VirtualPath {
        self.deref()
    }
}

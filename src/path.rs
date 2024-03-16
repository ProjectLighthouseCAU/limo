use std::{borrow::Borrow, fmt, ops::Deref};

use ref_cast::RefCast;

/// The separator for virtual paths.
pub const SEPARATOR: &str = "/";

/// Marker const for absolute paths.
pub const ABS: bool = true;

/// Marker const for relative paths.
pub const REL: bool = false;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct VirtualPathBuf<const IS_ABS: bool>(Vec<String>);

impl<const IS_ABS: bool> VirtualPathBuf<IS_ABS> {
    pub fn new() -> Self {
        Self(Vec::new())
    }
}

impl fmt::Display for VirtualPathBuf<ABS> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.deref())
    }
}

impl fmt::Display for VirtualPathBuf<REL> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0.join(SEPARATOR))
    }
}

#[derive(Debug, Hash, PartialEq, Eq, RefCast)]
#[repr(transparent)]
pub struct VirtualPath<const IS_ABS: bool>([String]);

impl<const IS_ABS: bool> VirtualPath<IS_ABS> {
    pub fn parent(&self) -> &Self {
        Self::ref_cast(&self.0[..(self.0.len() - 1)])
    }
}

impl VirtualPath<ABS> {
    pub fn root() -> &'static Self {
        Self::ref_cast(&[])
    }

    pub fn is_root(&self) -> bool {
        self.0.is_empty()
    }
}

impl<const IS_ABS: bool> fmt::Display for VirtualPath<IS_ABS> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if IS_ABS {
            write!(f, "{}{}", SEPARATOR, self.0.join(SEPARATOR))
        } else {
            write!(f, "{}", self.0.join(SEPARATOR))
        }
    }
}

impl<const IS_ABS: bool> ToOwned for VirtualPath<IS_ABS> {
    type Owned = VirtualPathBuf<IS_ABS>;

    fn to_owned(&self) -> Self::Owned {
        VirtualPathBuf(self.0.into_iter().map(|s| s.to_string()).collect())
    }
}

impl<const IS_ABS: bool> Deref for VirtualPathBuf<IS_ABS> {
    type Target = VirtualPath<IS_ABS>;

    fn deref(&self) -> &VirtualPath<IS_ABS> {
        VirtualPath::<IS_ABS>::ref_cast(&self.0[..])
    }
}

impl<const IS_ABS: bool> AsRef<VirtualPath<IS_ABS>> for VirtualPathBuf<IS_ABS> {
    fn as_ref(&self) -> &VirtualPath<IS_ABS> {
        self.deref()
    }
}

impl<const IS_ABS: bool> Borrow<VirtualPath<IS_ABS>> for VirtualPathBuf<IS_ABS> {
    fn borrow(&self) -> &VirtualPath<IS_ABS> {
        self.deref()
    }
}

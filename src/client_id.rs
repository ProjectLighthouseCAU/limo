use once_cell::sync::Lazy;
use uuid::Uuid;

/// This client's unique id.
pub const CLIENT_ID: Lazy<Uuid> = Lazy::new(|| Uuid::new_v4());

use const_format::concatcp;

pub const SCOPE_SEPERATOR: &str = ":";
pub const SCOPE_ACTION_READ: &str = "read";
pub const SCOPE_ACTION_WRITE: &str = "write";

pub const SCOPE_SUBJECT_USER: &str = "user";

pub const SCOPE_READ_USER: &str = concatcp!(SCOPE_ACTION_READ, SCOPE_SEPERATOR, SCOPE_SUBJECT_USER);

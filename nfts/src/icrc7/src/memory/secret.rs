use super::SECRET;

pub fn with_secret<R>(f: impl FnOnce(&[u8]) -> R) -> R {
    SECRET.with(|r| f(r.borrow().as_slice()))
}

pub fn set_secret(secret: [u8; 32]) {
    SECRET.with(|r| *r.borrow_mut() = secret);
}

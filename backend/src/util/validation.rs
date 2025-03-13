use rocket::form::{self, Error};

pub fn is_ulid<'v>(value: &Option<String>) -> form::Result<'v, ()> {
    if let Some(value) = value {
        if value.len() != ulid::ULID_LEN {
            Err(Error::validation("length must be 26 characters"))?;
        }
    }

    Ok(())
}

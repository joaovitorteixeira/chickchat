use super::validation::is_ulid;
use rocket::form::FromForm;

#[derive(FromForm)]
pub struct PaginationRequest {
    #[field(validate = range(1..=100))]
    pub limit: u8,

    #[field(validate = is_ulid())]
    pub last_id: Option<String>,
}

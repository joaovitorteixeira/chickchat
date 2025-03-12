use dotenv_codegen::dotenv;

pub struct Env {}

impl Env {
    pub fn get_backend_url() -> String {
        let backend_url = dotenv!("BACKEND_URL");

        backend_url.to_string()
    }
}

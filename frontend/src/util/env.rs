pub struct Env {}

impl Env {
    pub fn get_backend_url() -> String {
        // TODO: Get the backend url from the environment
        // See: https://github.com/leptos-rs/leptos/discussions/2530
        // let backend_url = std::env::var("BACKEND_URL").unwrap();
        let backend_url = "http://127.0.0.1:8000".to_string();

        backend_url
    }
}

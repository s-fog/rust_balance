pub mod env_load {
    use dotenv::dotenv;
    use once_cell::sync::Lazy;

    static INIT_DOTENV: Lazy<()> = Lazy::new(|| {
        dotenv().ok();
    });

    pub fn get_env_var(key: &String) -> String {
        Lazy::force(&INIT_DOTENV);

        env::var(key).expect("Missing env var")
    }
}
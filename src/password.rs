#[derive(Debug)]
pub struct Password {
    pub title: String,
    pub username: String,
    pub password: String,
    pub email: String,
}

impl Password {
    pub fn new (
        title: String,
        username: String,
        password: String,
        email: String,
    ) -> Self {
        Password {
            title,
            username,
            password,
            email,
        }
    }
}

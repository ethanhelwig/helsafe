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

    pub fn into_vec(self) -> Vec<String> {
        let mut password_as_vec = Vec::new();
        password_as_vec.push(self.title);
        password_as_vec.push(self.username);
        password_as_vec.push(self.password);
        password_as_vec.push(self.email);
        password_as_vec
    }
}

struct Password {
    title: String,
    username: String,
    password: String,
    email: String,
    recovery_codes: Vec<String>,
    notes: String,
}

impl Password {
    pub fn new (
        title: String,
        username: String,
        password: String,
        email: String,
        recovery_codes: Vec<String>,
        notes: String,
    ) -> Self {
        Password {
            title,
            username,
            password,
            email,
            recovery_codes,
            notes,
        }
    }
}

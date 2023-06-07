#[derive(Clone, Debug)]
pub struct Password {
    pub id: usize,
    pub title: String,
    pub username: String,
    pub password: String,
    pub email: String,
    pub recovery_codes: String,
    pub access_tokens: String,
    pub notes: String,
}

impl Password {
    #[allow(dead_code)]
    pub fn new (
        title: String, 
        username: String,
        password: String,
        email: String,
        recovery_codes: String,
        access_tokens: String,
        notes: String,
    ) -> Self {
        Password {
            id: 0,
            title,
            username,
            password,
            email,
            recovery_codes,
            access_tokens,
            notes
        }
    }

    pub fn new_with_id (
        id: usize,
        title: String, 
        username: String,
        password: String,
        email: String,
        recovery_codes: String,
        access_tokens: String,
        notes: String,
    ) -> Self {
        Password {
            id,
            title,
            username,
            password,
            email,
            recovery_codes,
            access_tokens,
            notes
        }
    }
}
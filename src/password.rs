use std::fmt;

#[derive(Clone, Debug)]
pub struct Password {
    pub id: String,
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
            id: "0".to_string(),
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
            id: id.to_string(),
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

impl fmt::Display for Password{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}, {}, {}, {}, {}, {}, {}, {}]", self.id, self.title, self.username, self.password, self.email, self.recovery_codes, self.access_tokens, self.notes)
    }
}
use shared::models::user::UserDetailToAddOrUpdate;

use crate::preprocess::Preprocess;

impl Preprocess for UserDetailToAddOrUpdate {
    async fn process(&mut self) -> crate::Result<()> {
        self.alias = self.alias.trim().to_owned();
        self.username = self.username.trim().to_owned();
        self.password = self.password.trim().to_owned();

        if self.alias.len() < 3 || self.alias.len() > 64 {
            return Err(crate::Error::FormatError("Username length only 3 - 64!"));
        }

        if self.username.len() < 3 || self.username.len() > 64 {
            return Err(crate::Error::FormatError("Username length only 3 - 64!"));
        }

        if self.password.len() < 8 || self.password.len() > 128 {
            return Err(crate::Error::FormatError("Password length only 8 - 128!"));
        }
        self.email = self.email.trim().to_owned();

        Ok(())
    }
}

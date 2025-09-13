use std::collections::HashMap;

use crate::domain::{
    data_stores::{LoginAttemptId, TwoFACode, TwoFACodeStore, TwoFACodeStoreError},
    email::Email,
};

#[derive(Default)]
pub struct HashmapTwoFACodeStore {
    codes: HashMap<Email, (LoginAttemptId, TwoFACode)>,
}

#[async_trait::async_trait]
impl TwoFACodeStore for HashmapTwoFACodeStore {
    async fn add_code(
        &mut self,
        email: Email,
        login_attempt_id: LoginAttemptId,
        code: TwoFACode,
    ) -> Result<(), TwoFACodeStoreError> {
        if self.codes.contains_key(&email) {
            return Err(TwoFACodeStoreError::UnexpectedError);
        }
        self.codes.insert(email.clone(), (login_attempt_id, code));
        Ok(())
    }

    async fn remove_code(&mut self, email: &Email) -> Result<(), TwoFACodeStoreError> {
        match self.codes.get(email) {
            Some(_) => {
                self.codes.remove(email);
                Ok(())
            }
            None => Err(TwoFACodeStoreError::LoginAttemptIdNotFound),
        }
    }

    async fn get_code(
        &self,
        email: &Email,
    ) -> Result<(LoginAttemptId, TwoFACode), TwoFACodeStoreError> {
        match self.codes.get(email) {
            Some(code) => Ok(code.clone()),
            None => Err(TwoFACodeStoreError::LoginAttemptIdNotFound),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_add_user() {
        let mut code_store = HashmapTwoFACodeStore::default();

        let email = Email::parse("test@example.com".to_owned()).unwrap();
        let login_attempt_id = LoginAttemptId::default();
        let code = TwoFACode::default();
        let result = code_store
            .add_code(email.clone(), login_attempt_id.clone(), code.clone())
            .await;
        assert!(result.is_ok());

        // If we try to add the same code again, it should return an error
        let result = code_store
            .add_code(email.clone(), login_attempt_id.clone(), code.clone())
            .await;
        assert_eq!(result, Err(TwoFACodeStoreError::UnexpectedError));
    }

    #[tokio::test]
    async fn test_remove_code() {
        let mut code_store = HashmapTwoFACodeStore::default();
        let email = Email::parse("test@example.com".to_owned()).unwrap();
        let login_attempt_id = LoginAttemptId::default();
        let code = TwoFACode::default();
        let result = code_store
            .add_code(email.clone(), login_attempt_id.clone(), code.clone())
            .await;
        assert!(result.is_ok());

        let result = code_store.remove_code(&email).await;
        assert!(result.is_ok());

        let result = code_store.remove_code(&email).await;
        assert_eq!(result, Err(TwoFACodeStoreError::LoginAttemptIdNotFound));

        let result = code_store.get_code(&email).await;
        assert_eq!(result, Err(TwoFACodeStoreError::LoginAttemptIdNotFound));
    }

    #[tokio::test]
    async fn test_get_code() {
        let mut code_store = HashmapTwoFACodeStore::default();
        let email = Email::parse("test@example.com".to_owned()).unwrap();
        let login_attempt_id = LoginAttemptId::default();
        let code = TwoFACode::default();
        let result = code_store
            .add_code(email.clone(), login_attempt_id.clone(), code.clone())
            .await;
        assert!(result.is_ok());

        let result = code_store.get_code(&email).await;
        assert_eq!(result, Ok((login_attempt_id, code)));

        let result = code_store
            .get_code(&Email::parse("thisdoesntexist@example.com".to_owned()).unwrap())
            .await;
        assert_eq!(result, Err(TwoFACodeStoreError::LoginAttemptIdNotFound));
    }
}

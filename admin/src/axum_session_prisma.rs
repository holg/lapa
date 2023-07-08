use crate::prisma::ArcPrisma;
use async_trait::async_trait;
use axum_session::{DatabasePool, Session, SessionError, SessionStore};
use prisma_client::db::session;
use prisma_client_rust::chrono::Utc;
use std::vec;

pub type SessionPrismaSession = Session<SessionPrismaPool>;
pub type SessionPrismaSessionStore = SessionStore<SessionPrismaPool>;

#[derive(Debug, Clone)]
pub struct SessionPrismaPool {
    pool: ArcPrisma,
}

impl From<ArcPrisma> for SessionPrismaPool {
    fn from(conn: ArcPrisma) -> Self {
        SessionPrismaPool { pool: conn }
    }
}

#[async_trait]
impl DatabasePool for SessionPrismaPool {
    async fn initiate(&self, _table_name: &str) -> Result<(), SessionError> {
        dbg!(("initiate", _table_name));
        Ok(())
    }

    async fn delete_by_expiry(&self, _table_name: &str) -> Result<(), SessionError> {
        dbg!("delete_by_expiry");
        self.pool
            .session()
            .delete_many(vec![session::expires::lt(Utc::now().timestamp() as i32)])
            .exec()
            .await
            .map_err(|e| SessionError::GenericDeleteError(e.to_string()))?;
        Ok(())
    }

    async fn count(&self, _table_name: &str) -> Result<i64, SessionError> {
        dbg!("count");
        let count = self
            .pool
            .session()
            .count(vec![])
            .exec()
            .await
            .map_err(|e| SessionError::GenericSelectError(e.to_string()))?;
        dbg!(("count_result", count));
        return Ok(count);
    }

    async fn store(
        &self,
        id: &str,
        session: &str,
        expires: i64,
        _table_name: &str,
    ) -> Result<(), SessionError> {
        dbg!(("store", id, session, expires));
        self.pool
            .session()
            .create(
                id.to_string(),
                session.to_string(),
                vec![session::expires::set(Some(expires as i32))],
            )
            .exec()
            .await
            .map_err(|e| SessionError::GenericCreateError(e.to_string()))?;
        Ok(())
    }

    async fn load(&self, id: &str, _table_name: &str) -> Result<Option<String>, SessionError> {
        dbg!(("load", id));
        let result = self
            .pool
            .session()
            .find_first(vec![
                session::id::equals(id.to_string()),
                prisma_client_rust::or!(
                    session::expires::equals(None),
                    session::expires::gt(Utc::now().timestamp() as i32)
                ),
            ])
            .exec()
            .await
            .map_err(|e| SessionError::GenericSelectError(e.to_string()))?;

        dbg!(("load_result", result.clone()));
        Ok(match result {
            Some(result) => Some(result.session),
            None => None,
        })
    }

    async fn delete_one_by_id(&self, id: &str, _table_name: &str) -> Result<(), SessionError> {
        dbg!(("delete_one_by_id", id));
        self.pool
            .session()
            .delete(session::id::equals(id.to_string()))
            .exec()
            .await
            .map_err(|e| SessionError::GenericDeleteError(e.to_string()))?;
        Ok(())
    }

    async fn exists(&self, id: &str, _table_name: &str) -> Result<bool, SessionError> {
        dbg!(("exists", id));
        let result = self
            .pool
            .session()
            .count(vec![
                session::id::equals(id.to_string()),
                prisma_client_rust::or!(
                    session::expires::equals(None),
                    session::expires::gt(Utc::now().timestamp() as i32)
                ),
            ])
            .exec()
            .await
            .map_err(|e| SessionError::GenericSelectError(e.to_string()))?;
        let exists = result > 0;
        dbg!(("exists_result", id, exists));
        Ok(exists)
    }

    async fn delete_all(&self, _table_name: &str) -> Result<(), SessionError> {
        dbg!("delete_all");
        self.pool
            .session()
            .delete_many(vec![])
            .exec()
            .await
            .map_err(|e| SessionError::GenericDeleteError(e.to_string()))?;
        Ok(())
    }
}

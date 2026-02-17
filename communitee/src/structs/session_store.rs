use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use actix_session::storage::{LoadError, SaveError, SessionKey, SessionStore, UpdateError, generate_session_key};
use actix_web::cookie::time::Duration;
use anyhow::Error;

struct Session {
    ttl: Duration,
    state: HashMap<String, String>
}

#[derive(Default, Clone)]
pub(crate) struct SessionStorage(Arc<Mutex<HashMap<String, Session>>>);

impl SessionStore for SessionStorage {
    fn load(
        &self,
        session_key: &SessionKey,
    ) -> impl Future<Output = Result<Option<HashMap<String, String>>, LoadError>> {
        async {
            let store = self.0.lock()
                .map_err(|err|LoadError::Other(anyhow::anyhow!(err.to_string())))?;
            Ok(store.get(&session_key.as_ref().to_string()).map(|session|session.state.clone()))
        }
    }

    fn save(
        &self,
        state: HashMap<String, String>,
        ttl: &Duration,
    ) -> impl Future<Output = Result<SessionKey, SaveError>> {
        async {
            let mut store = self.0.lock()
                .map_err(|err|SaveError::Other(anyhow::anyhow!(err.to_string())))?;
            let key = generate_session_key();
            store.insert(key.as_ref().to_string(), Session { ttl: ttl.clone(), state });
            Ok(key)
        }
    }

    fn update(
        &self,
        session_key: SessionKey,
        session_state: HashMap<String, String>,
        ttl: &Duration,
    ) -> impl Future<Output = Result<SessionKey, UpdateError>> {
        async {
            let mut store = self.0.lock()
                .map_err(|err|UpdateError::Other(anyhow::anyhow!(err.to_string())))?;

            let session = store.get_mut(session_key.as_ref())
                .ok_or_else(||UpdateError::Other(anyhow::anyhow!("No session with key {session_key:?} found.")))?;

            session.state = session_state;
            session.ttl = ttl.clone();
            
            Ok(session_key)
        }
    }

    fn update_ttl(
        &self,
        session_key: &SessionKey,
        ttl: &Duration,
    ) -> impl Future<Output = Result<(), Error>> {
        let session_key = session_key.as_ref().to_string();
        async move {
            let mut store = self.0.lock()
                .map_err(|err|anyhow::anyhow!(err.to_string()))?;

            let session = store.get_mut(&session_key)
                .ok_or_else(||anyhow::anyhow!("No session with key {session_key:?} found."))?;

            session.ttl = ttl.clone();
            
            Ok(())
        }
    }

    fn delete(
        &self,
        session_key: &SessionKey,
    ) -> impl Future<Output = Result<(), Error>> {
        let session_key = session_key.as_ref().to_string();
        async move {
            let mut store = self.0.lock()
                .map_err(|err|anyhow::anyhow!(err.to_string()))?;

            store.remove(&session_key)
                .ok_or_else(||anyhow::anyhow!("No session with key {session_key:?} found."))?;
            
            Ok(())
        }
    }
}

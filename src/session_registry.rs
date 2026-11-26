//! Loader-issued session IDs. Kept free of loader dependencies so it can be
//! tested on any host.

use std::collections::BTreeMap;

pub(crate) type SessionId = u64;

pub(crate) struct SessionRegistry<S> {
    last_issued: SessionId,
    sessions: BTreeMap<SessionId, Entry<S>>,
}

struct Entry<S> {
    owner: usize,
    session: S,
}

impl<S> SessionRegistry<S> {
    pub(crate) const fn new() -> Self {
        Self {
            last_issued: 0,
            sessions: BTreeMap::new(),
        }
    }

    pub(crate) fn insert(&mut self, owner: usize, session: S) -> Option<SessionId> {
        let id = self.last_issued.checked_add(1)?;
        self.last_issued = id;
        self.sessions.insert(id, Entry { owner, session });
        Some(id)
    }

    pub(crate) fn get_mut(&mut self, owner: usize, id: SessionId) -> Option<&mut S> {
        self.sessions
            .get_mut(&id)
            .filter(|entry| entry.owner == owner)
            .map(|entry| &mut entry.session)
    }

    pub(crate) fn take(&mut self, owner: usize, id: SessionId) -> Option<S> {
        self.get_mut(owner, id)?;
        self.sessions.remove(&id).map(|entry| entry.session)
    }

    #[cfg(test)]
    fn with_last_issued(last_issued: SessionId) -> Self {
        Self {
            last_issued,
            sessions: BTreeMap::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const OWNER: usize = 1;
    const OTHER_OWNER: usize = 2;

    #[test]
    fn issued_ids_are_nonzero_and_distinct() {
        let mut registry = SessionRegistry::new();
        let ids: Vec<_> = (0..3)
            .map(|_| registry.insert(OWNER, ()).unwrap())
            .collect();

        assert!(ids.iter().all(|&id| id != 0));
        assert!(ids[0] != ids[1] && ids[1] != ids[2] && ids[0] != ids[2]);
    }

    #[test]
    fn exhausted_ids_are_refused_instead_of_wrapping() {
        let mut registry = SessionRegistry::with_last_issued(SessionId::MAX - 1);

        assert_eq!(registry.insert(OWNER, ()), Some(SessionId::MAX));
        assert_eq!(registry.insert(OWNER, ()), None);
        assert_eq!(registry.insert(OWNER, ()), None);
    }

    #[test]
    fn another_owner_cannot_reach_or_consume_a_session() {
        let mut registry = SessionRegistry::new();
        let id = registry.insert(OWNER, "session").unwrap();

        assert!(registry.get_mut(OTHER_OWNER, id).is_none());
        assert!(registry.take(OTHER_OWNER, id).is_none());
        assert_eq!(registry.take(OWNER, id), Some("session"));
    }

    #[test]
    fn never_issued_ids_are_unknown() {
        let mut registry = SessionRegistry::new();
        let issued = registry.insert(OWNER, ()).unwrap();

        for id in [0, issued + 1, SessionId::MAX] {
            assert!(registry.get_mut(OWNER, id).is_none());
            assert!(registry.take(OWNER, id).is_none());
        }
    }

    #[test]
    fn consumed_ids_stay_unknown_after_later_sessions_are_issued() {
        let mut registry = SessionRegistry::new();
        let consumed = registry.insert(OWNER, ()).unwrap();
        registry.take(OWNER, consumed).unwrap();

        let later = registry.insert(OWNER, ()).unwrap();

        assert_ne!(later, consumed);
        assert!(registry.get_mut(OWNER, consumed).is_none());
        assert!(registry.take(OWNER, consumed).is_none());
    }
}

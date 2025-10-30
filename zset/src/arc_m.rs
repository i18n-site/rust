use std::{borrow::Borrow, marker::PhantomData, ops::Deref, sync::Arc};

use crate::{Key, Member};

#[derive(Debug)]
pub struct ArcM<K: Key, M: Member<K>> {
  pub(crate) inner: Arc<M>,
  _p: PhantomData<K>,
}

impl<K: Key, M: Member<K>> Deref for ArcM<K, M> {
  type Target = M;
  fn deref(&self) -> &M {
    &self.inner
  }
}

impl<K: Key, M: Member<K>> Borrow<K> for ArcM<K, M> {
  fn borrow(&self) -> &K {
    self.inner.as_ref().borrow()
  }
}

impl<K: Key, M: Member<K>> From<Arc<M>> for ArcM<K, M> {
  fn from(inner: Arc<M>) -> Self {
    Self {
      inner,
      _p: PhantomData,
    }
  }
}

impl<K: Key, M: Member<K>> From<M> for ArcM<K, M> {
  fn from(m: M) -> Self {
    Self {
      inner: Arc::new(m),
      _p: PhantomData,
    }
  }
}

impl<K: Key, M: Member<K>> Clone for ArcM<K, M> {
  fn clone(&self) -> Self {
    Self {
      inner: self.inner.clone(),
      _p: PhantomData,
    }
  }
}

impl<K: Key, M: Member<K>> AsRef<M> for ArcM<K, M> {
  fn as_ref(&self) -> &M {
    self.inner.as_ref()
  }
}

impl<K: Key, M: Member<K>> PartialOrd for ArcM<K, M> {
  fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
    Some(self.cmp(other))
  }
}

impl<K: Key, M: Member<K>> Ord for ArcM<K, M> {
  fn cmp(&self, other: &Self) -> std::cmp::Ordering {
    let k1: &K = self.inner.as_ref().borrow();
    let k2: &K = other.as_ref().borrow();
    k1.cmp(k2)
  }
}

impl<K: Key, M: Member<K>> PartialEq for ArcM<K, M> {
  fn eq(&self, other: &Self) -> bool {
    let k1: &K = self.inner.as_ref().borrow();
    let k2: &K = other.as_ref().borrow();
    k1.eq(k2)
  }
}

impl<K: Key, M: Member<K>> Eq for ArcM<K, M> {}

use std::{
  cmp::min,
  hash::{Hash, Hasher},
  sync::Arc,
};

use indexmap::IndexSet;
use tokio::sync::Mutex;

pub const CHUNK_SIZE: u64 = 512 * 1024;

#[derive(Debug)]
pub struct Chunk {
  pub id: u64,
  pub ts: u64,
}

impl PartialEq for Chunk {
  fn eq(&self, other: &Self) -> bool {
    self.id == other.id
  }
}

impl Eq for Chunk {}

impl Hash for Chunk {
  fn hash<H: Hasher>(&self, state: &mut H) {
    self.id.hash(state);
  }
}

impl Chunk {
  pub fn new(id: u64) -> Self {
    Self { id, ts: 0 }
  }

  pub fn wait(&mut self) -> u64 {
    let now = coarsetime::Clock::now_since_epoch().as_secs();
    let ts = self.ts;
    self.ts = now + 6; // 如果6秒没下载完，就允许开启另外一个线程
    if ts == 0 || now >= ts {
      return 0;
    }
    ts - now
  }
}

pub fn range(id: u64, size: u64) -> (u64, u64) {
  let begin = id * CHUNK_SIZE;
  let end = min(begin + CHUNK_SIZE, size) - 1;
  (begin, end)
}

#[derive(Clone)]
pub struct ChunkLi {
  pub li: Arc<Mutex<IndexSet<Chunk, std::hash::RandomState>>>,
  pub size: u64,
}

impl ChunkLi {
  pub async fn get(&self) -> Option<(u64, u64)> {
    loop {
      if let Some((id, sleep)) = {
        let mut li = self.li.lock().await;
        if let Some(mut o) = li.shift_remove_index(0) {
          let sleep = o.wait();
          let id = o.id;
          li.insert(o);
          Some((id, sleep))
        } else {
          None
        }
      } {
        if sleep > 0 {
          tokio::time::sleep(std::time::Duration::from_secs(sleep)).await;
        }

        if self.li.lock().await.contains(&Chunk::new(id)) {
          return Some(range(id, self.size));
        }
      } else {
        return None;
      }
    }
  }

  pub async fn remove(&self, begin: u64) -> bool {
    let id = begin / CHUNK_SIZE;
    let mut li = self.li.lock().await;
    li.shift_remove(&Chunk::new(id))
  }

  pub fn new(size: u64) -> Self {
    let total_chunk = size.div_ceil(CHUNK_SIZE);

    Self {
      size,
      li: Arc::new(Mutex::new(
        IndexSet::<_, std::hash::RandomState>::from_iter((0..total_chunk).map(Chunk::new)),
      )),
    }
  }
}

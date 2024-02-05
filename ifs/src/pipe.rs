use std::{
  io::{Read, Write},
  sync::mpsc::{sync_channel, Receiver, SyncSender},
};

pub struct Writer {
  pub send: Option<SyncSender<Box<[u8]>>>,
}

macro_rules! err {
  ($e:expr) => {{
    Err(std::io::Error::new(
      std::io::ErrorKind::Other,
      format!("{}", $e),
    ))
  }};
}

impl Write for Writer {
  fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
    if let Some(send) = &self.send {
      if let Err(err) = send.send(buf.into()) {
        return err!(err);
      }
    }
    Ok(buf.len())
  }

  fn flush(&mut self) -> std::io::Result<()> {
    self.send.take();
    Ok(())
  }
}

pub struct Reader {
  pub recv: Receiver<Box<[u8]>>,
  pub pend: Vec<u8>,
}

impl Read for Reader {
  fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
    let buf_len = buf.len();

    macro_rules! rt_if_full {
      () => {
        let pend_len = self.pend.len();
        if pend_len >= buf_len {
          buf.copy_from_slice(self.pend.drain(..buf_len).as_slice());
          return Ok(buf_len);
        }
      };
    }

    rt_if_full!();

    loop {
      if let Ok(r) = self.recv.recv() {
        self.pend.extend(&r[..]);
        rt_if_full!();
      } else {
        let len = self.pend.len();
        buf.copy_from_slice(&self.pend[..]);
        return Ok(len);
      }
    }
  }
}

pub fn channel(bound: usize) -> (Writer, Reader) {
  let (send, recv) = sync_channel(bound);
  (
    Writer { send: Some(send) },
    Reader {
      recv,
      pend: Vec::new(),
    },
  )
}

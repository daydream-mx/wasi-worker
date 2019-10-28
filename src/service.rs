use crate::Worker;
use std::io::{self, Read, Write};
use std::fs::File;

/// Connects Rust Worker with browser service worker
pub struct ServiceWorker {
  output: File,
  input: io::Stdin,
  worker: Box<dyn Worker>
}

impl ServiceWorker {
  pub const OUTFILE: &'static str = "/output.bin";

  pub fn new(worker: Box<dyn Worker>) -> io::Result<ServiceWorker> {
    Ok(ServiceWorker {
      output: File::create(Self::OUTFILE)?,
      input: io::stdin(),
      worker
    })
  }
  pub fn post_message(&mut self, msg: &[u8]) -> std::io::Result<()> {
    self.output.write_all(msg)
  }
  pub fn on_message(&mut self) -> io::Result<usize> {
    let mut buf: [u8; 1000] = [0; 1000];
    let len = self.input.read(&mut buf)?;
    self.worker.on_message(&buf[0..len]);
    Ok(len)
  }
}
use std::{fmt, time::Duration};

use crossbeam_channel::{Receiver, Sender};

use log::{Level, Log};

pub struct OwnedRecord {
  pub level:   Level,
  pub message: String,
}

struct RecordSender(Sender<OwnedRecord>);

impl Log for RecordSender {
  fn enabled(&self, _: &log::Metadata) -> bool { true }
  fn log(&self, record: &log::Record) {
    self.0.send(OwnedRecord { level: record.level(), message: record.args().to_string() }).unwrap();
  }
  fn flush(&self) {}
}

static mut LOGGER: Option<Receiver<OwnedRecord>> = None;

pub fn init() {
  let (tx, rx) = crossbeam_channel::unbounded();

  unsafe {
    #[allow(static_mut_refs)]
    if LOGGER.is_some() {
      panic!("Logger already initialized");
    }

    LOGGER = Some(rx);
  }

  fern::Dispatch::new()
    .format(|out, message, record| {
      struct LineOpt(Option<u32>);
      impl fmt::Display for LineOpt {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
          if let Some(line) = self.0 { write!(f, ":{}", line) } else { Ok(()) }
        }
      }

      out.finish(format_args!("{}{}: {}", record.target(), LineOpt(record.line()), message))
    })
    .level(log::LevelFilter::Debug)
    .chain(Box::new(RecordSender(tx)) as Box<dyn Log>)
    .apply()
    .unwrap();
}

pub fn poll() -> Option<OwnedRecord> {
  let logger = unsafe {
    #[allow(static_mut_refs)]
    LOGGER.as_ref().unwrap_or_else(|| {
      panic!("Logger not initialized");
    })
  };

  // Recv with a timeout of 1 second. This is infrequent enough that it won't
  // impact performance at all, but frequent enough that when reloading the
  // generator, this poll will get re-called after a new library has been
  // swapped out.
  logger.recv_timeout(Duration::from_secs(1)).ok()
}

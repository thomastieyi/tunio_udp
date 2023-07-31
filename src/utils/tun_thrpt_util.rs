use std::time::Instant;

pub struct TunnelStats {
  bytes_sent: u64,
  bytes_recv: u64,
  last_update: Instant,
}

impl TunnelStats {

    pub fn new() -> TunnelStats {
    TunnelStats {
      bytes_sent: 0,
      bytes_recv: 0,
      last_update: Instant::now(),
    }
  }

  pub fn record_sent(&mut self, n: u64) {
    self.bytes_sent += n;
  }
  
  pub fn record_recv(&mut self, n: u64) {
    self.bytes_recv += n;
  }

  pub fn throughput(&mut self) -> (f64, f64) {
    let now = Instant::now();
    let duration = now.duration_since(self.last_update);
    self.last_update = now;

    let secs = duration.as_secs() as f64 + duration.subsec_nanos() as f64 / 1_000_000_000.0;
    let sent = (self.bytes_sent as f64) / secs;
    let recv = (self.bytes_recv as f64) / secs;

    (sent, recv)
  }

}
use std::collections::{HashMap, VecDeque};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

pub trait LoadBalancer: Send {
    fn select_backend(&mut self) -> u16;

    // TODO: Manual lifecycle management risks counter leaks on early returns/errors
    // Consider guard pattern if this becomes an issue in practice
    fn on_request_complete(&mut self, _backend: u16) {}
}

pub struct RoundRobinBalancer {
    queue: VecDeque<u16>,
}

impl RoundRobinBalancer {
    pub fn new(backends: Vec<u16>) -> Self {
        Self {
            queue: backends.into_iter().collect(),
        }
    }
}

impl LoadBalancer for RoundRobinBalancer {
    fn select_backend(&mut self) -> u16 {
        let backend = self.queue.pop_front().expect("No backends available");
        self.queue.push_back(backend);
        backend
    }
}

pub struct LeastConnectionsBalancer {
    backends: Vec<u16>,
    active_connections: HashMap<u16, Arc<AtomicUsize>>,
}

impl LeastConnectionsBalancer {
    pub fn new(backends: Vec<u16>) -> Self {
        let active_connections = backends
            .iter()
            .map(|&port| (port, Arc::new(AtomicUsize::new(0))))
            .collect();

        Self {
            backends,
            active_connections,
        }
    }
}

impl LoadBalancer for LeastConnectionsBalancer {
    fn select_backend(&mut self) -> u16 {
        let backend = self
            .backends
            .iter()
            .min_by_key(|&&port| {
                self.active_connections[&port].load(Ordering::Relaxed)
            })
            .copied()
            .expect("No backends available");

        self.active_connections[&backend].fetch_add(1, Ordering::Relaxed);
        backend
    }

    fn on_request_complete(&mut self, backend: u16) {
        self.active_connections[&backend].fetch_sub(1, Ordering::Relaxed);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_round_robin() {
        let mut lb = RoundRobinBalancer::new(vec![3001, 3002, 3003]);

        assert_eq!(lb.select_backend(), 3001);
        assert_eq!(lb.select_backend(), 3002);
        assert_eq!(lb.select_backend(), 3003);
        assert_eq!(lb.select_backend(), 3001); // wraps around
        assert_eq!(lb.select_backend(), 3002);
    }
}

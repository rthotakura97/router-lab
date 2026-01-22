use std::collections::VecDeque;

pub trait LoadBalancer: Send {
    fn select_backend(&mut self) -> u16;
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

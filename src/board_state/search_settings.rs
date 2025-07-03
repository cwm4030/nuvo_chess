use std::time::Instant;

pub struct SearchSettings {
    pub start: Instant,
    pub depth: usize,
    pub stop_search: bool,
    pub nodes: usize,
    pub time: f64,
}

impl SearchSettings {
    pub fn new() -> Self {
        SearchSettings {
            start: Instant::now(),
            depth: usize::MAX,
            stop_search: false,
            nodes: usize::MAX,
            time: f64::MAX,
        }
    }

    pub fn should_stop_search(&self, depth: usize, nodes: usize) -> bool {
        self.stop_search
            || self.start.elapsed().as_secs_f64() >= self.time
            || depth > self.depth
            || nodes >= self.nodes
    }
}

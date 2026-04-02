#[derive(Debug, Clone)]
pub struct Candle {
    open: u64,
    close: u64,
    max: u64,
    min: u64,
}

impl Candle {
    pub fn new(open: u64, close: u64, max: u64, min: u64) -> Self {
        Self {
            open,
            close,
            max,
            min,
        }
    }

    pub fn open(&self) -> u64 {
        self.open
    }

    pub fn close(&self) -> u64 {
        self.close
    }

    pub fn max(&self) -> u64 {
        self.max
    }

    pub fn min(&self) -> u64 {
        self.min
    }
}

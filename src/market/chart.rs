#[derive(Debug, Clone)]
pub struct Candle {
    open: u32,
    close: u32,
    max: u32,
    min: u32,
}

impl Candle {
    pub fn new(open: u32, close: u32, max: u32, min: u32) -> Self {
        Self {
            open,
            close,
            max,
            min,
        }
    }

    pub fn open(&self) -> u32 {
        self.open
    }

    pub fn close(&self) -> u32 {
        self.close
    }

    pub fn max(&self) -> u32 {
        self.max
    }

    pub fn min(&self) -> u32 {
        self.min
    }
}

pub mod proto;
pub use proto::*;

use proto::product2::Dimension;

impl Dimension {
    pub fn new() -> Self {
        Dimension {
            unit: String::default(),
            width: 0.0,
            height: 0.0,
            length: 0.0,
        }
    }
}

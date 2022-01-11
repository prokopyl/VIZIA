use std::rc::Rc;

use parley::context::TextSource;



pub trait TextStorage {
    fn as_str(&self) -> &str;
}

impl TextStorage for String {
    fn as_str(&self) -> &str {
        self.as_str()
    }
}

pub struct Storage(Rc<dyn TextStorage>);

impl TextSource for Storage {
    fn as_str(&self) -> &str {
        self.0.as_str()
    }
}
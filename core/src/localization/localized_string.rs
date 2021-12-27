use crate::LocalizedStringKey;

use fluent_bundle::FluentValue;


pub struct LocalizedString<'a> {
    key: &'a str,
    args: Option<Vec<&'a str>>,
}

impl<'a> LocalizedString<'a> {
    pub fn new(key: &'a str) -> Self {
        Self {
            key,
            args: None,
        }
    }

    // TODO
    //pub fn with_arg() 
}

impl<'a> LocalizedStringKey<'a> for LocalizedString<'a> {
    fn key(&self) -> &'a str {
        self.key
    }
}

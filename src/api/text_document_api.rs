use crate::api::APIRegister;

pub struct TextDocumentAPI {}

impl TextDocumentAPI {}

impl APIRegister for TextDocumentAPI {
    fn register_methods(api: &mut super::API) {}
}

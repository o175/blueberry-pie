
mod utils;

extern crate serde_json;

extern crate once_cell;

use  valico::json_schema;

use url;

// use valico::json_schema::schema::ScopedSchema;
use serde_json::{Value};
// use once_cell::sync::OnceCell;

use std::sync::RwLock;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Validator {
    scope: RwLock<json_schema::Scope>
}

#[wasm_bindgen]
impl Validator{
    pub fn new() -> Validator {
        utils::set_panic_hook();
        let scope = RwLock::new(json_schema::Scope::new());
        Validator{
            scope
        }
    }
    #[wasm_bindgen(js_name = addSchema)]
    pub fn add_schema(&mut self, schema: String, id: Option<String>) -> Result<String, JsValue> {
        let json_v4_schema : Value = serde_json::from_str(&schema)
            .map_err(|_| JsValue::from_str(&"JSON parsing error"))?;
        let mut scope = self.scope.write().unwrap();
        let url = match id {
            None => scope.compile_and_return(json_v4_schema, false),
            Some(uid) => scope.compile_and_return_with_id(
                &(url::Url::parse(&uid)).map_err(|e| JsValue::from_str(&e.to_string()))?,
                json_v4_schema,
                false)
        };
        let scoped_schema = url.map_err(|e| JsValue::from_str(&e.to_string()))?;
        let url = (*scoped_schema).id.as_ref().unwrap();        
        Ok((*url.as_str()).to_string())
    }
    #[wasm_bindgen(js_name = validate)]
    pub fn validate_json (&self, json: String, id: String) -> Result<bool, JsValue> {
        let json : Value = serde_json::from_str(&json).map_err(|_| JsValue::from_str(&"JSON parsing error"))?;
        let lock = self.scope.read().unwrap();
        let url = url::Url::parse(&id).map_err(|e| JsValue::from_str(&e.to_string()))?;
        let validation_result = lock.resolve(&url).unwrap().validate(&json);
        match validation_result.is_valid(){
            true => Ok(true),
            false => Err(JsValue::from_serde(&validation_result).unwrap()),
        }
    }
}


// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = BlockParsingOptions)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `BlockParsingOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BlockParsingOptions`*"]
    pub type BlockParsingOptions;
    #[wasm_bindgen(method, setter = "blockScriptCreated")]
    fn block_script_created_shim(this: &BlockParsingOptions, val: bool);
}
impl BlockParsingOptions {
    #[doc = "Construct a new `BlockParsingOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BlockParsingOptions`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `blockScriptCreated` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BlockParsingOptions`*"]
    pub fn block_script_created(&mut self, val: bool) -> &mut Self {
        self.block_script_created_shim(val);
        self
    }
}
impl Default for BlockParsingOptions {
    fn default() -> Self {
        Self::new()
    }
}

use std::collections::HashMap;

// #![no_std]
// extern crate alloc;
// use alloc::{
//     format,
//     string::{String, ToString},
//     vec::Vec,
// };
// use casper_types::bytesrepr::FromBytes;
// use kunftmarketplace_contract::SellOrder;
// The wasm-pack uses wasm-bindgen to build and generate JavaScript binding file.
// Import the wasm-bindgen crate.
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

// #[derive(Serialize, Deserialize)]
// pub struct JSONSellOrder {
//     pub creator: String,
//     pub collection: String,
//     pub token_id: String,
//     pub pay_token: String,
//     pub price: String,
//     pub start_time: u64,
// }

// #[wasm_bindgen]
// pub fn from_bytes(bytes: Vec<u8>) -> JsValue {
//     let (sell_order, _) = SellOrder::from_bytes(&bytes).unwrap();

//     let json = JSONSellOrder {
//         creator: format!("{:?}", sell_order.creator),
//         collection: sell_order.collection.to_string(),
//         token_id: format!("{:?}", sell_order.token_id),
//         pay_token: format!("{:?}", sell_order.pay_token),
//         price: format!("{:?}", sell_order.price),
//         start_time: sell_order.start_time,
//     };

//     JsValue::from_serde::<JSONSellOrder>(&json).unwrap()
// }

#[derive(Serialize, Deserialize)]
pub struct Example {
    pub field1: HashMap<u32, String>,
    pub field2: Vec<Vec<f32>>,
    pub field3: [f32; 4],
}

#[wasm_bindgen]
pub fn send_example_to_js() -> JsValue {
    let mut field1 = HashMap::new();
    field1.insert(0, String::from("ex"));
    let example = Example {
        field1,
        field2: vec![vec![1., 2.], vec![3., 4.]],
        field3: [1., 2., 3., 4.],
    };

    JsValue::from_serde(&example).unwrap()
}

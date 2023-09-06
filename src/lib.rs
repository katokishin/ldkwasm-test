use std::str;
use wasm_bindgen::prelude::*;
use lightning_invoice::Bolt11Invoice;

// Decodes amount in invoice. Takes an array of bytes, decodes it as BOLT11 invoice.
// Returns amount in msats if it can be encoded as i64, otherwise returns 0.

#[wasm_bindgen]
pub fn decode_invoice_amount(invoice: &[u8]) -> i64 {
    let invoice_str = str::from_utf8(invoice).unwrap();
    let invoice_obj = invoice_str.parse::<Bolt11Invoice>().unwrap();
    let amount_msats = Bolt11Invoice::amount_milli_satoshis(&invoice_obj).unwrap_or(0);
    amount_msats.try_into().unwrap_or(0)
}
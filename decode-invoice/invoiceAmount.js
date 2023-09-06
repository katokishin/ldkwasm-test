// Loads the bindings and wasm as a module
const ldkwasm = require('../ldkwasm_test');
const encoder = new TextEncoder();

module.exports = (RED) => {
  function InvoiceAmount(config) {
    RED.nodes.createNode(this, config);
    const node = this;
    node.on('input', (msg, send, done) => {
      if (msg.payload !== undefined) {
        const msats = ldkwasm.decode_invoice_amount(encoder.encode(msg.payload));
        msg.payload = msats;
        send(msg);
        done();
      } else {
        done();
      }
    })
  }
  RED.nodes.registerType('invoice-amount', InvoiceAmount);
}
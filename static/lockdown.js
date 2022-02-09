function NoFetch() {
  throw new Error("Use of the fetch() API is disabled.");
}

function NoWebSocket() {
  throw new Error("Use of the WebSocket API is disabled.");
}

function NoXMLHttpRequest() {
  throw new Error("Use of the XMLHttpRequest API is disabled.");
}

window.fetch = globalThis.fetch = NoFetch;
window.WebSocket = globalThis.WebSocket = NoWebSocket;
window.XMLHttpRequest = globalThis.XMLHttpRequest = NoXMLHttpRequest;

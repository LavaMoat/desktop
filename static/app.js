import RpcProxy from './rpc.js';

const _dbg = document.querySelector(".debug");
function debug(msg) {
  const el = document.createElement("p");
  el.innerText = '' + msg;
  _dbg.appendChild(el);
}

window.dbg = debug;

class App {
  rpc = null;

  constructor() {
    this.rpc = new RpcProxy();
  }

  async start() {
    const result = await this.rpc.call("hello", "world");
    dbg(result);
  }
}

if (window.ipc) {
  const app = new App();
  app.start();
} else {
  alert("No IPC mechanism, window.ipc is not available");
}

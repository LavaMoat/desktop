import RpcProxy from './rpc.js';

const _dbg = document.querySelector(".debug");
function debug(msg) {
  const el = document.createElement("p");
  el.innerText = '' + msg;
  _dbg.appendChild(el);
}

window.dbg = debug;

class App {
  ipc = null;

  constructor() {
    this.ipc = new RpcProxy();
  }

  async start() {
    //const accounts = await this.ipc.call("Account.list");
    //dbg(JSON.stringify(accounts));

    const exists = await this.ipc.call("Account.exists");
    if (!exists) {
      dbg("No account yet...");
      const signup = document.getElementById("signup");
      signup.addEventListener('click', async () => {
        debug("Signing up account...");
        const {address, mnemonic} = await this.ipc.call("Account.signup", "mock password");
        debug("Account address " + address);
        debug("Signed up with " + mnemonic);
      });
    } else {
      debug("Do login...");
    }

    /*
    const login = document.getElementById("login");
    login.addEventListener('click', async () => {
      const passphrase = document.getElementById("passphrase").value;
      debug("Decrypting wallet..." + passphrase);
      const result = await this.ipc.call("Account.login", passphrase);
    });
    */

    //const address = await this.ipc.call("Account.create");
    //dbg(address);

  }
}

if (window.ipc) {
  const app = new App();
  app.start();
} else {
  alert("No IPC mechanism, window.ipc is not available");
}

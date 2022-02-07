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

  importFile() {

  }

  async start() {
    const listAccounts = document.getElementById("list-accounts");
    const accountsList = document.querySelector(".accounts");

    listAccounts.addEventListener('click', async () => {
      const accounts = await this.ipc.call("Account.list");
      accountsList.innerText = JSON.stringify(accounts, undefined, 2);
    });

    const exists = await this.ipc.call("Account.exists");
    if (!exists) {
      const signup = document.getElementById("signup");
      signup.addEventListener('click', async () => {
        debug("Signing up account...");
        const {address, mnemonic} = await this.ipc.call(
          "Account.signup", "mock password");
        debug("Account address " + address);
        debug("Signed up with " + mnemonic);
      });
    } else {
      const login = document.getElementById("login");
      login.addEventListener('click', async () => {
        const passphrase = document.getElementById("passphrase").value;
        const {address} = await this.ipc.call("Account.login", passphrase);
        debug("Logged in to account: " + address);
      });
    }
  }
}

if (window.ipc) {
  const app = new App();
  app.start();
} else {
  alert("No IPC mechanism, window.ipc is not available");
}

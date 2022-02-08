import { h, Component, render } from './vendor/preact.module.js';
import { useState } from './vendor/hooks.module.js';
import Router from './vendor/router.module.js';
import htm from './vendor/htm.module.js';

import RpcProxy from './rpc.js';

// Initialize htm with Preact
const html = htm.bind(h);

const _dbg = document.querySelector(".debug");
function debug(msg) {
  const el = document.createElement("p");
  el.innerText = '' + msg;
  _dbg.appendChild(el);
}

window.dbg = debug;

window.onerror = (e) => {
  dbg("ERROR: " + e);
}

function Signup(props) {
  return html`
    <div>
      <h3>Signup</h3>
      <input type="password" id="signup-passphrase" />
      <button>Create an account</button>
    </div>
  `;
}

function Login(props) {
  const [passphrase, setPassphrase] = useState("");

  const onChange = (event) => {
    event.preventDefault();
    setPassphrase(event.target.value);
  }

  const onClick = (event) => {
    event.preventDefault();
    throw new Error("mock error");
    dbg("Login:" + passphrase);
  }

  return html`
    <div>
      <h3>Login</h3>
      <input type="password" onChange=${onChange} value=${passphrase} />
      <button onClick=${onClick}>Login</button>
    </div>
  `;
}

function Header(props) {
  return html`
    <header>
      <h1><a href="/">MetaMask</a></h1>
    </header>
  `;
}

function Home(props) {
  return html`
    <p>Welcome! <a href="/signup">Signup</a> or <a href="/login">Login</a></p>
  `;
}

function App (props) {
  return html`
    <${Header} />
    <${Router}>
      <${Home} path="/" />
      <${Signup} path="/signup" />
      <${Login} path="/login" />
    <//>
  `;
}

render(html`<${App} />`, document.querySelector("main"));

/*
class App {
  ipc = null;

  constructor() {
    this.ipc = new RpcProxy();
  }

  async start() {

    dbg("Start was called...");

    const listAccounts = document.getElementById("list-accounts");
    const accountsList = document.querySelector(".accounts");

    listAccounts.addEventListener('click', async () => {
      const accounts = await this.ipc.call("Account.list");
      accountsList.innerText = JSON.stringify(accounts, undefined, 2);
    });

    const exists = await this.ipc.call("Account.exists");
    if (!exists) {
      const signup = document.getElementById("signup");
      const passphrase = document.getElementById("signup-passphrase").value;
      debug("Signup: " + passphrase);
      signup.addEventListener('click', async () => {
        debug("Signing up account...");
        const {address, mnemonic} = await this.ipc.call(
          "Account.signup", passphrase);
        debug("Account address " + address);
        debug("Signed up with " + mnemonic);
      });
    } else {
      const login = document.getElementById("login");
      login.addEventListener('click', async () => {
        const passphrase = document.getElementById("login-passphrase").value;
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
*/

import { h, Component, render } from './vendor/preact.module.js';
import { useState, useEffect } from './vendor/hooks.module.js';
import Router, { route } from './vendor/router.module.js';
import htm from './vendor/htm.module.js';
import {reaction, makeObservable, observable} from './vendor/mobx.module.js';

import IpcProxy from './ipc.js';

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
  dbg('' + e);
}

window.onunhandledrejection = (e) => {
  dbg('' + e.reason);
}

function Signup(props) {
  return html`
    <div>
      <h3>Signup</h3>
      <input type="password" />
      <button>Create an account</button>
    </div>
  `;
}

function Login(props) {
  const {ipc} = props.state;
  const [passphrase, setPassphrase] = useState("");

  const onChange = (event) => {
    event.preventDefault();
    setPassphrase(event.target.value);
  }

  const onClick = async (event) => {
    event.preventDefault();

    dbg("Doing login...");

    try {
      const account = await ipc.call("Account.login", passphrase);
      props.state.primaryAccount = account;
      route("/dashboard");
    } catch (e) {
      dbg("Login failed: " + e);
    }
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
  const [isAuthenticated, setAuthenticated] =
    useState(props.state.authenticated);

  reaction(
    () => props.state.primaryAccount,
    (value) => setAuthenticated(value !== null));

  const AuthState = isAuthenticated
    ? html`<p>Logged in!</p>`
    : html`<p>Needs log in</p>`;

  return html`
    <header>
      <h1><a href="/">MetaMask</a></h1>
      ${AuthState}
    </header>
  `;
}

function Dashboard(props) {
  const [accounts, setAccounts] = useState(props.state.accounts);
  const {ipc} = props.state;

  reaction(
    () => props.state.accounts,
    (value) => setAccounts(value));

  const loadAccounts = async () => {
    const accounts = await ipc.call("Account.list");
    props.state.accounts = accounts;
  }

  useEffect(() => {
    loadAccounts();
  }, []);

  return html`
    <p>${JSON.stringify(accounts)}</p>
  `;
}

function Home(props) {
  return html`
    <p>Welcome! <a href="/signup">Signup</a> or <a href="/login">Login</a></p>
  `;
}

class State {
  ipc = null;
  primaryAccount = null;
  accounts = [];

  constructor() {
    makeObservable(this, {
      primaryAccount: observable,
      accounts: observable,
    });

    this.ipc = new IpcProxy();
  }

  get authenticated() {
    this.primaryAccount !== null;
  }
}

function App (props) {
  const state = new State();

  return html`
    <${Header} state=${state} />
    <${Router}>
      <${Home} path="/" state=${state} />
      <${Signup} path="/signup" state=${state} />
      <${Login} path="/login" state=${state} />
      <${Dashboard} path="/dashboard" state=${state} />
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

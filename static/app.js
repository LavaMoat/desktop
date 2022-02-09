import { h, Component, render } from './vendor/preact.module.js';
import { useState, useEffect } from './vendor/hooks.module.js';
import Router, { route } from './vendor/router.module.js';
import htm from './vendor/htm.module.js';
import {reaction, makeObservable, observable} from './vendor/mobx.module.js';

import './debug.js';
import State from './state.js';

// Initialize htm with Preact
const html = htm.bind(h);

function Signup(props) {
  return html`
    <div>
      <h3>Signup</h3>
      <input type="password" />
      <button>Create an account</button>
    </div>
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
  const {state} = props;
  const {ipc} = state;

  const login = async (event) => {
    event.preventDefault();
    const account = await ipc.call("Account.login");
    if (account) {
      state.primaryAccount = account;
      route("/dashboard");
    }
  }

  if (state.authenticated) {
    route('/dashboard');
    return null;
  }

  return html`
    <p>Welcome! <a href="/signup">Signup</a> or <a href="#" onClick=${login}>Login</a></p>
  `;
}

function Header(props) {
  const [isAuthenticated, setAuthenticated] =
    useState(props.state.authenticated);

  reaction(
    () => props.state.authenticated,
    (value) => setAuthenticated(value));

  const AuthState = isAuthenticated
    ? html`<p><a href="/logout">Logout</a></p>`
    : html``;

  return html`
    <header>
      <h1><a href="/">MetaMask</a></h1>
      ${AuthState}
    </header>
  `;
}

function Logout(props) {
  const {ipc} = props.state;
  const logout = async () => {
    await ipc.call("Account.logout");
    props.state.reset();
    route("/");
  }

  useEffect(() => logout(), []);
  return null;
}

function App (props) {
  const state = new State();

  return html`
    <main>
      <div class="debug"></div>
      <${Header} state=${state} />
      <img src="qrcode://?text=Hello+world" />
      <${Router}>
        <${Home} path="/" state=${state} />
        <${Signup} path="/signup" state=${state} />
        <${Logout} path="/logout" state=${state} />
        <${Dashboard} path="/dashboard" state=${state} />
      <//>
    </main>
  `;
}

render(html`<${App} />`, document.body);

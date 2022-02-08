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

function Login(props) {
  const {ipc} = props.state;
  const [passphrase, setPassphrase] = useState("");

  const onChange = (event) => {
    event.preventDefault();
    setPassphrase(event.target.value);
  }

  const onClick = async (event) => {
    event.preventDefault();
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

  if (state.authenticated) {
    route('/dashboard');
    return null;
  }

  return html`
    <p>Welcome! <a href="/signup">Signup</a> or <a href="/login">Login</a></p>
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
  props.state.reset();
  route("/");
}

function App (props) {
  const state = new State();

  return html`
    <div class="debug"></div>
    <${Header} state=${state} />
    <${Router}>
      <${Home} path="/" state=${state} />
      <${Signup} path="/signup" state=${state} />
      <${Login} path="/login" state=${state} />
      <${Logout} path="/logout" state=${state} />
      <${Dashboard} path="/dashboard" state=${state} />
    <//>
  `;
}

render(html`<${App} />`, document.querySelector("main"));

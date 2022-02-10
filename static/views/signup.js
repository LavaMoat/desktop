import { h, Component, render } from '../vendor/preact.module.js';
import Router, { route } from '../vendor/router.module.js';
import { useEffect, useState } from '../vendor/hooks.module.js';
import htm from '../vendor/htm.module.js';
//import {reaction} from './vendor/mobx.module.js';

//import State from './state.js';
//import Signup from './views/signup.js';

// Initialize htm with Preact
const html = htm.bind(h);

export function Verify(props) {
  return html`
    <div>
      <h3>Verify 2FA</h3>
    </div>
  `;
}

export function Totp(props) {
  return html`
    <div>
      <h3>Two-factor Authentication (2FA)</h3>
      <a href="/signup/verify">Next: Verify 2FA</a>
    </div>
  `;
}

export function Recovery(props) {

  const [mnemonic, setMnemonic] = useState(null);
  const {ipc} = props.state;

  useEffect(() => {
    const run = async () => {
      const mnemonic = await ipc.call("Signup.mnemonic");
      setMnemonic(mnemonic);
    };
    run();
  }, []);

  if(mnemonic === null) {
    return null;
  }

  return html`
    <div>
      <h3>Recovery Seed</h3>
      <p>This is your <em>recovery seed phrase</em>:</p>
      <pre>${mnemonic}</pre>
      <p>If you lose your login passphrase you can recover
      your <em>primary account</em> using this seed recovery phrase so
      you should store it securely.</p>
      <p>Write it down on paper and store it in a secure, fire-proof location.</p>
      <a href="/signup/totp">Next: Two-Factor Authentication</a>
    </div>
  `;
}

export function Passphrase(props) {
  const [passphrase, setPassphrase] = useState(null);
  const {ipc} = props.state;

  useEffect(() => {
    const run = async () => {
      const passphrase = await ipc.call("Signup.passphrase");
      setPassphrase(passphrase);
    };
    run();
  }, []);

  if(passphrase === null) {
    return null;
  }

  return html`
    <div>
      <h3>Passphrase</h3>
      <p>This is your <em>login passphrase</em>:</p>
      <pre>${passphrase}</pre>
      <p>You must store this in a secure place such as a password manager.</p>
      <p>This is the passphrase you use to log in to the MetaMask Desktop application.</p>
      <a href="/signup/recovery">Next: Recovery Seed</a>
    </div>
  `;
}

export function Welcome(props) {
  return html`
      <div>
        <h3>Signup</h3>
        <p>Welcome to MetaMask!</p>
        <p>We will guide you through the signup process:</p>
        <ul>
          <li>Secure your login passphrase</li>
          <li>Secure your seed recovery phrase</li>
          <li>Enable two-factor authentication (2FA)</li>
          <li>Verify 2FA token</li>
        </ul>
        <a href="/signup/passphrase">Next: Passphrase</a>
      </div>
  `;
}

export class Signup {
  componentDidMount() {
    const {ipc} = this.props.state;
    const start = async () => {
      await ipc.call("Signup.start");
    }
    start();
  }

  componentWillUnmount() {
    const {ipc} = this.props.state;
    const finish = async () => {
      // Ensure we call finish so that secrets
      // stored in memory are erased
      await ipc.call("Signup.finish");
    }
    finish();
  }

  render(props) {
    const {state} = props;
    return html`
      <${Router}>
        <${Welcome} path="/signup" state=${state} />
        <${Passphrase} path="/signup/passphrase" state=${state} />
        <${Recovery} path="/signup/recovery" state=${state} />
        <${Totp} path="/signup/totp" state=${state} />
        <${Verify} path="/signup/verify" state=${state} />
      <//>
    `;
  }
}

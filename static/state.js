import {makeObservable, observable, computed} from './vendor/mobx.module.js';

class IpcProxy {
  _id = 0;
  _ipc = null;
  _requests = new Map();

  constructor() {
    this._ipc = window.ipc;
    window.addEventListener("message", (event) => {
      const {data: response} = event;
      const resolver = this._requests.get(response.id);
      if (resolver) {
        if (response.error) {
          resolver.reject(new Error(response.error.message));
        } else if (response.result !== undefined) {
          resolver.resolve(response.result);
        }
      }
    });
  }

  _send(request) {
    const payload = JSON.stringify(request);
    this._ipc.postMessage(payload);
  }

  id() {
    ++this._id;
    return this._id;
  }

  call(method, params) {
    const id = this.id();
    const request = {
      jsonrpc: "2.0",
      id,
      method,
      params,
    };
    const p = new Promise((resolve, reject) => {
      this._requests.set(id, {resolve, reject})
    });
    this._send(request);
    return p;
  }

  notify(method, params) {
    const request = {
      jsonrpc: "2.0",
      method,
      params,
    };
    this._send(request);
    return Promise.resolve();
  }

  open(url) {
    this.notify("Browser.open", url);
  }
}

export default class State {
  ipc = null;
  primaryAccount = null;
  accounts = [];

  constructor() {
    makeObservable(this, {
      primaryAccount: observable,
      authenticated: computed,
      accounts: observable,
    });

    this.ipc = new IpcProxy();
  }

  reset() {
    this.primaryAccount = null;
    this.accounts = [];
  }

  get authenticated() {
    return this.primaryAccount !== null;
  }
}

function debug(msg) {
  const _dbg = document.querySelector(".debug");
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


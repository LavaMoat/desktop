import { h, Component, render } from '../vendor/preact.module.js';
import { useState } from '../vendor/hooks.module.js';
import htm from '../vendor/htm.module.js';

// Initialize htm with Preact
const html = htm.bind(h);

export default function TwoFactorCode(props) {
  const digits = 6;
  const [token, setToken] = useState(null);

  const onChange = (e) => {
    e.preventDefault();
    setToken(e.target.value);
  }

  const onSubmit = (e) => {
    e.preventDefault();
    props.onSubmit(token);
  }

  return html`
    <form action="#" onSubmit=${onSubmit}>
      <input type="text"
        pattern="^[0-9]{6}"
        minlength="6"
        maxlength="6"
        value=${token}
        onChange=${onChange} />
      <input
        type="submit"
        value="Verify token" />
    </form>
  `;
}

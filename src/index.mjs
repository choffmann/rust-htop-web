import { h, render } from "https://esm.sh/preact";
import htm from "https://esm.sh/htm";
const html = htm.bind(h);

const App = (props) => {
  return html`<div class="container">
    ${props.cpus.map(
      (cpu) => html`
        <div class="bar">
          <label>${cpu.toFixed(2)}%</label>
          <div class="bar-inner" style="width: ${cpu}%" />
        </div>
      `,
    )}
  </div>`;
};

let url = new URL("/realtime/cpu", window.location.href);
url.protocol = url.protocol.replace("http", "ws");
let ws = new WebSocket(url.href);

ws.onmessage = (e) => {
  const json = JSON.parse(e.data);
  render(html`<${App} cpus=${json} />`, document.body);
};

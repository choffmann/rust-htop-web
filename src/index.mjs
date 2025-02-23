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

setInterval(async () => {
  const response = await fetch("/api/cpu");
  if (response.status != 200) {
    throw new Error(`http error! status: ${response.status}`);
  }

  const json = await response.json();
  render(html`<${App} cpus=${json} />`, document.body);
}, 200);

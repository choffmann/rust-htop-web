import { h, render } from "https://esm.sh/preact";

setInterval(async () => {
  const response = await fetch("/api/cpu");
  if (response.status != 200) {
    throw new Error(`http error! status: ${response.status}`);
  }

  const json = await response.json();
  const app = h("pre", null, JSON.stringify(json, null, 2));
  render(app, document.body);
}, 1000);

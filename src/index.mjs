import { h, render } from "https://esm.sh/preact";
import htm from "https://esm.sh/htm";
const html = htm.bind(h);

//document.addEventListener("DOMContentLoaded", () => {
//  let cpuWrapper = document.createElement("div");
//  cpuWrapper.id = "cpu-wrapper";
//
//  let processWrapper = document.createElement("div");
//  cpuWrapper.id = "process-wrapper";
//
//  document.body.appendChild(cpuWrapper);
//  document.body.appendChild(processWrapper);
//});

const CpuUsage = (props) => {
  return html`<div class="cpu-container">
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

const ProcessList = (props) => {
  return html`
    <table class="process-table">
      <tr>
        <th>PID</th>
        <th>Process name</th>
      </tr>
      ${Object.entries(props.process).map(
        ([pid, pname]) => html`
          <tr>
            <td>${pid}</td>
            <td>${pname}</td>
          </tr>
        `,
      )}
    </table>
  `;
};

let cpuUrl = new URL("/realtime/cpu", window.location.href);
cpuUrl.protocol = cpuUrl.protocol.replace("http", "ws");
let cpuWs = new WebSocket(cpuUrl.href);

cpuWs.onmessage = (e) => {
  const json = JSON.parse(e.data);
  render(
    html`<${CpuUsage} cpus=${json} />`,
    document.getElementById("cpu-wrapper"),
  );
};

let processesUrl = new URL("/realtime/process", window.location.href);
processesUrl.protocol = processesUrl.protocol.replace("http", "ws");
let processesWs = new WebSocket(processesUrl.href);

processesWs.onmessage = (e) => {
  const json = JSON.parse(e.data);
  render(
    html`<${ProcessList} process=${json} />`,
    document.getElementById("process-wrapper"),
  );
};

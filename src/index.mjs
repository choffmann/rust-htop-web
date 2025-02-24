import { h, render } from "https://esm.sh/preact";
import htm from "https://esm.sh/htm";
const html = htm.bind(h);

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
        <th>CPU</th>
        <th>Memory</th>
      </tr>
      ${Object.entries(props.process).map(
        ([pid, obj]) => html`
          <tr>
            <td>${pid}</td>
            <td>${obj.name}</td>
            <td>${obj.cpu_usage.toFixed(2)}%</td>
            <td>${(obj.memory / Math.pow(1024, 2)).toFixed(2)} mb</td>
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

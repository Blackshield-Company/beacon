function showError(message) {
  const el = document.getElementById("error");
  el.hidden = !message;
  el.textContent = message || "";
}

function bridge() {
  const tauri = window.__TAURI__;
  if (!tauri || !tauri.core || typeof tauri.core.invoke !== "function") {
    showError("Open this in the Beacon window.");
    return null;
  }
  return tauri.core.invoke;
}

async function call(cmd, args) {
  const invoke = bridge();
  if (!invoke) {
    throw new Error("Open this in the Beacon window.");
  }
  try {
    return await invoke(cmd, args || {});
  } catch (err) {
    const message = typeof err === "string" ? err : (err && err.message) || String(err);
    showError(message);
    throw err;
  }
}

function field(form, name) {
  return form.elements[name].value.trim();
}

async function loadChecklist(slug) {
  const steps = await call("checklist", { slug });
  const list = document.getElementById("checklist");
  list.replaceChildren();
  for (const step of steps) {
    const li = document.createElement("li");
    li.textContent = step;
    list.appendChild(li);
  }
  document.getElementById("csam-note").hidden = slug !== "csam";
}

async function loadBoard() {
  showError("");
  const [types, text] = await Promise.all([
    call("incident_types"),
    call("resources"),
  ]);
  document.getElementById("resources").textContent = text;
  const select = document.getElementById("type");
  select.replaceChildren();
  for (const item of types) {
    const option = document.createElement("option");
    option.value = item.slug;
    option.textContent = item.label;
    select.appendChild(option);
  }
  if (types.length) {
    await loadChecklist(types[0].slug);
  }
}

document.getElementById("type").addEventListener("change", async (event) => {
  await loadChecklist(event.target.value);
});

document.addEventListener("submit", async (event) => {
  if (!event.target.matches("#report")) return;
  event.preventDefault();
  const form = event.target;
  const button = form.querySelector("button");
  if (button) button.disabled = true;
  showError("");
  document.getElementById("saved").textContent = "";
  try {
    const path = await call("save_report", {
      slug: document.getElementById("type").value,
      incidentDates: field(form, "incident_dates"),
      description: field(form, "description"),
      amountLost: field(form, "amount_lost"),
      platforms: field(form, "platforms"),
      accountsInvolved: field(form, "accounts_involved"),
      suspectIdentifiers: field(form, "suspect_identifiers"),
      evidencePreserved: field(form, "evidence_preserved"),
      contactInfo: field(form, "contact_info"),
      existingReports: field(form, "existing_reports"),
      outputPath: field(form, "output_path"),
    });
    document.getElementById("saved").textContent = "Wrote " + path;
  } catch (_err) {
    // call() already surfaced the message
  } finally {
    if (button) button.disabled = false;
  }
});

loadBoard();

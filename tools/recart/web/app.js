/* Minimal JS (no build step). Keep dependencies at zero. */

function qs(sel) {
  const el = document.querySelector(sel);
  if (!el) throw new Error("Missing element: " + sel);
  return el;
}

function fmtBytes(n) {
  if (n == null) return "";
  const KB = 1024;
  const MB = KB * 1024;
  const GB = MB * 1024;
  const TB = GB * 1024;
  if (n >= TB) return (n / TB).toFixed(2) + " TB";
  if (n >= GB) return (n / GB).toFixed(2) + " GB";
  if (n >= MB) return (n / MB).toFixed(2) + " MB";
  if (n >= KB) return (n / KB).toFixed(2) + " KB";
  return n + " B";
}

function fmtUnix(ts) {
  if (!ts) return "";
  const d = new Date(ts * 1000);
  return d.toISOString().replace("T", " ").slice(0, 19) + "Z";
}

function readTokenFromUrl() {
  const u = new URL(window.location.href);
  const t = u.searchParams.get("token");
  if (t) sessionStorage.setItem("recart_token", t);
}

function tokenHeader() {
  const t = sessionStorage.getItem("recart_token");
  return t ? { "X-Recart-Token": t } : {};
}

async function api(path, opts = {}) {
  const res = await fetch(path, opts);
  if (!res.ok) {
    let body = "";
    try {
      body = await res.text();
    } catch {
      body = "";
    }
    throw new Error(res.status + " " + res.statusText + (body ? "\n" + body : ""));
  }
  const ct = res.headers.get("content-type") || "";
  if (ct.includes("application/json")) return await res.json();
  return await res.text();
}

const state = {
  distros: [],
  kinds: [],
  selectedDistro: null,
  selectedKind: null,
  storeOffset: 0,
  storeLimit: 30,
  mutationsEnabled: false,
  treeRelPath: null,
  outRootAbs: null,
  storeRootAbs: null,

  outputsFilter: "",
  storeFilter: "",
  treeFilter: "",

  lastSummary: null,
  lastTree: null,
  lastStore: null,

  // Cache sha256 calculations by rel_path.
  shaCache: new Map(),
};

function setStatus(msg) {
  qs("#status-line").textContent = msg;
}

function setMutations(enabled) {
  state.mutationsEnabled = !!enabled;
  qs("#mut-pill").textContent = "mutations: " + (enabled ? "ON" : "off");
  qs("#gc-btn").disabled = !enabled;
  qs("#prune-btn").disabled = !enabled;
  qs("#ingest-all").disabled = !enabled;
  qs("#restore-missing").disabled = !enabled;
}

async function copyText(text) {
  try {
    await navigator.clipboard.writeText(text);
    setStatus("Copied to clipboard");
  } catch {
    window.prompt("Copy to clipboard:", text);
  }
}

function renderDistroSelect() {
  const sel = qs("#distro-select");
  sel.innerHTML = "";
  for (const d of state.distros) {
    const opt = document.createElement("option");
    opt.value = d.dir;
    opt.textContent = `${d.dir} (${d.label})`;
    sel.appendChild(opt);
  }
  if (state.selectedDistro) sel.value = state.selectedDistro;
}

function renderKindSelect() {
  const sel = qs("#kind-select");
  sel.innerHTML = "";
  for (const k of state.kinds) {
    const opt = document.createElement("option");
    opt.value = k;
    opt.textContent = k;
    sel.appendChild(opt);
  }
  if (state.selectedKind) sel.value = state.selectedKind;
}

function tag(text, ok) {
  const span = document.createElement("span");
  span.className = "tag " + (ok ? "tag--ok" : "tag--bad");
  span.textContent = text;
  return span;
}

function link(text, href) {
  const a = document.createElement("a");
  a.className = "link";
  a.textContent = text;
  a.href = href;
  a.target = "_blank";
  a.rel = "noreferrer";
  return a;
}

function actionButton(text, onClick, { danger = false, requiresMutate = false } = {}) {
  const b = document.createElement("button");
  b.className = "btn " + (danger ? "btn--danger" : "");
  b.textContent = text;
  b.onclick = onClick;
  b.disabled = requiresMutate && !state.mutationsEnabled;
  return b;
}

function renderOutputs() {
  const summary = state.lastSummary;
  if (!summary) return;
  const tbody = qs("#outputs-table tbody");
  tbody.innerHTML = "";

  const needle = (state.outputsFilter || "").trim().toLowerCase();
  const filtered = summary.artifacts.filter((row) => {
    if (!needle) return true;
    const parts = [
      row.kind || "",
      row.rel_path || "",
      row.input_key || "",
      row.store?.blob_sha256 || "",
      row.store?.format || "",
    ];
    return parts.join(" ").toLowerCase().includes(needle);
  });

  for (const row of filtered) {
    const tr = document.createElement("tr");

    const tdKind = document.createElement("td");
    tdKind.textContent = row.kind;
    tr.appendChild(tdKind);

    const tdPath = document.createElement("td");
    tdPath.appendChild(
      link(row.rel_path, `/api/v1/file/download?path=${encodeURIComponent(row.rel_path)}`)
    );
    tr.appendChild(tdPath);

    const tdExists = document.createElement("td");
    tdExists.appendChild(tag(row.exists ? "yes" : "no", row.exists));
    tr.appendChild(tdExists);

    const tdSize = document.createElement("td");
    tdSize.textContent = fmtBytes(row.size_bytes);
    tr.appendChild(tdSize);

    const tdM = document.createElement("td");
    tdM.textContent = fmtUnix(row.mtime_unix);
    tr.appendChild(tdM);

    const tdKey = document.createElement("td");
    if (row.input_key) {
      const t = document.createElement("span");
      t.textContent = row.input_key.slice(0, 16) + "…";
      t.title = row.input_key;
      t.style.cursor = "pointer";
      t.onclick = () => copyText(row.input_key);
      tdKey.appendChild(t);
    } else {
      tdKey.textContent = "";
    }
    tr.appendChild(tdKey);

    const tdSha = document.createElement("td");
    const cached = state.shaCache.get(row.rel_path);
    if (cached) {
      const t = document.createElement("span");
      t.textContent = cached.slice(0, 16) + "…";
      t.title = cached;
      t.style.cursor = "pointer";
      t.onclick = () => copyText(cached);
      tdSha.appendChild(t);
    } else {
      tdSha.textContent = "";
    }
    tr.appendChild(tdSha);

    const tdStore = document.createElement("td");
    if (row.store && row.store.present) {
      tdStore.appendChild(tag("present", true));
      tdStore.appendChild(document.createTextNode(" "));
      tdStore.appendChild(
        link(row.store.blob_sha256.slice(0, 16), `/api/v1/blob/${row.store.blob_sha256}`)
      );
      if (row.store.format) {
        tdStore.appendChild(document.createTextNode(" "));
        tdStore.appendChild(tag(row.store.format, true));
      }
      if (row.store.hardlinked_to_blob === true) {
        tdStore.appendChild(document.createTextNode(" "));
        tdStore.appendChild(tag("linked", true));
      } else if (row.store.hardlinked_to_blob === false) {
        tdStore.appendChild(document.createTextNode(" "));
        tdStore.appendChild(tag("copied", false));
      }
    } else {
      tdStore.appendChild(tag("missing", false));
    }
    tr.appendChild(tdStore);

    const tdAct = document.createElement("td");
    const wrap = document.createElement("div");
    wrap.style.display = "flex";
    wrap.style.gap = "8px";
    wrap.appendChild(
      actionButton(
        "Restore",
        async () => {
          const dir = state.selectedDistro;
          if (!dir) return;
          setStatus(`Restoring ${row.kind}…`);
          await api(`/api/v1/distro/${encodeURIComponent(dir)}/restore`, {
            method: "POST",
            headers: { "content-type": "application/json", ...tokenHeader() },
            body: JSON.stringify({ kind: row.kind }),
          });
          await loadOutputs();
          await loadTree(`${dir}`);
          setStatus(`Restored ${row.kind}`);
        },
        { requiresMutate: true }
      )
    );
    wrap.appendChild(
      actionButton(
        "Ingest",
        async () => {
          const dir = state.selectedDistro;
          if (!dir) return;
          setStatus(`Ingesting ${row.kind}…`);
          await api(`/api/v1/distro/${encodeURIComponent(dir)}/ingest_existing`, {
            method: "POST",
            headers: { "content-type": "application/json", ...tokenHeader() },
            body: JSON.stringify({ kinds: [row.kind] }),
          });
          await loadOutputs();
          setStatus(`Ingested ${row.kind}`);
        },
        { requiresMutate: true }
      )
    );
    wrap.appendChild(
      actionButton("Hash", async () => {
        if (!row.rel_path) return;
        setStatus(`Hashing ${row.rel_path}…`);
        const resp = await api(`/api/v1/file/sha256?path=${encodeURIComponent(row.rel_path)}`);
        state.shaCache.set(row.rel_path, resp.sha256);
        renderOutputs();
        setStatus(`SHA256 ${row.rel_path}: ${resp.sha256.slice(0, 16)}…`);
      })
    );
    wrap.appendChild(
      actionButton("Copy Path", async () => {
        const base = state.outRootAbs || "";
        const abs = base && row.rel_path ? `${base}/${row.rel_path}` : row.rel_path;
        await copyText(abs);
      })
    );
    tdAct.appendChild(wrap);
    tr.appendChild(tdAct);

    tbody.appendChild(tr);
  }
}

async function loadOutputs() {
  const dir = state.selectedDistro;
  if (!dir) return;

  setStatus("Loading outputs…");
  const summary = await api(`/api/v1/distro/${encodeURIComponent(dir)}/summary`);
  state.lastSummary = summary;
  state.outRootAbs = summary.out_root;
  renderOutputs();

  setStatus("Outputs loaded");
}

async function loadTree(relPath) {
  state.treeRelPath = relPath;
  qs("#tree-path").textContent = `.artifacts/out/${relPath}`;
  setStatus("Loading tree…");

  const list = await api(`/api/v1/out/ls?path=${encodeURIComponent(relPath)}`);
  state.lastTree = list;
  renderTree();
}

function renderTree() {
  const list = state.lastTree;
  const relPath = state.treeRelPath || "";
  if (!list) return;

  const root = qs("#tree-list");
  root.innerHTML = "";

  if (relPath !== "") {
    const up = document.createElement("div");
    up.className = "tree__item";
    up.onclick = () => {
      const parts = relPath.split("/").filter(Boolean);
      parts.pop();
      loadTree(parts.join("/"));
    };
    const left = document.createElement("div");
    left.className = "tree__name";
    left.textContent = "← ..";
    up.appendChild(left);
    const right = document.createElement("div");
    right.className = "tree__meta muted";
    right.textContent = "";
    up.appendChild(right);
    root.appendChild(up);
  }

  const needle = (state.treeFilter || "").trim().toLowerCase();
  const entries = list.entries.filter((e) => {
    if (!needle) return true;
    return (e.name || "").toLowerCase().includes(needle);
  });

  for (const e of entries) {
    const item = document.createElement("div");
    item.className = "tree__item";
    const left = document.createElement("div");
    left.className = "tree__name";
    left.textContent = (e.kind === "dir" ? "▸ " : "• ") + e.name;
    item.appendChild(left);

    const right = document.createElement("div");
    right.className = "tree__meta";
    right.textContent = e.kind === "dir" ? "" : fmtBytes(e.size_bytes);
    item.appendChild(right);

    if (e.kind === "dir") {
      item.onclick = () => loadTree(e.rel_path);
    } else {
      item.onclick = () =>
        window.open(
          `/api/v1/file/download?path=${encodeURIComponent(e.rel_path)}`,
          "_blank"
        );
    }

    root.appendChild(item);
  }

  setStatus("Tree loaded");
}

async function loadStore() {
  if (!state.selectedKind) return;
  setStatus("Loading store entries…");

  const kind = state.selectedKind;
  const page = Math.floor(state.storeOffset / state.storeLimit) + 1;
  qs("#store-page").textContent = `page ${page}`;

  const data = await api(
    `/api/v1/store/${encodeURIComponent(kind)}/entries?offset=${state.storeOffset}&limit=${state.storeLimit}`
  );
  state.lastStore = data;
  renderStore();
}

function renderStore() {
  const data = state.lastStore;
  if (!data) return;
  const tbody = qs("#store-table tbody");
  tbody.innerHTML = "";

  const needle = (state.storeFilter || "").trim().toLowerCase();
  const filtered = data.entries.filter((e) => {
    if (!needle) return true;
    const parts = [e.input_key || "", e.blob_sha256 || "", e.format || ""];
    return parts.join(" ").toLowerCase().includes(needle);
  });

  for (const e of filtered) {
    const tr = document.createElement("tr");

    const tdT = document.createElement("td");
    tdT.textContent = fmtUnix(e.stored_at_unix);
    tr.appendChild(tdT);

    const tdK = document.createElement("td");
    tdK.textContent = e.input_key.slice(0, 16) + "…";
    tr.appendChild(tdK);

    const tdB = document.createElement("td");
    tdB.appendChild(link(e.blob_sha256.slice(0, 16), `/api/v1/blob/${e.blob_sha256}`));
    tr.appendChild(tdB);

    const tdF = document.createElement("td");
    tdF.textContent = e.format;
    tr.appendChild(tdF);

    const tdS = document.createElement("td");
    tdS.textContent = fmtBytes(e.size_bytes);
    tr.appendChild(tdS);

    const tdO = document.createElement("td");
    const a = document.createElement("button");
    a.className = "btn btn--quiet";
    a.textContent = "View";
    a.onclick = async () => {
      const detail = await api(
        `/api/v1/store/${encodeURIComponent(kind)}/entry?input_key=${encodeURIComponent(e.input_key)}`
      );
      qs("#entry-details").textContent = JSON.stringify(detail.entry, null, 2);
    };
    tdO.appendChild(a);
    tr.appendChild(tdO);

    tbody.appendChild(tr);
  }

  setStatus("Store loaded");
}

async function init() {
  readTokenFromUrl();

  setStatus("Connecting…");
  const st = await api("/api/v1/status");
  qs("#repo-pill").textContent = "repo: " + st.repo_root;
  qs("#store-pill").textContent = "store: " + st.store_root;
  state.storeRootAbs = st.store_root;
  setMutations(st.mutations_enabled);

  state.kinds = await api("/api/v1/store/kinds");
  state.distros = await api("/api/v1/distro");

  state.selectedDistro = state.distros[0]?.dir || null;
  state.selectedKind = state.kinds[0] || null;

  renderDistroSelect();
  renderKindSelect();

  qs("#distro-select").onchange = async (e) => {
    state.selectedDistro = e.target.value;
    await loadOutputs();
    await loadTree(state.selectedDistro);
  };

  qs("#kind-select").onchange = async (e) => {
    state.selectedKind = e.target.value;
    state.storeOffset = 0;
    await loadStore();
  };

  qs("#refresh-outputs").onclick = async () => {
    await loadOutputs();
    await loadTree(state.selectedDistro);
  };
  qs("#refresh-store").onclick = async () => {
    await loadStore();
  };

  qs("#outputs-filter").oninput = async (e) => {
    state.outputsFilter = e.target.value || "";
    renderOutputs();
  };
  qs("#tree-filter").oninput = async (e) => {
    state.treeFilter = e.target.value || "";
    renderTree();
  };
  qs("#store-filter").oninput = async (e) => {
    state.storeFilter = e.target.value || "";
    renderStore();
  };

  qs("#ingest-all").onclick = async () => {
    if (!state.mutationsEnabled) return;
    const dir = state.selectedDistro;
    if (!dir) return;
    setStatus("Ingesting all existing outputs…");
    await api(`/api/v1/distro/${encodeURIComponent(dir)}/ingest_existing`, {
      method: "POST",
      headers: { "content-type": "application/json", ...tokenHeader() },
      body: JSON.stringify({}),
    });
    await loadOutputs();
    await loadStore();
    setStatus("Ingest complete");
  };

  qs("#restore-missing").onclick = async () => {
    if (!state.mutationsEnabled) return;
    const dir = state.selectedDistro;
    if (!dir) return;
    const summary = state.lastSummary;
    if (!summary) return;
    const missing = summary.artifacts.filter((a) => !a.exists).map((a) => a.kind);
    if (missing.length === 0) {
      setStatus("No missing artifacts to restore");
      return;
    }
    setStatus(`Restoring ${missing.length} missing artifact(s)…`);
    for (const k of missing) {
      await api(`/api/v1/distro/${encodeURIComponent(dir)}/restore`, {
        method: "POST",
        headers: { "content-type": "application/json", ...tokenHeader() },
        body: JSON.stringify({ kind: k }),
      });
    }
    await loadOutputs();
    await loadTree(`${dir}`);
    setStatus("Restore missing complete");
  };

  qs("#prune-btn").onclick = async () => {
    if (!state.mutationsEnabled) return;
    const raw = (qs("#prune-keep-last").value || "").trim();
    const keep = Math.max(1, Number(raw || "3"));
    setStatus(`Pruning (keep last ${keep})…`);
    await api("/api/v1/actions/prune", {
      method: "POST",
      headers: { "content-type": "application/json", ...tokenHeader() },
      body: JSON.stringify({ keep_last: keep }),
    });
    await loadStore();
    setStatus("Prune complete");
  };

  qs("#store-prev").onclick = async () => {
    state.storeOffset = Math.max(0, state.storeOffset - state.storeLimit);
    await loadStore();
  };
  qs("#store-next").onclick = async () => {
    state.storeOffset = state.storeOffset + state.storeLimit;
    await loadStore();
  };

  qs("#gc-btn").onclick = async () => {
    if (!state.mutationsEnabled) return;
    setStatus("GC running…");
    await api("/api/v1/actions/gc", { method: "POST", headers: { ...tokenHeader() } });
    await loadStore();
    setStatus("GC complete");
  };

  if (state.selectedDistro) {
    await loadOutputs();
    await loadTree(state.selectedDistro);
  }
  if (state.selectedKind) {
    await loadStore();
  }

  setStatus("Ready");
}

init().catch((e) => {
  console.error(e);
  setStatus("ERROR: " + String(e?.message || e));
});

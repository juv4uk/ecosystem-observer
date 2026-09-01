const invoke = window.__TAURI__?.core?.invoke
const state = { snapshot: null, timer: null }

const $ = (id) => document.getElementById(id)
const text = (tag, value, className) => {
  const node = document.createElement(tag)
  node.textContent = value
  if (className) node.className = className
  return node
}

function badge(value, kind = "unknown") {
  return text("span", value, `badge ${kind}`)
}

function metric(label, value) {
  const node = document.createElement("div")
  node.className = "metric"
  node.append(text("span", label), text("strong", String(value)))
  return node
}

function statusKind(status) {
  if (status === "Complete" || status === "Fresh") return "ok"
  if (status === "Partial" || status === "Stale") return "partial"
  if (status === "Failed" || status === "Orphaned") return "failed"
  return "unknown"
}

function renderSummary(snapshot) {
  const complete = snapshot.repositories.filter((repo) => repo.scan_status === "Complete").length
  const dirty = snapshot.repositories.filter((repo) => repo.git?.is_dirty).length
  const live = snapshot.local_processes.filter((process) => process.os_observed).length
  const swarm = snapshot.swarm_node.live_instances.length
  $("summary").replaceChildren(
    metric("Репозиторії прочитано", `${complete}/${snapshot.repositories.length}`),
    metric("Dirty worktrees", dirty),
    metric("Живі процеси", live),
    metric("swarm-node", swarm),
  )
}

function renderRepositories(snapshot) {
  const filter = $("repo-filter").value.trim().toLowerCase()
  const list = $("repositories")
  list.replaceChildren()
  for (const repo of snapshot.repositories.filter((item) => item.name.toLowerCase().includes(filter))) {
    const card = document.createElement("article")
    card.className = "card"
    const head = document.createElement("div")
    head.className = "card-head"
    const title = document.createElement("div")
    title.append(text("div", repo.name, "card-title"), text("div", repo.path, "mono"))
    head.append(title, badge(repo.scan_status, statusKind(repo.scan_status)))
    card.append(head)
    if (repo.git) {
      const dirty = repo.git.is_dirty ? "DIRTY" : "CLEAN"
      const details = [
        `branch  ${repo.git.branch ?? "unknown"}${repo.git.is_detached ? " (detached)" : ""}`,
        `head    ${repo.git.head_sha ?? "unknown"}`,
        `state   ${dirty}`,
      ]
      if (repo.git.changed_paths.length) details.push(`paths   ${repo.git.changed_paths.join(", ")}`)
      if (repo.git.unavailable.length) details.push(`unknown ${repo.git.unavailable.map((item) => item.probe).join(", ")}`)
      card.append(text("div", details.join("\n"), "mono"))
    } else {
      card.append(text("div", repo.error ?? "No Git state", "mono failed"))
    }
    list.append(card)
  }
  if (!list.children.length) list.append(text("p", "Немає репозиторіїв за цим фільтром.", "empty"))
}

function renderProcesses(snapshot) {
  const list = $("processes")
  list.replaceChildren()
  for (const process of snapshot.local_processes) {
    const card = document.createElement("article")
    card.className = "card"
    const observed = process.os_observed
    const claimed = process.identity
    const name = claimed.instance ?? observed?.command?.split(" ")[0] ?? `PID ${process.pid}`
    const head = document.createElement("div")
    head.className = "card-head"
    head.append(text("div", name, "card-title"), badge(process.identity_status, statusKind(process.identity_status)))
    const details = [
      `pid       ${process.pid}`,
      `repo      ${observed?.repo_association ?? claimed.repository_identity ?? "unknown"}`,
      `model     ${claimed.model ?? "unknown / not reported"}`,
      `task      ${claimed.task ?? "unknown / not reported"}`,
      `command   ${observed?.command ?? "not observed"}`,
    ]
    card.append(head, text("div", details.join("\n"), "mono"))
    list.append(card)
  }
  if (!list.children.length) list.append(text("p", "Живих або orphaned agent records не спостережено.", "empty"))
}

function renderGuard(snapshot) {
  const root = $("guard")
  root.replaceChildren()
  const guard = snapshot.guard
  if (!guard) {
    root.append(text("p", "Не налаштовано: unknown, не false.", "empty"))
    return
  }
  root.append(badge(guard.status, statusKind(guard.status)))
  root.append(text("div", `source  ${guard.source_path}\nguard-ask  ${guard.canonical_entry_point_present ?? "unknown"}\ntopics  ${guard.topics.length}`, "mono"))
  const topics = document.createElement("div")
  topics.className = "topic-list"
  for (const topic of guard.topics) topics.append(text("span", topic, "topic"))
  root.append(topics)
  const legacy = guard.legacy_paths.map((item) => `${item.exists ? "present" : "absent"}  ${item.path}`).join("\n")
  root.append(text("div", `\nlegacy presence only:\n${legacy || "none configured"}${guard.error ? `\nerror: ${guard.error}` : ""}`, "mono"))
}

function renderSwarm(snapshot) {
  const root = $("swarm")
  root.replaceChildren(badge(snapshot.swarm_node.status, statusKind(snapshot.swarm_node.status)))
  if (!snapshot.swarm_node.live_instances.length) {
    root.append(text("p", "Жодного живого процесу не спостережено.", "empty"))
  }
  for (const node of snapshot.swarm_node.live_instances) {
    root.append(text("div", `pid ${node.pid}\n${node.command}\ncwd ${node.cwd ?? "unknown"}`, "mono"))
  }
  root.append(text("div", snapshot.swarm_node.note, "mono"))
}

function render(snapshot) {
  state.snapshot = snapshot
  renderSummary(snapshot)
  renderRepositories(snapshot)
  renderProcesses(snapshot)
  renderGuard(snapshot)
  renderSwarm(snapshot)
  $("status-line").textContent = `snapshot ${snapshot.scan.scan_id} · ${snapshot.scan.finished_at}`
}

async function refresh() {
  const button = $("refresh")
  button.disabled = true
  $("status-line").textContent = "Читаю реальний стан…"
  try {
    if (!invoke) throw new Error("Tauri bridge unavailable: запустіть застосунок через Tauri, не як звичайну web-сторінку")
    render(await invoke("get_ecosystem_snapshot"))
  } catch (error) {
    $("status-line").textContent = `SCAN FAILED · ${error?.message ?? String(error)}`
  } finally {
    button.disabled = false
  }
}

function configureTimer() {
  if (state.timer) clearInterval(state.timer)
  state.timer = $("auto-refresh").checked ? setInterval(refresh, 15_000) : null
}

$("refresh").addEventListener("click", refresh)
$("repo-filter").addEventListener("input", () => state.snapshot && renderRepositories(state.snapshot))
$("auto-refresh").addEventListener("change", configureTimer)
configureTimer()
refresh()

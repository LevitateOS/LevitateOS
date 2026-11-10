# levitate-agent

The three desktop apps (`gui/recipe`, `gui/recab`, `gui/disk`) hand their
request to an agent and read what it reports. A real coding agent such as
Codex does not speak that protocol. `levitate-agent` is what the apps start
instead: it speaks the apps' protocol on one side and the agent's own on the
other, keeps a record of every run, and runs the commands the agent is not
allowed to run itself, after the person has said yes.

C++20, QtCore only (no widgets), CMake. Binary: `levitate-agent`.

## The protocol toward the apps

The same as `gui/kit/run.cpp` and `gui/kit/work.cpp` read and write.

- **Standard input**, JSON lines. The first: `{"prompt": "...", "app": "...",
  "title": "..."}` (`app` and `title` may be missing). Later ones answer a
  question: `{"question": ID, "answer": "..."}`. The input stays open.
- **Standard error**, one JSON object per line:

| Event | Fields |
|---|---|
| `plan` | `steps`: pairs of id and title |
| `step` | `step` (an id of the plan), `status` (`running` or `done`) |
| `say` | `text` |
| `output` | `line` |
| `agent` | `summary`: the agent's last word, shown as it is |
| `question` | `id`, `text`, `choices` (empty: the person types an answer) |
| `failed` | `reason` |
| `done` | |
| `did` | `kind` (`command`, `write` or `search`), `what`, `exit_code` (a number or null), `as_administrator`, `approved` |

`did` is new and today's apps ignore it. It is one event for each thing the
agent did, for a page that will list the activity. A `question` that asks to
run a command also has `kind` (`administrator` or `command`) and `command`;
today's apps show only `text` and `choices`, and `text` holds the command and
the reason.

Exit code: 0 when done, 1 when failed, 143 when stopped. On SIGTERM, SIGINT or
SIGHUP the agent and everything it started (its whole process group) are
killed, the record gets its last line `stopped`, and `levitate-agent` exits.

## Which agent

1. `LEVITATE_AGENT` in the environment, else
2. `"agent"` in `~/.config/levitate/agent.json`, else
3. `codex`.

`codex` uses the Codex adapter below. Any other value is the path of a program
that already speaks the protocol above: its input and output pass straight
through (the record is still written). Claude Code is not here yet; its
adapter is another class beside `src/codex.cpp`, chosen in `src/main.cpp`.

`~/.config/levitate/agent.json`:

```json
{"agent": "codex", "run_commands": "ask"}
```

`run_commands` is `"ask"` (default) or `"allowed"`. With `"ask"` Codex works in
its read-only sandbox, and a command it needs run is put to the person first.
With `"allowed"` Codex gets its `workspace-write` sandbox in the home folder,
so it may change files there itself. That is all it widens: a `RUN:` or
`RUN-AS-ADMINISTRATOR:` request is the agent asking for something outside its
sandbox, run unsandboxed as the person, so it is always asked.

| Environment | Meaning |
|---|---|
| `LEVITATE_AGENT` | the agent, as above |
| `LEVITATE_AGENT_CODEX` | the `codex` program (default `codex`); the checks put the stand-in here |
| `LEVITATE_AGENT_ROOT_HELPER` | what runs an approved administrator command (default `pkexec`); set and empty runs it directly, for checks |
| `XDG_CONFIG_HOME`, `XDG_STATE_HOME` | where the settings and the records are |

## The Codex adapter

Each turn is one `codex exec --json --skip-git-repo-check -s read-only -C
<home> -` (or `workspace-write`), the prompt on standard input. The first turn
sends a preamble (in `src/codex.cpp`; it is the contract with the agent) and
the app's prompt. After a question or a command the same session continues
with `codex exec resume <thread_id> --json --skip-git-repo-check -c
sandbox_mode="..." -`: `resume` accepts neither `-s` nor `-C`, so the sandbox
goes in as configuration and the folder is the one it is started in.

What Codex prints becomes events:

| Codex | Event |
|---|---|
| `todo_list` | `plan` when the wording of the list changes; `step` (completed: `done`, first open one: `running`) |
| `command_execution` | `output` `$ command`; when finished its output (at most 30 lines) and a `did` |
| `file_change` finished | an `output` line and a `did` for each path |
| `web_search` | `did` |
| `agent_message` | `say` (the convention lines below are left out) |
| `reasoning` | nothing |
| `turn.failed` | `failed` |
| `error` | remembered; `failed` if the turn then ends without `turn.completed` (Codex also reports trouble it gets over) |
| child ends without `turn.completed` | `failed` with the tail of its error output; a sign-in problem says so and names `codex login` |

When the turn completes, the agent's last message is read. It may end with one
line, which is how the agent talks to the person:

- `ASK: question | choice | choice`: a `question`; the answer is the next prompt.
- `RUN-AS-ADMINISTRATOR: command`, then optionally `BECAUSE: one sentence`: a
  `question` with choices `Approve` and `Refuse`. Approve runs it through the
  root helper, sends `output` and a `did` with `as_administrator` and
  `approved` true, and tells the agent the exit code and the output. Refuse
  sends a `did` with `approved` false and tells the agent the person refused.
- `RUN: command`: the same, run as the person.

A command is split into arguments here and run without a shell. When it needs
one (`|`, `;`, `&`, `>`, `<`, a backtick, `$(`, a line break) the person is not
asked: the agent is told to name one plain command. With none of these lines
the message is the `summary` of an `agent` event and the run is `done`. A run
that goes back and forth more than 30 turns is failed.

The list is re-sent as a `plan` only when its wording changes; the apps
replace the steps shown when a `plan` arrives.

## The record

`~/.local/state/levitate/agent/runs/<UTC time>-<id>.jsonl`, one file for each
run, one JSON object per line, written as it happens:

1. `{"record": "request", "id", "t", "app", "title", "prompt", "agent", "pid"}`
2. each event sent to the app, with `"t"`; when the person answered a question,
   `{"event": "answer", "id", "answer", "t"}` as well (not sent to the app);
3. `{"record": "end", "t", "state"}` with `state` `done`, `failed` or `stopped`.

A run whose last event is a `question` is waiting; with no last line and no
process left (killed hard) it is listed as `stopped`.

- `levitate-agent --list` prints `{"runs": [...]}`, newest first (at most 200),
  each with `id`, `app`, `title`, `state` (`running`, `waiting`, `done`,
  `failed`, `stopped`), `started`, `ended` (null while going), `summary` (the
  agent's last word, or the reason it failed).
- `levitate-agent --show ID` prints that run's lines as a JSON array.
- `levitate-agent --status` prints `{"chosen": "...", "agents": [{"id", "name",
  "installed", "version", "signed_in"}]}` from `codex --version` and `codex
  login status` (exit 0 and "Logged in using ..." when signed in). `signed_in`
  is null when it could not be asked. A chosen program that is not `codex` is
  listed too.

## Build and check

```sh
cmake -S gui/agent -B .artifacts/agent/build -G Ninja
cmake --build .artifacts/agent/build
gui/agent/fixtures/story
```

It is also part of `gui/CMakeLists.txt`; `cmake --install` puts it in `bin`.

`fixtures/codex` is a stand-in for `codex`: it answers `--version`, `login
status`, `exec` and `exec resume`, and prints the JSON lines in the shapes of
`codex-rs/exec/src/exec_events.rs`, by story (`LEVITATE_STANDIN_STORY`).
`fixtures/story` runs levitate-agent through every story (plain, asks,
administrator approved and refused, a plain command, a command with a pipe,
failing, signed out, dying, stopping with SIGTERM, an agent that speaks the
protocol itself), also `--list`, `--show` and `--status`, in a scratch
folder, and prints a line for each expectation that does not hold. It ends with
`all flows ended where they should` or the opposite, and the exit code agrees.
There are no unit tests (project rule, `AGENTS.md`).

## Not done

- **Two requests have run against the real Codex (0.167, 2026-11-10).**
  "Say hello" went through as `say`, `agent` and `done`. A second one made
  Codex ask a question (`ASK:` came through as `question`), took the answer,
  resumed the same session (so `exec resume` takes `-c sandbox_mode=...`),
  ran `uname -r` in its read-only sandbox (shown as `output` and one `did`)
  and ended with `done`. Still checked against the stand-in only: `RUN:` and
  `RUN-AS-ADMINISTRATOR:` with Approve and Refuse, the real `pkexec`, a todo
  list as steps, file changes, a turn that fails, and what a sign-in failure
  prints.
- **A `did` for a command the agent ran itself in its sandbox says
  `"approved": true`**, though nobody was asked. It should say that it was
  not asked about; whoever builds the page that reads the record should
  settle the field.
- **No Claude Code adapter.**
- **A run ends with the window that started it.** There is no background
  service; closing the window closes the run (the record says `stopped`).
- **"Ask each time" for the agent's own commands depends on the agent
  following the `RUN` convention.** In the read-only sandbox it cannot run
  them itself, but nothing here makes it ask rather than give up.

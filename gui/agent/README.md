# levitate-agent

The three desktop apps (`gui/recipe`, `gui/recab`, `gui/disk`) hand their
request to an agent and read what it reports. A real coding agent such as
Codex does not speak that protocol. `levitate-agent` is what the apps start
instead: it speaks the apps' protocol on one side and the agent's own on the
other, keeps a record of every run, and runs the commands the agent is not
allowed to run itself, after the person has said yes.

C++20, QtCore only (no widgets), CMake. Binary: `levitate-agent`.

## The life of a run

A run does not belong to the window that started it.

- `levitate-agent` started the usual way (the request on standard input) is a
  **client**: it starts the run in a process of its own (`levitate-agent --host`,
  a session of its own, with nothing of the window's open, not for people to
  run) and relays between the window and that process. Its standard input and
  error carry exactly what they always carried; while the window is open
  nothing is different.
- The **run** lives in the host process until it ends. The host writes the
  record, runs the agent, and listens on a Unix socket for clients. When the
  run ends the process ends: there is no service, nothing to install, and
  nothing left when no run is going.
- **The socket** is `$XDG_RUNTIME_DIR/levitate-agent/ID.sock`, made for the
  person alone; with no `XDG_RUNTIME_DIR` it is `levitate/agent/sockets/ID.sock`
  under the state folder. The record stays the source of truth for replay,
  `--list` and `--show`; the socket only carries what happens from now on.
- **A client going away does not end the run.** A stop has to be asked for:

| What happens to a client | The run |
|---|---|
| its standard input closes (the window will not answer any more) | goes on; the client stays and relays until the run ends and exits as the run does (0, 1, 143), **except** that when the run waits for an answer the client has nobody to get one from, so it leaves with exit code 75 and the run waits |
| it is killed (SIGKILL), or gets SIGHUP | goes on (SIGHUP: the client exits 129) |
| it gets **SIGTERM** or SIGINT | **is stopped**: the agent and everything it started end, the record says `stopped`, the client exits 143. This is what the apps' "Stop" does. So an app that wants a run to outlive its window closes the input (or just exits); it must not terminate the client |
| `levitate-agent --stop ID` | is stopped, the same way; prints `{"id": ID, "state": "stopped"}` and exits 0 (a run that has already ended is said so on standard error, exit 0; an unknown id exits 1) |
| a run waits for an answer and no client is attached | waits, for as long as it takes |
| the host process is killed hard | is over (`--list` says `stopped`; what the agent started may still run) |

- **What the apps do** (`gui/kit/run.*`): a window that closes lets go of its run
  by closing the input and hanging up (SIGHUP), never SIGTERM, and its status line
  says the run carries on; "Stop" in a window is `--stop ID` (the id is the one in
  the client's first line, `run`; a client that never named one, a stand-in, is
  asked to end with SIGTERM instead); an exit code 75 is not shown as a failure.
  Activity attaches to every run that is going (`attachable`), answers its
  questions through the attached client, and lets go (SIGHUP) when the run is not
  shown any more.
- `levitate-agent --attach ID` is a client of a run that is still going. It
  first replays the events so far from the record, then follows the run, and
  takes answers on standard input like the usual client. It reads the events
  **as the record holds them** (with their time `t`), and also the `answer`
  events (`{"event": "answer", "id", "answer", "t"}`, which the usual client
  never gets). Two or more clients may be attached at once: all see every
  event. A question is answered once: whoever answers first is heard, a later
  answer to it is ignored. SIGTERM to an attached client stops the run, as for
  any client. An id that is unknown, or a run that has ended, exits 1 with one
  sentence (read an ended run with `--show`).
- The client the window starts reads events as it always did: the host sends it
  the lines exactly as they were sent to the apps (without the time, and without
  `answer` events), and a program agent's own standard output too.
- The client's exit code is the run's: 0 done, 1 failed, 143 stopped.

## Notices when no window is watching

A run that needs the person and has no client attached shows one desktop
notice: when it asks a question or wants an approval (also when the last client
leaves while the question waits), and when it ends. The text is the run's title
(its app's name when it has none) and what is true: `waits for an answer`, `is
done: <summary>`, `failed: <reason>`, `stopped: <reason>` (`it was asked to stop`
or `it was stopped from a window`). A client that is attached means no notice;
a window that started the run and has not yet connected (five seconds at most) is
counted as attached.

The notice goes through the session bus, as `org.freedesktop.Notifications.Notify`,
by **`gdbus call`**. `gdbus` is part of GLib's tools, which Plasma and LXQt both
need already; `notify-send` is in libnotify, which neither depends on, and
QtDBus would add a library to a program that is QtCore only. A notice that
cannot be shown (no bus, no `gdbus`) is not an error. The setting
`"notices": true|false` in `agent.json` (default true) turns them off
(`--settings`, `--set notices=false`).

For checks, `LEVITATE_AGENT_NOTIFIER` names a program that is run with the title
and the text as its two arguments instead of `gdbus`; set but empty, nothing is
shown. `fixtures/notifier` is the stand-in that writes down what it was asked.
**A check that drives the helper for real should set it** (to empty, or to that
stand-in), or a client that leaves a waiting run shows a notice on the desktop of
whoever runs the check.

## The protocol toward the apps

The same as `gui/kit/run.cpp` and `gui/kit/work.cpp` read and write.

- **Standard input**, JSON lines. The first: `{"prompt": "...", "app": "...",
  "title": "...", "may_write": ["/absolute/folder", ...]}` (`app`, `title` and
  `may_write` may be missing). Later ones answer a question: `{"question": ID,
  "answer": "..."}`. The input may stay open, or be closed (see the life of a
  run).
- **`may_write`** names folders the agent may write in although `run_commands`
  is `"ask"`, which is what lets an app have a recipe file written. With it (and
  `"ask"`), Codex runs in its `workspace-write` sandbox with the first folder as
  its working root and the others added; Claude Code runs in `acceptEdits` mode
  in the first folder with the others as `--add-dir`. Without it the agent can
  only read, in the home folder. `run_commands: "allowed"` keeps meaning the
  home folder, whatever `may_write` says (every folder must lie in it anyway).
  A folder must be a full path of a folder that exists inside the person's home
  folder (a link that leads out of it is outside). Anything else, or a
  `may_write` that is not a list, fails the whole run at once: a `failed` event
  with a plain reason (`The folder /etc is outside your home folder, so the agent
  is not allowed to write there.`), exit code 1, a record, and the agent is not
  started. The list as given is recorded in the first line of the record.
- **`set_aside`** (optional, in the first line) hands the helper a list of places to
  move, so that the agent never needs the right to write in the home folder for it:
  `"set_aside": {"holding": "/abs/holding/folder", "places": ["/abs/path", ...],
  "ask_first": ["/abs/path", ...]}` (`ask_first` may be missing; each must also be in
  `places`). The holding folder and every place must lie inside the person's home
  folder with the links of their folders resolved (a place may itself be a link; one
  that leads out is refused later, below), the holding folder and a place may not lie
  one inside the other, and the home folder itself is no place. Anything else, or a
  `set_aside` of the wrong shape, fails the whole run at once with a plain reason
  (`The holding folder /tmp/h is outside your home folder, so nothing may be set aside
  there.`), exit code 1, a record, and the agent is not started. The request is
  recorded in the first line of the record. With a program as the agent (the
  pass-through) the line goes through as it is and nothing is moved here.
- **Standard error**, one JSON object per line:

| Event | Fields |
|---|---|
| `run` | `id`: the run's id. The client sends it first of all, before the run has a record, so that the window can recognise its run in `--list` and stop it with `--stop ID`. Not in the record, and never sent by an attached client (it knows the id) |
| `plan` | `steps`: pairs of id and title |
| `step` | `step` (an id of the plan), `status` (`running` or `done`) |
| `say` | `text` |
| `output` | `line` |
| `agent` | `summary`: the agent's last word, shown as it is |
| `question` | `id`, `text`, `choices` (empty: the person types an answer) |
| `failed` | `reason` |
| `done` | |
| `place` | `path` (as given in `places`), `outcome` (`set-aside`, `kept` or `could-not`), `why`, `bytes` (the size moved, else 0), and `asked` and `answer` when the person was asked about it in this run. Only with `set_aside` (below) |
| `did` | `kind` (`command`, `write` or `search`), `what`, `exit_code` (a number or null), `as_administrator`, `asked` (whether the person was asked), `approved` (true or false when asked, null when not) |

`did` is read by the Activity page in `gui/kit`; the other pages ignore it. It is one event for each thing the
agent did, for a page that will list the activity. A `question` that asks to
run a command also has `kind` (`administrator` or `command`) and `command`;
today's apps show only `text` and `choices`, and `text` holds the command and
the reason.

Exit code: 0 when done, 1 when failed, 143 when stopped (and 75, 129 or 141 for a
client that leaves a run that goes on: see the life of a run). On SIGTERM or
SIGINT the run is stopped: the agent and everything it started (its whole
process group) are killed, the record gets its last line `stopped`, and
`levitate-agent` exits 143.

## Which agent

1. `LEVITATE_AGENT` in the environment, else
2. `"agent"` in `~/.config/levitate/agent.json`, else
3. `codex`.

`codex` uses the Codex adapter, `claude` the Claude Code adapter (both below).
Any other value is the path of a program that already speaks the protocol
above: its input and output pass straight through (the record is still written;
what it writes on its own standard output is sent to the window that started the
run). Each adapter is a class beside `src/conversation.cpp`, chosen in
`src/main.cpp`.

`~/.config/levitate/agent.json`:

```json
{"agent": "codex", "run_commands": "ask", "notices": true}
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
| `LEVITATE_AGENT_CLAUDE` | the `claude` program (default `claude`); likewise |
| `LEVITATE_AGENT_NOTIFIER` | what shows a notice, as above |
| `LEVITATE_AGENT_ROOT_HELPER` | what runs an approved administrator command (default `pkexec`); set and empty runs it directly, for checks |
| `XDG_CONFIG_HOME`, `XDG_STATE_HOME` | where the settings and the records are |
| `XDG_RUNTIME_DIR` | where the sockets are (`levitate-agent/` in it) |

## The Codex adapter

Each turn is one `codex exec --json --skip-git-repo-check -s read-only -C
<home> -` (or `workspace-write`; with `may_write` and `"ask"`: `-s workspace-write
-C <first folder> --add-dir <other>...`), the prompt on standard input. The first turn
sends a preamble (in `src/conversation.cpp`; it is the contract with the agent, shared with Claude Code) and
the app's prompt. After a question or a command the same session continues
with `codex exec resume <thread_id> --json --skip-git-repo-check -c
sandbox_mode="..." -`: `resume` accepts neither `-s` nor `-C`, so the sandbox
goes in as configuration and the folder is the one it is started in. The
folders beyond the first go in as `-c 'sandbox_workspace_write.writable_roots=[...]'`
there (the `--add-dir` of the first turn has no counterpart on `resume`).

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

## Setting places aside: the agent decides, the helper moves

With `set_aside` in the request, the contract the agent gets (`src/conversation.cpp`,
for both adapters) adds three lines, on lines of their own, as many as it likes in one
message, and lists the approved places and the ones to ask about first:

- `SET-ASIDE: /path`: move this place into the holding folder.
- `KEEP: /path | why in a few words`: the agent decided to leave it.
- `CANNOT: /path | why`: it should go but something stops it.

For each `SET-ASIDE:` the **helper** moves the place (`src/holding.*`), nobody being
asked per move, because the person approved the list before the run: only when the path
is exactly one of `places` (after resolving the links of its folders; a trailing slash
or `..` is cleaned first), into the holding folder keeping its path beneath it
(`/home/me/.cache/x` goes to `<holding>/home/me/.cache/x`), by `renameat2` with
`RENAME_NOREPLACE` relative to descriptors it checked (`/proc/self/fd`) to be the
folders it resolved, so a link swapped in meanwhile is not followed, without a shell
and without a copy: a place on another disk is `could-not`. A place that is a link
leading out of the home folder (or to nothing) is `could-not` and not moved. The holding
folder is made when it is missing (mode 0700). The manifest `gui/disk` reads
(`{"version": 1, "entries": [{"from", "held", "bytes"}]}`, `bytes` the length of what
was moved) is written after every move, and a move whose record cannot be written is put
back. Then it sends `place` and a `did` (`write`, nobody asked) and tells the agent at
the next turn, for each line: `done`, `not done, <why>`, or `refused`. `KEEP:` and
`CANNOT:` only send `place` (`kept`, `could-not`) and are `noted`.

Refused with nothing moved and no `place` event, the agent being told so (it may try
again): a path that is not on the list; a place of `ask_first` that the person was not
asked about in this run. A place counts as asked about when the agent's `ASK:` named
its full path (or the path with `~` for the home folder) and the person answered; the
`place` event then carries `asked` and the `answer`. What the person answered is not
read: the agent decides what a "yes" is. A place answered for once is not moved again
(`ignored`). A message with `SET-ASIDE:` lines and nothing else is followed by one more
turn with the results (`Carry on with any place you have not answered for, or finish
now`); with `ASK:` or `RUN:` the results come first in the turn that follows the answer.
Without `set_aside` these lines mean nothing and are left in the agent's text. Moving a
big folder blocks the helper's event loop while its size is counted.

## What both adapters share

The adapters are two classes over one (`src/conversation.cpp`): a run is a series
of turns, each one a start of the agent that ends with a last message.

When a turn completes, the agent's last message is read. It may end with one
line, which is how the agent talks to the person:

- `ASK: question | choice | choice`: a `question`; the answer is the next prompt.
- `RUN-AS-ADMINISTRATOR: command`, then optionally `BECAUSE: one sentence`: a
  `question` with choices `Approve` and `Refuse`. Approve runs it through the
  root helper, sends `output` and a `did` with `as_administrator`, `asked` true and
  `approved` true, and tells the agent the exit code and the output. Refuse
  sends a `did` with `asked` true and `approved` false and tells the agent the person refused.
- `RUN: command`: the same, run as the person.

A command is split into arguments here and run without a shell. When it needs
one (`|`, `;`, `&`, `>`, `<`, a backtick, `$(`, a line break) the person is not
asked: the agent is told to name one plain command. With none of these lines
the message is the `summary` of an `agent` event and the run is `done`. A run
that goes back and forth more than 30 turns is failed.

The list is re-sent as a `plan` only when its wording changes; the apps
replace the steps shown when a `plan` arrives.

## The Claude Code adapter

Chosen with `agent: "claude"` (the program is `LEVITATE_AGENT_CLAUDE`, else
`claude`). Written from Anthropic's documentation; **it has never run against a
real Claude Code** (none is installed on the machine it was written on), only
against `fixtures/claude`, which prints what the documentation says.

Each turn is one `claude -p --output-format stream-json --verbose ...`, the
prompt on standard input, in the home folder (or the first `may_write` folder).
The contract (the same text as Codex gets, and a short note on refused tools)
goes in `--append-system-prompt`, again on every turn. After a question or a
command the session continues with `--resume <session_id>`, the id from the
`system`/`init` message. The permissions, as far as `run_commands` and
`may_write` allow:

| | flags |
|---|---|
| `"ask"` (default), no `may_write` | `--permission-mode dontAsk --allowedTools Read,Glob,Grep,WebSearch,WebFetch --disallowedTools Edit,Write,NotebookEdit`: it reads and looks things up; shell commands outside Claude Code's own read-only set and every write are refused, and the agent must ask through the three lines |
| `"ask"` with `may_write` | `--permission-mode acceptEdits --allowedTools Read,Glob,Grep,WebSearch,WebFetch` in the first folder, `--add-dir` for each other: files change inside those folders only |
| `"allowed"` | the same as with `may_write`, in the home folder |

Shell commands are never allowed to Claude Code beyond its read-only set (not
even with `"allowed"`: that setting widens files, as it says above). It asks
with `RUN:` and the person answers. This is narrower than Codex's
`workspace-write` sandbox, which runs commands inside its sandbox; there is no
sandbox here to run them in. `--bare` is not used (it ignores the person's
sign-in), so a `claude -p` run loads the hooks, skills, MCP servers and
CLAUDE.md of an interactive session; and `--permission-prompts none` is not used
(it needs Claude Code 2.1.259 or later).

What Claude Code prints becomes events (one JSON object a line on standard output):

| Claude Code | Event |
|---|---|
| `system` / `init` | remembered: `session_id` |
| `system` / `permission_denied` | `output` `not allowed: <command or tool and file>`; the refused call is not a `did` |
| `assistant`, `text` block | `say` (the convention lines are left out) |
| `assistant`, `tool_use` `Bash` | `output` `$ command`; at its `tool_result` the output (at most 30 lines) and a `did` (exit code 0 when the result is not an error, else unknown) |
| `tool_use` `TodoWrite` | `plan` when the wording changes; `step` (`completed`: `done`, `in_progress`: `running`) |
| `Edit`, `Write`, `NotebookEdit` with a successful `tool_result` | an `output` line (`write` or `update`, the path) and a `did` |
| `WebSearch`, `WebFetch` with a successful `tool_result` | `did` (kind `search`; the query or the address) |
| `thinking`, other tools, messages of subagents | nothing |
| `result`, `subtype` `success`, `is_error` false | the turn is over; its `result` text is the last message (for `ASK:` and the others, and the `summary`) |
| `result` with `is_error` true, or an `error_*` subtype | `failed`: the `result` text, or the `errors`, or the `subtype`; a sign-in problem says so and says to run `claude` once in a terminal |
| child ends with no `result` | `failed` with the tail of its error output |

After a `result` the process may stay open for background work; it is given ten
seconds to end by itself, then ended.

**Sign-in and status.** `--status` lists Claude Code (`id` `claude`) with `claude
--version` and `claude auth status` (exit code 0 when signed in, 1 when not;
`signed_in` is null when it could not be asked). `--sign-in` and `--sign-out`,
with `claude` as the chosen agent, run `claude auth login` and `claude auth
logout`, passing what they print on standard error, like Codex's.

**The lines of Anthropic's documentation the adapter rests on** (fetched
2026-11-11; the pages are `code.claude.com/docs/en/headless`, `cli-reference`,
`permission-modes` and `agent-sdk/typescript`):

- Print mode, the prompt on standard input: "To run Claude Code in
  non-interactive mode, pass `-p` with your prompt and the CLI options you need";
  "Non-interactive mode reads stdin" (`gh pr diff "$1" | claude -p
  --append-system-prompt "..." --output-format json`, no prompt argument).
- Streaming: "Use `--output-format stream-json` with `--verbose` and
  `--include-partial-messages` to receive tokens as they're generated. Each line
  is a JSON object representing an event"; "The last line of the stream is a
  `result` message with the final response text, cost, and session metadata."
  (`--verbose` is in every `stream-json` example; partial messages are not asked
  for.)
- First message: "The `system/init` event reports session metadata including the
  model, tools, MCP servers, and loaded plugins. It is the first event in the
  stream unless startup events precede it."
- Resuming: "use `--resume` with a session ID to continue a specific
  conversation" (`claude -p "Continue that review" --resume "$session_id"`;
  `session_id` is a field of the messages). CLI reference: `--resume`, `-r`:
  "Resume a specific session by ID or name".
- Permissions in print mode: "`claude -p` and Agent SDK runs with no permission
  mode set start in Manual mode ... Calls needing approval are denied unless you
  approve them with an allow rule, hook, `canUseTool` callback, or
  `--permission-prompt-tool`."; `dontAsk`: "Claude Code denies every call that
  would otherwise prompt, which is useful for locked-down CI runs"; "Actions that
  need no approval in Manual mode still run, such as file reads in your working
  directories and the read-only command set"; `acceptEdits`: "Claude writes files
  without prompting ... Apart from the read-only command set, other shell commands
  and network requests still need an `--allowedTools` entry or a
  `permissions.allow` rule", "auto-approval applies only to paths inside your
  working directory or `additionalDirectories`".
- Flags: `--allowedTools`: "Tools that execute without prompting for permission"
  (`--allowedTools "Bash,Read,Edit"`); `--disallowedTools`: "A bare tool name
  removes the matching tools from Claude's context: `"Edit"` removes Edit";
  `--add-dir`: "Add additional working directories for Claude to read and edit
  files"; `--append-system-prompt`: "Append custom text to the end of the default
  system prompt"; `--permission-mode`: "Accepts `default`, `acceptEdits`, `plan`,
  `auto`, `dontAsk`, `bypassPermissions`".
- Denials: "With `--output-format stream-json`, denials appear as
  `permission_denied` system messages" (`SDKPermissionDeniedMessage`:
  `type: "system"`, `subtype: "permission_denied"`, `tool_name`, `tool_use_id`,
  `message`).
- Failures: "When a failure happens inside the run, such as missing
  authentication, Claude Code prints the failure as the result on stdout." The
  result type (`SDKResultMessage`): `type: "result"`, `subtype: "success"` with
  `is_error`, `result`, `session_id`, or `subtype` `error_max_turns` /
  `error_during_execution` / `error_max_budget_usd` with `errors: string[]`.
  An assistant message's `error` may be `authentication_failed`.
- Messages (`SDKAssistantMessage`, `SDKUserMessage`): `type: "assistant"` with
  `message: BetaMessage // From Anthropic SDK`, `parent_tool_use_id: string |
  null` (null for the main conversation; subagents' messages carry the id);
  `type: "user"` with `message: MessageParam`. The blocks inside are those of
  the Anthropic Messages API, which the Claude Code pages do not repeat:
  `{"type": "text", "text"}`, `{"type": "tool_use", "id", "name", "input"}` and,
  in user messages, `{"type": "tool_result", "tool_use_id", "content",
  "is_error"}` (content a string or blocks with `text`). That is from the
  Messages API, not from these pages.
- Tool inputs: `BashInput.command`, `FileEditInput.file_path`,
  `FileWriteInput.file_path`, `NotebookEditInput.notebook_path`,
  `WebSearchInput.query`, `WebFetchInput.url`, `TodoWriteInput.todos[]` with
  `content`, `status` (`pending`, `in_progress`, `completed`), `activeForm`.
- Sign-in: `claude auth status`: "Show authentication status as JSON ... Exits
  with code 0 if logged in, 1 if not"; `claude auth login`, `claude auth logout`.
- `claude --version`'s output is taken as a version number somewhere in it;
  nothing documents its exact wording.

**Not in the documentation, so not relied on:** an exit code for a `Bash`
`tool_result` (hence `did` has 0 or unknown); the Task tools that replace
`TodoWrite` on some models (their steps are not shown); what a sign-in failure
inside a run says word for word (the adapter looks for words like "log in" and
"authentication" in the result).

## The record

`~/.local/state/levitate/agent/runs/<UTC time>-<id>.jsonl`, one file for each
run, one JSON object per line, written as it happens (only by the run's own
process):

1. `{"record": "request", "id", "t", "app", "title", "prompt", "agent", "pid",
   "host"}` and `"may_write"` when the request had it. `pid` is the process
   that asked, the client the window started (it may be gone long before the
   run ends, so nothing matches it any more: a window knows its run by the `id`
   the client named in its first line); `host` is the process the run lives in.
2. each event sent to the app, with `"t"`; when the person answered a question,
   `{"event": "answer", "id", "answer", "t"}` as well (not sent to the app that
   started the run);
3. `{"record": "end", "t", "state"}` with `state` `done`, `failed` or `stopped`.

A run whose last event is a `question` is waiting; with no last line and no
process left (the host killed hard) it is listed as `stopped`.

- `levitate-agent --list` prints `{"runs": [...]}`, newest first (at most 200),
  each with `id`, `app`, `title`, `state` (`running`, `waiting`, `done`,
  `failed`, `stopped`), `started`, `ended` (null while going), `summary` (the
  agent's last word, or the reason it failed), and `attachable`: true while the
  run is going and its socket is there, so that `--attach ID` will work.
- `levitate-agent --show ID` prints that run's lines as a JSON array.
- `levitate-agent --attach ID` and `--stop ID`: see the life of a run.
- `levitate-agent --status` prints `{"chosen": "...", "agents": [{"id", "name",
  "installed", "version", "signed_in"}]}`: Codex (`codex --version`, `codex login
  status`: exit 0 and "Logged in using ..." when signed in) and Claude Code (above),
  always, in that order; `signed_in` is null when it could not be asked. A chosen
  program that is neither is listed too.

`levitate-agent --version` prints `levitate-agent 0.1` (the project version in
`CMakeLists.txt`, passed in as `LEVITATE_AGENT_VERSION`) and exits 0; it starts nothing.

## Settings and sign-in

The helper is the only writer of `agent.json`; the Settings page of the apps
(`gui/kit/settings.*`) asks it.

- `levitate-agent --settings` prints `{"agent": "...", "run_commands": "ask" |
  "allowed", "notices": true | false, "file": "<path of agent.json>"}`: what is
  in force, defaults (and `LEVITATE_AGENT`) included; `file` is where it would be
  written, whether or not it exists yet.
- `levitate-agent --set agent=codex`, `--set agent=claude`, `--set
  agent=/absolute/path`, `--set run_commands=ask|allowed` or `--set
  notices=true|false` checks the value (a path must be an existing executable
  file), writes `agent.json` keeping its other keys (`notices` as a JSON
  boolean), and prints the new settings as `--settings` does. A refused value
  exits 1, writes nothing, and says one sentence on standard error.
- `levitate-agent --sign-in` and `--sign-out` run `codex login` and `codex logout`
  (the program of `LEVITATE_AGENT_CODEX`, else `codex`) and pass what they print
  through, line by line, on standard error (where the apps read progress; standard
  output stays for answers). `codex login` prints an address to open and waits for
  it. The exit code is Codex's. Stopping the helper (SIGTERM) stops Codex too (exit
  143). Codex 0.167 has both commands (`codex login --help`, `codex logout --help`).
  With `claude` chosen they run `claude auth login` and `claude auth logout` the
  same way. When the chosen agent is another program, both exit 1, saying that
  program signs in by itself; when the agent's program cannot be started they say
  it is not installed.

## Build and check

```sh
cmake -S gui/agent -B .artifacts/agent/build -G Ninja
cmake --build .artifacts/agent/build
gui/agent/fixtures/story
```

It is also part of `gui/CMakeLists.txt`; `cmake --install` puts it in `bin`.

`fixtures/codex` is a stand-in for `codex`: it answers `--version`, `login
status`, `login` and `logout` (they keep whether it is signed in in
`$HOME/.standin-codex-signed`, which `login status` follows; `login` prints two lines
including an address and ends after `LEVITATE_STANDIN_LOGIN_SECONDS`, default 0.3;
`LEVITATE_STANDIN_LOGIN_FAILS` makes it fail), `exec` and `exec resume`, and prints the JSON lines in the shapes of
`codex-rs/exec/src/exec_events.rs`, by story (`LEVITATE_STANDIN_STORY`).
`fixtures/claude` is the same for `claude`: `--version`, `auth status|login|logout`
and print mode, printing the shapes quoted above. `fixtures/notifier` records the
notices.

`fixtures/story` runs levitate-agent through every story of both stand-ins (plain,
asks, administrator approved and refused, a plain command, a command with a pipe,
failing, signed out, dying, stopping with SIGTERM, a refused call, writing with
`may_write`, an agent that speaks the protocol itself), and:

- **setting places aside**, with the `script` story of both stand-ins (turn N says
  message N of a JSON list) and the `disk` story of the Codex stand-in (reads the
  Disk window's list): a run that sets aside two approved places (one a folder with
  files in it), keeps one, cannot do one, asks about an `ask_first` place and sets it
  aside after "yes" (files really moved under the holding folder, the manifest exactly
  as Disk reads it, the `place` and `did` events, the agent never given a sandbox that
  writes, one question and none about a move, and what the next turn told the agent),
  for Codex and for Claude Code; a path not on the list, an `ask_first` place not
  asked about and a link leading out of home (each refused, nothing moved, the agent
  told); and the runs that fail at once (a holding folder outside home, behind a link,
  the home folder itself or not a full path; a place outside home, behind a linked
  folder or inside the holding folder; `ask_first` not on the list; a wrong shape),
  none of which starts the agent;
- `--version`;

- a run that survives its client and is finished by an answer from an attached
  client; two clients at once, whose answers count once; a client that leaves by
  closed input, SIGHUP and SIGKILL (the run goes on); SIGTERM to a client and
  `--stop` (the run ends, with what the agent started); `--attach` to a run that
  has ended or does not exist;
- notices: none while a client is attached, one when a run asks or ends with
  nobody watching, one for a stopped run, none with `"notices": false`, and a
  notifier that fails;
- `may_write` accepted (root, other folders, the resumed turn, `"allowed"`) and
  refused (outside the home folder, relative, missing, a link that leads out, one
  bad among good, not a list);
- `--list` (with `attachable`), `--show`, `--status`, `--settings`, each `--set`
  (accepted and refused), `--sign-in` and `--sign-out` for both agents;

in a scratch folder (`.artifacts/agent/story`, or `LEVITATE_STORY_SCRATCH`), and
prints a line for each expectation that does not hold. At the end it checks that
no process of its own and no socket is left. It ends with `all flows ended where
they should` or the opposite, and the exit code agrees.

`fixtures/records STATE-FOLDER` fills a state folder with four runs (done with
commands and a file change, waiting on a question, failed, an administrator
step refused; from Recipe and System) and one that is still working, by driving
the built `levitate-agent` with the stand-in, so that the Activity page of an app
has something to show. The waiting run and the working one keep going without the
helpers that started them (the script lets those go); `fixtures/records --stop
STATE-FOLDER` ends them with `levitate-agent --stop`. It prints
`state=FOLDER/state waiting_id=ID runtime=FOLDER/run`: an app's story gives the
first as `XDG_STATE_HOME` and the last as `XDG_RUNTIME_DIR` (the runs' sockets are
there, which `--attach` needs), and stops the runs when it ends, also when it
fails.
There are no unit tests (project rule, `AGENTS.md`).

## Not done

- **The set-aside convention has never run against a real Codex or Claude Code.**
  Whether an agent writes the `SET-ASIDE:`, `KEEP:` and `CANNOT:` lines as the
  contract asks, and names the full path of an `ask_first` place in its `ASK:` (the
  only way the helper knows it was asked), is checked against stand-ins only. An agent
  that asks without the path is refused its `SET-ASIDE:` and told to ask again.

- **Two requests have run against the real Codex (0.167, 2026-11-10).**
  "Say hello" went through as `say`, `agent` and `done`. A second one made
  Codex ask a question (`ASK:` came through as question), took the answer,
  resumed the same session (so `exec resume` takes `-c sandbox_mode=...`),
  ran `uname -r` in its read-only sandbox (shown as `output` and one `did`)
  and ended with `done`. Still checked against the stand-in only: `RUN:` and
  `RUN-AS-ADMINISTRATOR:` with Approve and Refuse, the real `pkexec` (now from a
  process that has no terminal and is not a child of the window: whether polkit
  finds the person's session from there is unchecked), a todo list as steps, file
  changes, a turn that fails, what a sign-in failure prints, `--add-dir` on a
  first turn, and `sandbox_workspace_write.writable_roots` on a resume.
- **The Claude Code adapter has never run against a real Claude Code.** Every
  flag and message shape is from the documentation quoted above. Unchecked: that
  a `dontAsk` run really lets the read-only commands through, that `acceptEdits`
  scopes writes to the folders given, that the comma-joined `--disallowedTools`
  works (the documentation shows it for `--allowedTools`), the
  `--append-system-prompt` on a resumed session, `claude auth login` without a
  terminal (it may need one), and the exit code and the wording of failures.
- **Nothing here has run on a desktop.** The notice goes through `gdbus call`
  with the arguments of `Notify` as GVariant text (checked to parse with GLib);
  it has not been shown by Plasma's or LXQt's notification server. A click on
  the notice does nothing: it has no actions.
- **A run's agent children outlive a host that is killed hard.** The host ends
  the agent and everything it started when it stops or finishes; `SIGKILL` on the
  host leaves them (`--list` then says `stopped`).
- **"Ask each time" for the agent's own commands depends on the agent
  following the `RUN` convention.** In the read-only sandbox it cannot run
  them itself, but nothing here makes it ask rather than give up.

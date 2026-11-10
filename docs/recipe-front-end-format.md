# Recipe files for the front end: design (2026-11-10, not built)

How a recipe file should change so a graphical front end can show a person what
a recipe is, where it comes from, what it will do and how far it has got,
without running it and without guessing. Nothing below is built.

It covers the `.rhai` file, the few files beside it, and what `recipe` prints
for a front end. It does not cover running Recipe without root or how an agent
writes a recipe; each needs its own design (see "Not in this design").

Corrected the same day: while this was being written, Recipe gained
activation (`tools/recipe` `c815381`): `install` puts a recipe's `package`
output onto the system, and `installed`, `update` and `remove` work from a
receipt. Change 3 below now builds on that instead of proposing its own
output shape.

## What a recipe can say today

Read without running it:

- `name`, `version`, `description`: literal strings in `ctx`
  (`METADATA_FIELDS` in `tools/recipe/src/core/executor.rs`). `recipe info`
  prints these three and nothing else.
- `NEEDS`: network endpoints, limits, input and output names
  (`recipe inspect`).
- `deps` and `build_deps`.

While running, with `--machine-events`: one JSON line when a hook starts,
finishes or is skipped (`recipe-hook`, with the recipe, the hook and a fixed
sentence per hook). Nothing inside a hook is reported except the commands' own
output.

About what is installed, `recipe installed` gives each package's name,
version, recipe path and list of files, and nothing else.

Everything else the draft of the front end shows is absent: the category, the
kind of thing it is, a longer text, the home page, the licence, an icon, where
the source comes from, what the steps do, what lands on the system, progress
inside a step.

## Three kinds of fact

The front end must not present an author's sentence as if the sandbox
guaranteed it. Every fact a recipe shows is one of three kinds, and the front
end marks which:

| Kind | Meaning | Examples |
|---|---|---|
| Enforced | Ginux or Recipe refuses anything else | network endpoints, limits, checksums, the pinned commit |
| Measured | Recipe looked at the result | programs and menu entries in the output, its size |
| Claimed | the author's words, checked only for form | the longer text, the licence, the notes on steps |

Rules that follow:

1. A fact that can be enforced is declared as data and enforced, not described.
2. A fact that can be measured is measured after the build; a claim about it
   made beforehand is checked against the measurement.
3. What is left is a claim, kept in one block so it is easy to see and review.

## Other rules

- **Literal data.** Everything the front end reads is a literal, as `NEEDS`
  is: no helper calls, no variables. It is read from the compiled file without
  running setup or hooks.
- **One home per fact.** `name`, `version` and `description` stay in `ctx`
  because hooks use them. Nothing is declared twice.
- **Optional for OS recipes.** `distro/builder/os-recipes/` needs none of this.
  A catalog of software for people requires it (`recipe validate --catalog`).
- **Small for an agent.** An agent fills in data blocks and writes two or three
  hooks. It should not write download or checksum code.

## Change 1: `ABOUT`, what the recipe says about itself

A new literal block beside `NEEDS`. All of it is claimed, except where noted.

```rhai
const ABOUT = #{
    kind: "app",
    category: "utilities",
    details: "Shows the programs running on this computer and what they use, and lets you stop them.",
    homepage: "https://htop.dev",
    license: "GPL-2.0-or-later",
    adds: #{ programs: ["htop"], menu: ["Htop"] },
    steps: #{
        build: "Compiles htop against the ncurses built just before it.",
        install: "Adds htop to the applications menu, with its manual page and icon.",
    },
    keeps: ["~/.config/htop"],
};
```

| Field | Required | Meaning |
|---|---|---|
| `kind` | yes | `app` (has a menu entry), `tool` (command line), `library` (only for other recipes; never listed to people), `system` (drivers, codecs, fonts) |
| `category` | for `app`, `tool`, `system` | one name from a fixed list (open question 1) |
| `details` | for `app` | one short paragraph, plain text, at most 600 characters |
| `homepage` | yes | the project's own page, `https` |
| `license` | yes | an SPDX expression, or `proprietary`; the front end flags `proprietary` |
| `adds` | for `app`, `tool` | `programs` (names in `usr/bin`) and `menu` (names of menu entries); checked against the output after the build (rule 2) |
| `steps` | no | one sentence per hook the recipe defines (`build`, `install`, `acquire` if it has its own); shown as the author's note beside what the front end writes itself |
| `keeps` | no | paths in the home folder the program writes and removal leaves; lets the remove question name them |
| `after_install` | no | `log-in-again` or `restart`, for drivers and the like; absent means nothing |
| `systems` | no | `levitate`, `acorn` or both; absent means both |

Not in `ABOUT`: the name, version and one-line description (in `ctx`), the
source (change 2), the icon (change 5), sizes and times (measured).

Decided (2026-11-10): the notes on steps are fields, not comments above the
hooks. Recipe would need
a second reader for comments, nothing would check them, and the three existing
comment forms (`//! extends:`, `// AUTOFIX_PROMPT:`) are already scanned by
hand. `validate` can require a note for a hook that exists and refuse one for a
hook that does not.

## Change 2: `SOURCES`, where the code comes from

Today the address, checksum and commit are fields the author names freely in
`ctx`, and each recipe writes its own download and checksum code. The front end
cannot find them and nothing ties a download to what was declared.

A new literal block, enforced:

```rhai
const SOURCES = #{
    htop: #{
        url: "https://github.com/htop-dev/htop/releases/download/{version}/htop-{version}.tar.xz",
        sha256: "a8b164386494cb85bb255a415a3f5f80afe7a0c4491da5d113b3a0f951087e65",
        checksum_from: "publisher",
    },
};
```

or, for a Git source:

```rhai
const SOURCES = #{
    tree: #{ git: "https://github.com/Old-Man-Programmer/tree", commit: "1a41b51bea1cec4109eb9f726370449b384b4576" },
};
```

- `{version}` and `{name}` in a `url` are filled in from `ctx`. A version bump
  is then two edits: `version` and `sha256`.
- `checksum_from` is a claim about the checksum: `publisher` (the project
  publishes it), `signature` (checked against the project's signature) or
  `first-download` (the hash of what the author fetched). The front end says
  which. A Git commit needs none.
- **Recipe fetches the sources itself.** With `SOURCES` present, a recipe may
  leave out `acquire` and `is_acquired`: Recipe downloads each source, checks
  it, and keeps it under `/work/build`. Hooks get at them with
  `source(ctx, "htop")` (the archive or checkout) and `unpack(ctx, "htop")`
  (the unpacked folder).
- **Nothing else may be fetched.** In a recipe with `SOURCES`, `download` and
  `git_checkout` fail for an address that is not declared.
- `validate` checks that each source's host is in `NEEDS.network`, and names
  endpoints no source uses (a redirect target is still listed in `NEEDS`).
- Optional per source: `latest: #{ github_release: "htop-dev/htop" }`, a rule
  for finding the newest version. It lets `recipe` report an outdated recipe
  to a maintainer or an agent without either reading the project's site.

This removes the code where the trial recipes spent about a fifth of their lines
and their one hack (calling `sha256sum` to compare a checksum in a check).

## Change 3: measure what a recipe produces

The shape exists (`recipes/README.md`): a public recipe publishes one directory
output named `package`, holding `bin/COMMAND` and `share/PATH` files, and
`install` activates it into the profile. Recipe already refuses symlinks,
special files and setuid files in it, and records the files in the receipt.

```rhai
const NEEDS = #{ outputs: #{ "package": "directory" }, ... };
```

What the front end still needs on top of that:

- After a successful install Recipe measures `package` and keeps the result
  in the receipt: each menu entry (name, icon and file types, from
  `share/applications/*.desktop`), the size, and the date. Programs are
  already there as the `bin/` files.
- `ABOUT.adds` is checked against that. A recipe that promised `htop` and a
  menu entry `Htop` and produced something else fails.
- A menu entry whose program is not in the tree is refused.
- Desktop programs need more than `bin` and `share` on `PATH`: the profile's
  `share` has to be where the menus, icon themes and file-type database look
  (`XDG_DATA_DIRS`), and their caches refreshed after a change.

For `library`: `sdk` (headers and libraries to build against, as the `ncurses`
trial recipe does), never activated, and `package` as well if programs need
its files when they run.

## Change 4: events while it runs

Today a front end can show which hook is running and a wall of output. Added to
`--machine-events`, one JSON object per line:

| Event | When | Carries |
|---|---|---|
| `plan` | once, before anything runs | every recipe that will run, in order, and the steps each needs |
| `step` | a hook starts, finishes, is skipped (exists as `recipe-hook`) | recipe, hook, status |
| `fetch` | while Recipe downloads a source | source name, bytes so far, total when the server gives one |
| `progress` | a command reports a count | done, total |
| `say` | the recipe calls `say("...")` | one line for a person |
| `output` | a line from a command | the line, and which command |

- `plan` is what lets the front end draw "ncurses, then htop" and "step 4 of 6"
  before the first download.
- `progress` comes from an option on the command helpers,
  `exec_checked("ninja", [...], #{ progress: "counter" })`, which reads
  `[412/697]` prefixes. Recipes for `make` have nothing to count and send none;
  the front end then shows the step as running, with no bar.
- A failed run already ends in one JSON error. It gains the step, the command,
  and for a refused connection the endpoint and the declared list, so the
  "stopped" page needs no second command.

## Change 5: files beside the recipe

A catalog stays one flat folder, as `recipe list` reads today:

```
htop.rhai
htop.svg        the icon, required for kind "app"
ncurses.rhai
repo.toml
```

- `<name>.svg`: shown before the program is installed. After, the front end
  uses the icon the program installed.
- `repo.toml`: the catalog's own name, one line about it, its address and who
  keeps it. The front end shows it in the list of sources.

## Change 6: what `recipe` prints for a front end

- `recipe info NAME --json`: one object with `name`, `version`, `description`,
  `about`, `sources`, `needs`, `deps`, `build_deps`, the hooks the file
  defines, and the file's digest. It replaces calling `info` and `inspect`
  and reading the file.
- `recipe list --json`: the same objects for a whole folder, with a `problems`
  entry for a file that does not compile instead of leaving it out.
- `recipe validate --catalog`: the rules below.

## A whole recipe in this form

`htop` from the second trial. Today's version is 97 lines and tells a front end
three things. This one is 82 lines with the `ABOUT` block included; the
download and checksum code is gone, and the part an agent has to get right as
code is `build` and `install`.

```rhai
const RECIPE_API = 2;

const ABOUT = #{
    kind: "app",
    category: "utilities",
    details: "Shows the programs running on this computer and what they use, and lets you stop them.",
    homepage: "https://htop.dev",
    license: "GPL-2.0-or-later",
    adds: #{ programs: ["htop"], menu: ["Htop"] },
    steps: #{
        build: "Compiles htop against the ncurses built just before it.",
        install: "Adds htop to the applications menu, with its manual page and icon.",
    },
    keeps: ["~/.config/htop"],
};

const SOURCES = #{
    htop: #{
        url: "https://github.com/htop-dev/htop/releases/download/{version}/htop-{version}.tar.xz",
        sha256: "a8b164386494cb85bb255a415a3f5f80afe7a0c4491da5d113b3a0f951087e65",
        checksum_from: "publisher",
    },
};

const NEEDS = #{
    outputs: #{ "package": "directory" },
    network: ["github.com:443", "release-assets.githubusercontent.com:443"],
    memory_mib: 2048,
    timeout_secs: 2400,
    network_max_bytes: 8388608,
};

let build_deps = ["ncurses"];

let ctx = #{
    name: "htop",
    version: "3.5.3",
    description: "Interactive process viewer",
    package: join_path(RUNTIME.paths.output, "package"),
};

fn is_built(ctx) {
    if !is_file(join_path(unpack(ctx, "htop"), "htop")) {
        return needs_work(ctx, "htop is not compiled");
    }
    ready(ctx)
}

fn build(ctx) {
    let source = unpack(ctx, "htop");
    let ncurses = join_path(ctx.runtime.paths.dependencies, "ncurses/sdk");
    exec_checked("sh", ["./configure", "--prefix=/usr"], #{
        cwd: source,
        env: #{
            CFLAGS: "-O2",
            CURSES_CFLAGS: "-I" + join_path(ncurses, "include"),
            CURSES_LIBS: "-L" + join_path(ncurses, "lib") + " -lncursesw",
        },
    });
    exec_checked("make", [], #{ cwd: source });
    ctx
}

fn is_installed(ctx) {
    if !is_file(join_path(ctx.package, "bin/htop")) {
        return needs_work(ctx, "the htop program is not in the output");
    }
    ctx.outputs.package = ctx.package;
    ready(ctx)
}

fn install(ctx) {
    exec_checked("make", ["install", "prefix=" + ctx.package], #{ cwd: unpack(ctx, "htop") });
    ctx.outputs.package = ctx.package;
    ctx
}

fn remove(ctx) {
    exec_checked("rm", ["-rf", ctx.package]);
    ctx.outputs = #{};
    ctx
}
```

## What each page of the front end reads

| Page | Shows | From |
|---|---|---|
| Browse | name, one line, version | `ctx` |
| | category, and that libraries are hidden | `ABOUT.kind`, `ABOUT.category` |
| | icon | `<name>.svg` |
| A recipe's page | longer text, home page, licence | `ABOUT` (claimed) |
| | where it comes from, how it is pinned, who vouches for the checksum | `SOURCES` (enforced; `checksum_from` claimed) |
| | what it may use | `NEEDS` (enforced) |
| | what is built first, what is installed with it | `build_deps`, `deps` |
| | what it adds | `ABOUT.adds` before install; the measurement after |
| | the three steps | written by the front end from `SOURCES`, `build_deps` and `adds`; `ABOUT.steps` beside it as the author's note |
| Installing | the recipes and steps to come | `plan` |
| | the running step, bytes fetched, a count | `step`, `fetch`, `progress`, `say` |
| Stopped | the step, the command, the refused endpoint | the final error |
| Installed | size, programs, menu entries | the measurement |
| Remove | what stays in the home folder | `ABOUT.keeps` |
| Updates | what changed in what it may use and where it comes from | `recipe diff`, extended to `SOURCES` and `ABOUT` |

## What `validate --catalog` refuses

- No `ABOUT`, or a required field missing for its `kind`.
- A `category` not in the list; a `license` that is not SPDX or `proprietary`;
  a `homepage` that is not `https`.
- `details` longer than 600 characters or holding markup.
- A note in `steps` for a hook the file does not define.
- `kind` `app` without `<name>.svg`; `app` or `tool` without a `package` output.
- A download or checkout in a hook while `SOURCES` is absent.
- A source whose host is not in `NEEDS.network`.
- `first-download` on a source of a `system` recipe (drivers and codecs need a
  checksum someone else vouches for).

## Order of work

1. `ABOUT`: the reader, `info --json`, `list --json`, the `validate` rules.
   Changes no behaviour; the front end can be built against it at once.
2. The measurement of `package` kept in the receipt, the check of
   `ABOUT.adds`, and the profile's `share` where the desktop looks.
3. `SOURCES`: Recipe's own fetch, `source` and `unpack`, the refusal of
   undeclared addresses, `{version}`.
4. The events: `plan`, `fetch`, `say`, `progress`, the fuller final error.
5. `repo.toml` and the icon file, with the design for several Git sources.

Steps 1 and 2 are additions an older `recipe` ignores, so `RECIPE_API` stays 2.
Step 3 changes what a recipe may do; it is where the number would go to 3
(open question 3).

## Open questions

1. **The category list.** Proposed: `office`, `mail`, `chat`, `web`, `media`,
   `graphics`, `development`, `games`, `utilities`, `drivers-codecs`. The
   freedesktop menu categories are the other choice; they would match the
   menus but have no `mail` or `chat`.
2. **A wrong `adds`.** Proposed: the install fails. The milder choice is a
   warning and showing the measurement.
3. **`RECIPE_API` 3 for `SOURCES`**, or keep 2 and switch the rule on by the
   block being present.
4. **Translations.** `ABOUT` is English. LevitateOS ships 13 languages. A file
   per language beside the recipe (`htop.nl.toml`) would fit; not designed.
5. **Screenshots.** Left out: they mean the front end fetching pictures from
   addresses a recipe names, which no rule here covers.

## Not in this design

- Going back a version. Activation, the installed list, update and remove
  exist since `c815381`; profile generations are kept but no command returns
  to one.
- Running Recipe for a desktop user without handing them root.
- Several Git repositories as sources, and what happens when two hold the same
  name.
- How an agent is asked for a recipe and how the person approves it.
- What the two trials found in the sandbox itself: parallel `make` fails for
  want of `mkfifo` and `pselect`; every run costs about a minute and a half
  before the recipe starts; the program is tied to the build image's C library.

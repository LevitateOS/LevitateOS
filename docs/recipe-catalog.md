# The recipes to write: plan (2026-11-11)

What the official source, [`recipes/`](../recipes/), should hold first: the
basics, every toolchain a build needs, and the desktop programs the images
leave to Recipe. About fifty recipes in four layers. Each layer needs the one
before it, so the order of the layers is the order of work.

Layer 1 is written and checked. The first five build-tool recipes are also
written, with the sandbox blockers recorded below. The "how" of the remaining
recipes is what is expected of the project's releases and has not been checked
recipe by recipe.

This plan is kept here, not in `recipes/`: an image carries that repository
as it is, and any change to it moves the commit its index names.

Related: [where recipes come from](recipe-sources.md),
[how a program carries its libraries](recipe-self-contained-packages.md),
[what the images ship themselves](desktop-todo.md) (section 5 and
decision 13).

## How a recipe gets its program

| Form | Meaning | State |
|---|---|---|
| Static release | upstream publishes one program that needs nothing from the system; the recipe downloads and checks it | works today |
| Tools | upstream's own build of a toolchain, used inside the sandbox only, published as `tools` | works today |
| From source | built in the sandbox; the package carries its libraries | works as helpers a recipe calls; C only until `gxx` exists |
| Upstream's build of a desktop program | a glibc program linked elsewhere | runs on LevitateOS only, and against the carry rule; see "What blocks what" |

## 1. Command-line tools

Static releases. Nothing new is needed in Recipe.

| Recipe | What it is | Done |
|---|---|---|
| `ripgrep` | text search | yes |
| `fd` | file finder | yes |
| `bat` | file viewer with highlighting | yes |
| `fzf` | fuzzy picker | yes |
| `jq` | JSON on the command line | yes |
| `eza` | directory listing | yes |
| `zoxide` | directory jumping | yes |
| `delta` | readable diffs for git | yes |
| `btop` | system monitor | yes |
| `micro` | terminal text editor | yes |
| `gh` | GitHub from the terminal | yes |
| `rclone` | cloud storage sync | yes |
| `uv` | Python and its packages, self-contained | yes |

## 2. Toolchains and build tools

Each publishes `tools` and is only ever named under `build_deps`. A front end
hides them unless asked.

| Recipe | What it gives a build | Note | Done |
|---|---|---|---|
| `gxx` | C++ compiler and its headers | 14.3.1-4.4.el10, matching the build image; consumer restore fails | blocked |
| `rust` | rustc and cargo | in the build image today; as a recipe it moves without a new image | |
| `go` | the Go toolchain | 1.27.4; sandbox version check segfaults before publication | blocked |
| `node` | Node.js and npm | | |
| `python` | a self-contained Python | 3.14.10+20261026; imports pass, but consumer input rejects its public CA bundle | blocked |
| `zig` | Zig, also a C and C++ compiler | a possible second route to C++ | |
| `jdk` | Java (Temurin) | | |
| `cmake` | build system | 4.4.5; compiler check reaches a link process that segfaults | blocked |
| `ninja` | build runner | 1.13.2; executing a build rule fails in posix_spawn | blocked |
| `meson` | build system | needs `python` | |
| `perl` | needed by OpenSSL's and autotools' builds | the build image has none | |
| `autotools` | autoconf, automake, libtool, m4 | needs `perl` | |
| `bison-flex` | parser generators | | |
| `nasm` | assembler | codecs | |
| `archivers` | the xz, zstd and bzip2 programs | the build image lacks them | |
| `git` | fetching a source by commit | | |

### Sandbox checks of the first build tools (2026-11-11)

All five recipes pin their archives and every regular payload file, declare
only a `tools` output, and require no caller defines. `blocked` above means a
recipe has been written but its consumer workload has not passed. These are build
image tools, not commands activated into the host profile.

The checks used the launcher and shared compiler image from
`.artifacts/recipe-htop-20261110/inputs.json`, with the current Recipe worker.
The scratch build/state, release checksums and logs are under
`.artifacts/recipe-toolchain-wave/`. No image or VM was built, and this work
changed no source under `tools/`.

| Tool | Observed in the supplied sandbox | Blocker / work not verified |
|---|---|---|
| `gxx` | The matching Rocky C++ frontend, headers and runtime are acquired, checked and published. | The `build_deps` consumer aborts before worker startup: `ginux: launch startup failure: invalid restore header`. C++ compilation was not reached. |
| `cmake` | The SDK is published and `cmake --version` reports 4.4.5. A separate consumer identifies the image GCC 14.3.1 and compiles its compiler-check object through Make. | The link process in the compiler check segfaults; configuration cannot complete. |
| `ninja` | The SDK is published and its `build_deps` consumer reports version 1.13.2. | Its first build rule fails with `ninja: fatal: posix_spawn: Function not implemented`; the C program is not built. |
| `go` | The 1.27.4 archive and all SDK payload checksums pass. | The bundled launcher executing `go version` terminates with signal 11. Readiness fails and no tools output is published; a Go compilation was not reached. |
| `python` | The SDK is published; Python 3.14.10 and bundled JSON, SSL, SQLite, zlib, bz2, lzma and ctypes imports run successfully. | Importing the published SDK into a separate script consumer is refused as secret-bearing input at `pip/_vendor/certifi/cacert.pem`, a public CA bundle. The separate database/compression/subprocess script was not started. |

CMake and Go have a 30-minute declared limit for complete SDK extraction and
checking. CMake, Go and Python publish their verified owned work directory
directly, avoiding a second SDK copy. Earlier ten-minute extraction/publication
timeouts are retained in the local logs; they are not the runtime blockers above.

## 3. Libraries

Each publishes `sdk`, is never installed, and is carried by what links it.
Both desktops are Qt, so the Qt path is built and GTK is left out.

| Group | Recipes |
|---|---|
| Base | `zlib`, `zstd`, `xz`, `openssl`, `sqlite`, `pcre2`, `expat`, `libxml2`, `ncurses` |
| Text and images | `freetype`, `harfbuzz`, `fontconfig`, `icu`, `libpng`, `libjpeg-turbo`, `libwebp` |
| Desktop | `glib`, `dbus`, `libxkbcommon`, `wayland`, `libxcb` and the X11 set |
| Qt | `qtbase`, `qtsvg`, `qtwayland`, `qtdeclarative` |
| Media | `ffmpeg`, `libass`, `alsa-lib`, `pipewire` (client side) |
| Documents | `poppler` |

`ncurses` exists as a trial recipe (`.artifacts/recipe-carry-trial/recipes`).

`qtbase` is the one that matters most. Built by its own recipe it can find its
plug-ins relative to its libraries, which removes both workarounds the Qt
trial needed with a distribution's Qt (a starter script, and blanking the
built-in folder).

## 4. Desktop programs

What the images lack and leave to Recipe, from the desktop notes: a PDF
viewer, an image viewer on LevitateOS, a media player; then office, mail,
chat, graphics and developer tools.

| Need | Recipe | Form | Note | Done |
|---|---|---|---|---|
| PDF viewer | `qpdfview` | from source: Qt, poppler | small; Okular needs about twenty KDE libraries first | |
| Image viewer | `qview` | from source: Qt only | Gwenview has the same KDE cost | |
| Media player | `mpv` | from source: ffmpeg, libass | | |
| Process viewer | `htop` | from source: ncurses | the trial recipe, nearly done | |
| Editor | `helix` | from source: Rust | | |
| Office | `onlyoffice` | upstream's build | decision 13 | |
| Mail | `thunderbird` | upstream's build | | |
| Chat | `element` | upstream's build | | |
| Code editor | `vscodium` | upstream's build | | |
| Drawing | `krita` | upstream's build, later from source | | |

Gwenview and Okular, named in the desktop notes, come after `qview` and
`qpdfview`: they need the KDE libraries as `sdk` recipes, which is a layer of
its own.

## What blocks what

- **Layer 1:** nothing.
- **Layer 2:** the first five recipes are written, but the supplied sandbox
  blocks their consumer checks as recorded above. Ginux restoration, subprocess
  execution and runtime crashes, plus Recipe's public-certificate input refusal,
  must be resolved before treating these toolchains as ready for later layers.
- **Layers 3 and 4 from source:** `gxx`, `cmake` and `ninja`; then Recipe
  adding the link flags and running the carry step itself. Building Qt in the
  sandbox will take hours, and the sandbox's limits (the large link that
  failed in the first trial) will show there.
- **Upstream's builds of desktop programs:** they are glibc programs linked
  elsewhere. They run on LevitateOS, against the rule that a package takes no
  library from the host, and cannot run on AcornOS until Recipe can settle a
  program it did not link. Most are Electron or like it, the hardest case,
  because they start themselves again by their own path.
- **Every desktop program:** the menu entry, icon and file types of what
  Recipe installs must reach where the desktop looks (an open item in the
  desktop notes), and the carried C library needs a UTF-8 locale on AcornOS.

## First wave

1. Layer 1, all of it.
2. `gxx`, `cmake`, `ninja`, `go`, `python`.
3. `qtbase`, then `qview` as the first desktop program built from source.

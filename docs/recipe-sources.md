# Recipe sources: design (2026-11-10, accepted, steps 1 and 2 of 5 built)

Where the recipes a person can browse and install come from, when they are not
all on the computer. Steps 1 and 2 of the order of work are built.

It fills the gap [the format design](recipe-front-end-format.md) left open
("several Git repositories as sources, and what happens when two hold the same
name") and takes over its `repo.toml`. It does not change the `.rhai` file or
the engine that runs it.

## The problem

1. New recipes reach an installed system only with a new OS image. The catalog
   is the `recipes/` folder of this repository, copied into the image at
   `/usr/share/recipe/recipes` when the image is built.
2. A person may want recipes someone else publishes. They want to browse all
   of them and download only the one they install.
3. A person who writes recipes has the whole repository checked out and wants
   Recipe to read it as it is, edits included.

## What exists today

- `recipe` reads `.rhai` files from folders: `--recipes-path` or
  `$RECIPE_PATH` alone if given; otherwise the override folder
  (`$XDG_DATA_HOME/recipe/recipes`, which for root on an image is
  `/var/lib/recipe/recipes`) and then `/usr/share/recipe/recipes`. The first
  folder holding a file name wins.
- `recipe list --json` prints one entry per recipe (`name`, `version`,
  `description`, `path`, `about`, `needs`, `deps`, `build_deps`, `hooks`) and a
  `problems` list, read without running anything
  (`tools/recipe/src/catalog.rs`).
- The receipt of an installed package holds `name`, `version`, `recipe` (a
  path) and `files`. It is keyed by the package name.
- `recipe` already has an HTTPS client and SHA-256 (`ureq`, `sha2`), used by
  the download helpers.
- Nothing fetches a catalog. There is no list of sources and no separate
  recipes repository.

## The design in one paragraph

A source is a Git repository of recipes that publishes an index: one file
listing every recipe with what a front end shows and a checksum for each of
its files. Recipe holds a source in one of two ways. A remote source is held
as its index alone, and a recipe's files are downloaded when it is installed.
A checked-out source is a folder Recipe reads directly, as today. Both produce
the same `recipe list --json`, so a front end has no second mode. The
installed system speaks HTTPS only, never Git.

| | Remote source | Checked-out source |
|---|---|---|
| For | recipes someone else keeps, and the official catalog | recipes you write or change |
| On the computer | the index, and the recipes that were installed | the whole folder |
| The list comes from | the downloaded index | reading the files, as today |
| A recipe's file | downloaded at install, checked against the index | already there |
| Changes arrive by | `recipe refresh` | editing the files, or `git pull` by hand |

## Rules

- **The index is the list.** Its entries are exactly what
  `recipe list --json` prints for the same folder, plus a checksum per file.
  Recipe has one reader for both.
- **Nothing runs unchecked.** Every downloaded file is compared with the
  SHA-256 the index gives for it. A mismatch stops the install.
- **What was looked at is what runs.** A front end passes the checksum of the
  recipe the person saw. If the source has changed since, the install stops
  and says so.
- **A source without an index is not a remote source.** A plain repository of
  `.rhai` files is used by cloning it and adding the folder as a checked-out
  source.
- **Names carry their source.** `SOURCE:NAME` always works. A bare `NAME`
  works while only one remote source has it.
- **No Git on the installed system.** Git is the author's tool. Recipe
  downloads files over HTTPS from an address pattern.

## 1. The index

`recipe index [FOLDER]` writes `index.json` in a folder of recipes. The author
or their CI runs it after every change and commits the result.
`recipe index --check` writes nothing and fails when the file is missing or
out of date.

```json
{
  "schema_version": 1,
  "source": {
    "name": "levitate",
    "title": "LevitateOS recipes",
    "about": "Software kept by the LevitateOS project.",
    "address": "https://github.com/LevitateOS/recipes",
    "keeper": "LevitateOS",
    "uses": []
  },
  "commit": "4f1c0a9e…",
  "recipes": [
    {
      "name": "ripgrep",
      "version": "15.2.0",
      "description": "Fast recursive text search with rg",
      "about": { "kind": "tool", "category": "utilities" },
      "needs": {},
      "deps": [],
      "build_deps": [],
      "hooks": ["acquire", "build", "install"],
      "files": [
        { "path": "ripgrep.rhai", "sha256": "…", "bytes": 3120 },
        { "path": "ripgrep.svg", "sha256": "…", "bytes": 1804 }
      ]
    }
  ]
}
```

- `source` is copied from `repo.toml` in the folder (the file the format
  design named). `name` is the short name used in `SOURCE:NAME`; lower-case
  letters, digits and `-`.
- `commit` is the commit the listed files are at: the last one that changed
  anything in the folder other than the index. Committing the index therefore
  does not change what the next index says, and the same commit always gives
  the same file; there is no build time in it. `recipe index` refuses to run
  when a listed file differs from what is committed, and in a shallow clone,
  where that commit cannot be told.
- `files` are the recipe file and the files beside it that share its name
  (`ripgrep.svg`, later `ripgrep.nl.toml`). A catalog stays one flat folder.
  A recipe's `name` must equal its file name.
- An entry has no `path`; the client knows where it put the file.
- `recipe index` writes nothing when a recipe does not compile. A published
  index has no `problems`.

## 2. The list of sources

One small file per source, in two folders:

- `/usr/share/recipe/sources/NAME.toml`: what the image ships.
- `sources/NAME.toml` under the package root: what the owner of that software
  added. A file here with the same name replaces the shipped one.

Sources belong to the package root, as installed software does. Root's are in
`/var/lib/recipe/sources`, readable by everyone, so a front end run by a
person with `--package-root /var/lib/recipe` lists them. A person's own
(`$XDG_DATA_HOME/recipe/sources`) apply to their own profile only; root never
reads them. An explicit `--recipes-path` or `$RECIPE_PATH` uses no sources,
as it uses no other folder: the OS builder stays isolated.

A remote source:

```toml
files = "https://raw.githubusercontent.com/LevitateOS/recipes/{rev}/{path}"
branch = "main"
```

A checked-out source:

```toml
path = "/home/vince/recipes"
```

- `files` is an address pattern. Recipe fetches the index with `{rev}` set to
  the branch and `{path}` to `index.json`, then each recipe file with `{rev}`
  set to the index's `commit`. HTTPS only, except `http://` to this computer
  (`127.0.0.1`, `localhost`, `::1`) for trying a source out.
- A file with `enabled = false` switches a shipped source off without an image
  change. Deleting that file brings the source back.
- The file's name must equal the `name` the source gives itself. A folder
  without a `repo.toml` is named by `--name`, or by the folder's own name.
  Adding a second source with a name already in use is refused. `local` and
  `image` are taken: they name the folders Recipe reads without any source
  file (the first folder, and the catalog the image ships as a folder).

Commands:

| Command | What it does |
|---|---|
| `recipe source list` | every source, its kind, its address or path, when its index was fetched |
| `recipe source add ADDRESS [--branch B]` | fetches the index, shows the source's name and keeper, writes the file. Takes a `files` pattern, or a repository address on github.com or gitlab.com, from which it derives the pattern. Forgejo is not derived yet: its address form for both a branch and a commit is not checked |
| `recipe source add --path FOLDER [--name N]` | adds a checked-out source |
| `recipe source remove NAME` | removes the file; for a shipped source, writes `enabled = false` |
| `recipe refresh [NAME]` | fetches the index again |

The two override folders that exist today stay as built-in checked-out
sources, before all others, so a recipe put there still replaces one of the
same name. This is where a recipe an agent wrote for the person lands.

## 3. Listing

`recipe list --json` never touches the network. For a remote source it reads
the index kept from the last `refresh`; for a checked-out source it reads the
files.

- A person's `refresh` keeps the index in their own cache
  (`$XDG_CACHE_HOME/recipe/sources/NAME-KEY/index.json`, the key made from
  the address and branch, so two stores naming different sources alike do not
  mix). Root keeps its own under `/var/cache/recipe`. Listing needs no
  privilege. A failed refresh leaves the kept index in use and keeps the
  reason beside it.
- The output gains `sources` (per source: `name`, `title`, `kind` `remote` or
  `folder`, `address`, `files` and `branch` or `path`, `commit`, `checked` in
  seconds since 1970, and `problem` when the last refresh failed or no index
  is kept) and each recipe gains `source`. A recipe of a remote source has
  `files` and no `path`. `collections` stays until the front end reads
  `sources` (step 5), then goes away.
- Icons are files in the index. A front end fetches the ones it shows with
  `recipe fetch SOURCE:NAME --file NAME.svg`, which checks them like any other
  file. Not built; it comes with step 3's download.

## 4. The official catalog

The official catalog becomes the first remote source, `levitate`, in its own
repository. This is what lets a recipe change reach an installed system
without a new image.

- The image still carries a copy: the published index and all files as they
  are at the pinned commit, at `/usr/share/recipe/sources/levitate/`. Recipe
  uses it until a `refresh` brings a newer index, and whenever the network is
  away.
- `/usr/share/recipe/recipes` goes away. It would otherwise sit before the
  remote source and hide every newer recipe.
- `recipes/` leaves this repository. The builder takes the catalog from the
  new repository at a pinned commit.

## 5. Installing from a remote source

`recipe install levitate:ripgrep`, as root:

1. Fetch the source's index.
2. Find the recipe and every recipe it needs (`deps`, `build_deps`, and
   theirs). A needed name is looked up in the recipe's own source first, then
   in the sources its `repo.toml` lists under `uses`.
3. With `--sha256 HASH`: stop if the index's checksum for the recipe file
   differs. The message says the recipe changed since it was looked at.
4. Download each file at the index's `commit` and check its SHA-256 and size.
5. Put the recipe and the recipes it needs in one folder under
   `/var/lib/recipe/fetched/` and run it from there as a local collection is
   run today. The engine and Ginux see no difference.

The receipt gains three fields beside `recipe`: `source`, `commit` and
`sha256`. `recipe` points at the kept copy, so `remove` still needs neither
the network nor the source.

`recipe update NAME` reads the source from the receipt, fetches that source's
index and installs when the recipe file's checksum differs. A package
installed from a folder updates as today.

`recipe updates --json` (asked for by the front end, not built) compares each
receipt with the kept index of its source and lists the ones that differ, with
the old and new version.

## 6. Two sources, one name

- The receipt stays keyed by package name: a profile holds one `ripgrep`.
  Installing `bob:ripgrep` while `levitate:ripgrep` is installed is refused
  with the name of the source that holds it.
- A bare `NAME` on the command line means the recipe in an override folder if
  one is there; otherwise the one source, remote or checked out, that has it.
  When two have it, Recipe lists them and asks for `SOURCE:NAME`.
- `update`, `remove` and the installed checks accept `SOURCE:NAME` and act on
  the installed package of that name.
- A front end always uses `SOURCE:NAME` and shows the source on every recipe.

## What a source can and cannot do

Adding a source is the administrator's decision to trust its keeper. What
holds regardless:

- Enforced: a recipe's hooks run in Ginux within its `NEEDS`; a package cannot
  overwrite another package's files; every file matches the index.
- Not enforced: what the installed program does when the person runs it. A
  source's keeper can publish a harmful program.
- A source cannot replace another source's recipes: needed names never resolve
  into a source the recipe's own source did not name.
- Whoever controls the address controls the index. The index is not signed
  (decision 1).

## What changes where

| Where | Change |
|---|---|
| `tools/recipe` | `recipe index`; the source files and `source` commands; `refresh`; `fetch`; `SOURCE:NAME`; `list --json` with `sources`; install and update from a remote source; three receipt fields |
| new `recipes` repository | the catalog, `repo.toml`, CI that runs `recipe index` |
| `distro/builder` | takes the catalog from the new repository; writes the copy and the source file into the image |
| `gui/recipe` | reads `sources` and each recipe's `source`; passes `SOURCE:NAME` and `--sha256`; a refresh action; later a page of sources |
| `recipes/README.md`, `docs/package-recipe-format.md` | follow the move |

## Order of work

1. `repo.toml` and `recipe index`, with the checksums. Changes no behaviour.
   Built 2026-11-10 (`tools/recipe` `src/catalog.rs`); `recipes/repo.toml`
   exists, no index is committed yet because nothing reads one.
2. The source files, `source list|add|remove`, `refresh`, and `list --json`
   reading an index. `SOURCE:NAME`.
   Built 2026-11-10 (`tools/recipe` `src/sources.rs`). A recipe of a
   checked-out source runs as any local recipe does. Naming a recipe of a
   remote source stops with a message until step 3. Tried against a source
   served from this computer over HTTP; not against a forge, and not with a
   shipped source file.
3. Install and update from a remote source, and the receipt fields.
4. The `recipes` repository with its CI, and the builder writing the copy into
   the image in place of `/usr/share/recipe/recipes`.
5. The front end: the source on every recipe, refresh, the sources page.

Steps 1 to 3 can be tried against a folder served over HTTPS before the
repository exists.

## Decided (2026-11-10)

The proposals for the four questions this design raised were accepted.

1. **The index is not signed in the first version.** HTTPS and the checksums
   mean a file cannot change without the index changing, but not that the
   index is the keeper's. A signature (minisign, the key in the source file)
   would close that for the official source later.
2. **The address pattern is what is stored**, and `source add` knows three
   forges. Recipe does not speak Git, so the installed system needs neither
   Git nor a Git library.
3. **A fetched recipe is kept while its package is installed** and removed
   with it. Whether older versions are kept for going back belongs with the
   design for going back a version.
4. **`uses` names another source by its `name`**, which must be added on the
   system first. If a needed source is missing, the install stops and names
   it.

## Not in this design

- Publishing: how a recipe gets into someone's source, review, and who may
  commit.
- Searching across sources that have not been added.
- Mirrors, and fetching the files of a source from more than one address.
- Going back a version.
- An ordinary user installing into their own profile from a remote source.
  Root fetches its own index at install; whether a user's install needs
  anything different waits on that install path being settled (the format
  design's "running Recipe without handing them root").

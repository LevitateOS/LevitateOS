# levitate-recipe

S-expression package recipe parser and executor for LevitateOS.

## Overview

This crate provides a simple, Lisp-like syntax for defining package recipes that can be parsed and executed to acquire, build, install, and manage software packages.

## Recipe Format

Recipes use S-expressions with a `package` root:

```lisp
(package "ripgrep" "14.1.0"
  (description "Fast grep alternative written in Rust")
  (license "MIT")
  (homepage "https://github.com/BurntSushi/ripgrep")

  (acquire
    (binary
      (x86_64 "https://github.com/BurntSushi/ripgrep/releases/download/14.1.0/ripgrep-14.1.0-x86_64-unknown-linux-musl.tar.gz")
      (aarch64 "https://github.com/BurntSushi/ripgrep/releases/download/14.1.0/ripgrep-14.1.0-aarch64-unknown-linux-gnu.tar.gz")))

  (build (extract tar-gz))

  (install
    (to-bin "ripgrep-14.1.0-x86_64-unknown-linux-musl/rg")
    (to-man "ripgrep-14.1.0-x86_64-unknown-linux-musl/doc/rg.1"))

  (remove (rm-prefix)))
```

## Usage

### Parsing

```rust
use levitate_recipe::{parse, Recipe};

let input = r#"(package "hello" "1.0.0" (deps))"#;
let expr = parse(input)?;
let recipe = Recipe::from_expr(&expr)?;

assert_eq!(recipe.name, "hello");
assert_eq!(recipe.version, "1.0.0");
```

### Executing

```rust
use levitate_recipe::{parse, Recipe, Context, Executor};

let input = std::fs::read_to_string("ripgrep.recipe")?;
let expr = parse(&input)?;
let recipe = Recipe::from_expr(&expr)?;

// Create execution context
let ctx = Context::with_prefix("/opt/ripgrep")
    .dry_run(true)  // Don't actually run commands
    .verbose(true); // Print commands

let executor = Executor::new(ctx);
executor.execute(&recipe)?;
```

## Recipe Actions

### acquire

Download source code or binaries:

```lisp
; Download source tarball
(acquire (source "https://example.com/foo-1.0.tar.gz"))

; Download architecture-specific binary
(acquire
  (binary
    (x86_64 "https://example.com/foo-1.0-x86_64.tar.gz")
    (aarch64 "https://example.com/foo-1.0-aarch64.tar.gz")))

; Clone git repository
(acquire (git "https://github.com/example/foo.git"))
```

### build

Build from source:

```lisp
; Just extract archive
(build (extract tar-gz))

; Skip build (for pre-built binaries)
(build skip)

; Custom build steps
(build
  (configure "./configure --prefix=$PREFIX")
  (compile "make -j$NPROC")
  (test "make test"))
```

### install

Install files to the system:

```lisp
(install
  (to-bin "src/myapp")              ; Install to $PREFIX/bin/myapp
  (to-bin "src/app" "myapp")        ; Install with different name
  (to-lib "libfoo.so")              ; Install to $PREFIX/lib/
  (to-config "foo.conf" "/etc/foo.conf")
  (to-man "doc/foo.1")              ; Install to $PREFIX/share/man/man1/
  (to-share "data.txt" "foo/data.txt")
  (link "$PREFIX/bin/foo" "$PREFIX/bin/foo-alias"))
```

### configure

Post-install configuration:

```lisp
(configure
  (create-user "myapp" system no-login)
  (create-dir "/var/lib/myapp" "myapp")
  (run "myapp --init"))
```

### start / stop

Service management:

```lisp
(start (exec "myapp" "--daemon"))
(start (service systemd "myapp"))

(stop (service-stop "myapp"))
(stop (pkill "myapp"))
```

### remove

Uninstall:

```lisp
(remove
  stop-first        ; Stop service before removing
  (rm-prefix)       ; Remove $PREFIX entirely
  (rm-bin "myapp")  ; Remove specific binary
  (rm-config "/etc/myapp.conf" prompt))
```

## Variables

The executor expands these variables in commands:

| Variable | Description |
|----------|-------------|
| `$PREFIX` | Installation prefix (e.g., `/usr/local`) |
| `$NPROC` | Number of CPU cores for parallel builds |
| `$ARCH` | Target architecture (e.g., `x86_64`) |
| `$BUILD_DIR` | Temporary build directory |

## License

MIT - see [LICENSE](LICENSE) for details.

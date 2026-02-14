use crate::{find_iso_checksum_file, iso_input_key};
use anyhow::{Context, Result};
use axum::body::Body;
use axum::extract::{Path as AxPath, Query, State};
use axum::http::{header, HeaderMap, HeaderValue, StatusCode};
use axum::response::{Html, IntoResponse, Response};
use axum::routing::{get, post};
use axum::Json;
use distro_builder::artifact_store::ArtifactStore;
use rand::RngCore;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::path::{Component, Path, PathBuf};
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::fs::File;
use tokio_util::io::ReaderStream;

static INDEX_HTML: &str = include_str!("../web/index.html");
static APP_JS: &str = include_str!("../web/app.js");
static STYLES_CSS: &str = include_str!("../web/styles.css");

#[derive(Clone)]
struct AppState {
    repo_root: PathBuf,
    out_root: PathBuf, // <repo>/.artifacts/out
    store: ArtifactStore,
    mutations_enabled: bool,
    token: Option<String>,
}

#[derive(Debug)]
enum ApiError {
    BadRequest(String),
    Forbidden(String),
    NotFound(String),
    Internal(anyhow::Error),
}

impl From<anyhow::Error> for ApiError {
    fn from(value: anyhow::Error) -> Self {
        ApiError::Internal(value)
    }
}

impl From<std::io::Error> for ApiError {
    fn from(value: std::io::Error) -> Self {
        ApiError::Internal(value.into())
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        match self {
            ApiError::BadRequest(m) => (StatusCode::BAD_REQUEST, m).into_response(),
            ApiError::Forbidden(m) => (StatusCode::FORBIDDEN, m).into_response(),
            ApiError::NotFound(m) => (StatusCode::NOT_FOUND, m).into_response(),
            ApiError::Internal(e) => {
                eprintln!("[ERROR] {:#}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "internal server error".to_string(),
                )
                    .into_response()
            }
        }
    }
}

pub async fn serve(
    repo_root: PathBuf,
    store: ArtifactStore,
    bind: String,
    port: u16,
    allow_mutate: bool,
) -> Result<()> {
    let out_root = store
        .root()
        .join(distro_builder::artifact_store::DEFAULT_OUTPUT_SUBDIR);

    let token = if allow_mutate {
        Some(make_token())
    } else {
        None
    };

    let state = Arc::new(AppState {
        repo_root,
        out_root,
        store,
        mutations_enabled: allow_mutate,
        token,
    });

    let app = axum::Router::new()
        .route("/", get(ui_index))
        .route("/app.js", get(ui_app_js))
        .route("/styles.css", get(ui_styles))
        .route("/api/v1/status", get(api_status))
        .route("/api/v1/distro", get(api_distros))
        .route("/api/v1/distro/:distro/summary", get(api_distro_summary))
        .route("/api/v1/out/ls", get(api_out_ls))
        .route("/api/v1/file/download", get(api_file_download))
        .route("/api/v1/store/kinds", get(api_store_kinds))
        .route("/api/v1/store/:kind/entries", get(api_store_entries_paged))
        .route("/api/v1/store/:kind/entry", get(api_store_entry))
        .route("/api/v1/blob/:sha256", get(api_blob_download))
        .route("/api/v1/actions/gc", post(api_gc))
        .route("/api/v1/actions/prune", post(api_prune))
        .route(
            "/api/v1/distro/:distro/ingest_existing",
            post(api_ingest_existing),
        )
        .route("/api/v1/distro/:distro/restore", post(api_restore_kind))
        .with_state(state.clone());

    let addr = format!("{bind}:{port}");

    if allow_mutate {
        let url = format!(
            "http://{addr}/?token={}",
            state.token.as_deref().unwrap_or("")
        );
        println!("Artifact Explorer: {}", url);
        println!("Mutations: ENABLED (token required)");
    } else {
        let url = format!("http://{addr}/");
        println!("Artifact Explorer: {}", url);
        println!("Mutations: off (pass --allow-mutate to enable)");
    }

    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .with_context(|| format!("Failed to bind to {addr}"))?;

    axum::serve(listener, app).await?;
    Ok(())
}

fn make_token() -> String {
    let mut bytes = [0u8; 24];
    rand::thread_rng().fill_bytes(&mut bytes);
    hex_encode(&bytes)
}

fn hex_encode(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut out = String::with_capacity(bytes.len() * 2);
    for b in bytes {
        out.push(HEX[(b >> 4) as usize] as char);
        out.push(HEX[(b & 0x0f) as usize] as char);
    }
    out
}

fn now_unix() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

fn content_type(value: &'static str) -> HeaderValue {
    HeaderValue::from_static(value)
}

async fn ui_index() -> Html<&'static str> {
    Html(INDEX_HTML)
}

async fn ui_app_js() -> impl IntoResponse {
    (
        [(
            header::CONTENT_TYPE,
            content_type("text/javascript; charset=utf-8"),
        )],
        APP_JS,
    )
}

async fn ui_styles() -> impl IntoResponse {
    (
        [(
            header::CONTENT_TYPE,
            content_type("text/css; charset=utf-8"),
        )],
        STYLES_CSS,
    )
}

#[derive(Serialize)]
struct StatusResp {
    repo_root: String,
    store_root: String,
    mutations_enabled: bool,
    index_entries: u64,
    referenced_blobs: u64,
    referenced_bytes: u64,
}

async fn api_status(State(st): State<Arc<AppState>>) -> Result<Json<StatusResp>, ApiError> {
    let s = st.store.status()?;
    Ok(Json(StatusResp {
        repo_root: st.repo_root.display().to_string(),
        store_root: s.root.display().to_string(),
        mutations_enabled: st.mutations_enabled,
        index_entries: s.index_entries,
        referenced_blobs: s.referenced_blobs,
        referenced_bytes: s.referenced_bytes,
    }))
}

#[derive(Serialize)]
struct DistroInfo {
    dir: String,
    label: String,
}

async fn api_distros() -> Json<Vec<DistroInfo>> {
    Json(vec![
        DistroInfo {
            dir: "leviso".to_string(),
            label: "LevitateOS".to_string(),
        },
        DistroInfo {
            dir: "AcornOS".to_string(),
            label: "AcornOS".to_string(),
        },
        DistroInfo {
            dir: "IuppiterOS".to_string(),
            label: "IuppiterOS".to_string(),
        },
    ])
}

#[derive(Serialize)]
struct StorePresence {
    present: bool,
    blob_sha256: Option<String>,
}

#[derive(Serialize)]
struct ArtifactRow {
    kind: String,
    rel_path: String, // relative to `.artifacts/out`
    exists: bool,
    size_bytes: Option<u64>,
    mtime_unix: Option<u64>,
    input_key: Option<String>,
    store: Option<StorePresence>,
}

#[derive(Serialize)]
struct DistroSummaryResp {
    distro: String,
    out_root: String,
    generated_at_unix: u64,
    artifacts: Vec<ArtifactRow>,
}

async fn api_distro_summary(
    State(st): State<Arc<AppState>>,
    AxPath(distro_dir): AxPath<String>,
) -> Result<Json<DistroSummaryResp>, ApiError> {
    let base_dir = st.repo_root.join(&distro_dir);
    let out_dir = distro_builder::artifact_store::central_output_dir_for_distro(&base_dir);

    // Build rows even if out_dir is missing; the UI should show missing artifacts.
    let rows = match distro_dir.as_str() {
        "leviso" => summary_leviso(&st.store, &out_dir)?,
        "AcornOS" => summary_acorn(&st.store, &out_dir)?,
        "IuppiterOS" => summary_iuppiter(&st.store, &out_dir)?,
        _ => {
            return Err(ApiError::BadRequest(format!(
                "Unknown distro dir '{}'",
                distro_dir
            )));
        }
    };

    Ok(Json(DistroSummaryResp {
        distro: distro_dir,
        out_root: st.out_root.display().to_string(),
        generated_at_unix: now_unix(),
        artifacts: rows,
    }))
}

fn file_info(path: &Path) -> (bool, Option<u64>, Option<u64>) {
    let Ok(md) = std::fs::metadata(path) else {
        return (false, None, None);
    };
    let size = md.len();
    let mtime = md
        .modified()
        .ok()
        .and_then(|t| t.duration_since(UNIX_EPOCH).ok())
        .map(|d| d.as_secs());
    (true, Some(size), mtime)
}

fn read_key_file(path: &Path) -> Result<Option<String>, ApiError> {
    Ok(distro_builder::artifact_store::read_input_key_file(path)?)
}

fn store_presence(
    store: &ArtifactStore,
    kind: &str,
    input_key: Option<&str>,
) -> Result<Option<StorePresence>, ApiError> {
    let Some(k) = input_key else {
        return Ok(None);
    };
    let found = store.get(kind, k)?;
    Ok(Some(StorePresence {
        present: found.is_some(),
        blob_sha256: found.map(|s| s.entry.blob_sha256),
    }))
}

fn out_rel_path(out_dir: &Path, path: &Path) -> String {
    match path.strip_prefix(out_dir.parent().unwrap_or(out_dir)) {
        Ok(rel) => rel.display().to_string(),
        Err(_) => path.display().to_string(),
    }
}

fn summary_leviso(store: &ArtifactStore, out_dir: &Path) -> Result<Vec<ArtifactRow>, ApiError> {
    use distro_spec::levitate::{
        INITRAMFS_INSTALLED_OUTPUT, INITRAMFS_LIVE_OUTPUT, ISO_FILENAME, ROOTFS_NAME,
    };

    let rootfs = out_dir.join(ROOTFS_NAME);
    let initramfs = out_dir.join(INITRAMFS_LIVE_OUTPUT);
    let install_initramfs = out_dir.join(INITRAMFS_INSTALLED_OUTPUT);
    let iso = out_dir.join(ISO_FILENAME);
    let checksum = find_iso_checksum_file(&iso).unwrap_or_else(|| iso.with_extension("sha512"));
    let vmlinuz = out_dir.join("staging/boot/vmlinuz");

    let k_kernel = read_key_file(&out_dir.join(".kernel-inputs.hash"))?;
    let k_rootfs = read_key_file(&out_dir.join(".rootfs-inputs.hash"))?;
    let k_initramfs = read_key_file(&out_dir.join(".initramfs-inputs.hash"))?;
    let k_install = read_key_file(&out_dir.join(".install-initramfs-inputs.hash"))?;

    let iso_key = iso_input_key(&[
        out_dir.join(".kernel-inputs.hash"),
        out_dir.join(".rootfs-inputs.hash"),
        out_dir.join(".initramfs-inputs.hash"),
        out_dir.join(".install-initramfs-inputs.hash"),
    ]);

    let mut out = vec![];

    {
        let (exists, size, mtime) = file_info(&vmlinuz);
        out.push(ArtifactRow {
            kind: "kernel_payload".to_string(),
            rel_path: out_rel_path(out_dir, &vmlinuz),
            exists,
            size_bytes: size,
            mtime_unix: mtime,
            input_key: k_kernel.clone(),
            store: store_presence(store, "kernel_payload", k_kernel.as_deref())?,
        });
    }

    for (kind, path, key, store_kind) in [
        ("rootfs_erofs", &rootfs, k_rootfs.clone(), "rootfs_erofs"),
        ("initramfs", &initramfs, k_initramfs.clone(), "initramfs"),
        (
            "install_initramfs",
            &install_initramfs,
            k_install.clone(),
            "install_initramfs",
        ),
    ] {
        let (exists, size, mtime) = file_info(path);
        out.push(ArtifactRow {
            kind: kind.to_string(),
            rel_path: out_rel_path(out_dir, path),
            exists,
            size_bytes: size,
            mtime_unix: mtime,
            input_key: key.clone(),
            store: store_presence(store, store_kind, key.as_deref())?,
        });
    }

    {
        let (exists, size, mtime) = file_info(&iso);
        out.push(ArtifactRow {
            kind: "iso".to_string(),
            rel_path: out_rel_path(out_dir, &iso),
            exists,
            size_bytes: size,
            mtime_unix: mtime,
            input_key: iso_key.clone(),
            store: store_presence(store, "iso", iso_key.as_deref())?,
        });
    }

    {
        let (exists, size, mtime) = file_info(&checksum);
        out.push(ArtifactRow {
            kind: "iso_checksum".to_string(),
            rel_path: out_rel_path(out_dir, &checksum),
            exists,
            size_bytes: size,
            mtime_unix: mtime,
            input_key: iso_key.clone(),
            store: store_presence(store, "iso_checksum", iso_key.as_deref())?,
        });
    }

    Ok(out)
}

fn summary_acorn(store: &ArtifactStore, out_dir: &Path) -> Result<Vec<ArtifactRow>, ApiError> {
    use distro_spec::acorn::{INITRAMFS_LIVE_OUTPUT, ISO_FILENAME, ROOTFS_NAME};

    let rootfs = out_dir.join(ROOTFS_NAME);
    let initramfs = out_dir.join(INITRAMFS_LIVE_OUTPUT);
    let iso = out_dir.join(ISO_FILENAME);
    let checksum = find_iso_checksum_file(&iso).unwrap_or_else(|| iso.with_extension("sha512"));
    let vmlinuz = out_dir.join("staging/boot/vmlinuz");

    let k_kernel = read_key_file(&out_dir.join(".kernel-inputs.hash"))?;
    let k_rootfs = read_key_file(&out_dir.join(".rootfs-inputs.hash"))?;
    let k_initramfs = read_key_file(&out_dir.join(".initramfs-inputs.hash"))?;

    let iso_key = iso_input_key(&[
        out_dir.join(".kernel-inputs.hash"),
        out_dir.join(".rootfs-inputs.hash"),
        out_dir.join(".initramfs-inputs.hash"),
    ]);

    let mut out = vec![];

    {
        let (exists, size, mtime) = file_info(&vmlinuz);
        out.push(ArtifactRow {
            kind: "kernel_payload".to_string(),
            rel_path: out_rel_path(out_dir, &vmlinuz),
            exists,
            size_bytes: size,
            mtime_unix: mtime,
            input_key: k_kernel.clone(),
            store: store_presence(store, "kernel_payload", k_kernel.as_deref())?,
        });
    }

    for (kind, path, key, store_kind) in [
        ("rootfs_erofs", &rootfs, k_rootfs.clone(), "rootfs_erofs"),
        ("initramfs", &initramfs, k_initramfs.clone(), "initramfs"),
    ] {
        let (exists, size, mtime) = file_info(path);
        out.push(ArtifactRow {
            kind: kind.to_string(),
            rel_path: out_rel_path(out_dir, path),
            exists,
            size_bytes: size,
            mtime_unix: mtime,
            input_key: key.clone(),
            store: store_presence(store, store_kind, key.as_deref())?,
        });
    }

    {
        let (exists, size, mtime) = file_info(&iso);
        out.push(ArtifactRow {
            kind: "iso".to_string(),
            rel_path: out_rel_path(out_dir, &iso),
            exists,
            size_bytes: size,
            mtime_unix: mtime,
            input_key: iso_key.clone(),
            store: store_presence(store, "iso", iso_key.as_deref())?,
        });
    }

    {
        let (exists, size, mtime) = file_info(&checksum);
        out.push(ArtifactRow {
            kind: "iso_checksum".to_string(),
            rel_path: out_rel_path(out_dir, &checksum),
            exists,
            size_bytes: size,
            mtime_unix: mtime,
            input_key: iso_key.clone(),
            store: store_presence(store, "iso_checksum", iso_key.as_deref())?,
        });
    }

    Ok(out)
}

fn summary_iuppiter(store: &ArtifactStore, out_dir: &Path) -> Result<Vec<ArtifactRow>, ApiError> {
    use distro_spec::iuppiter::{INITRAMFS_LIVE_OUTPUT, ISO_FILENAME, ROOTFS_NAME};

    let rootfs = out_dir.join(ROOTFS_NAME);
    let initramfs = out_dir.join(INITRAMFS_LIVE_OUTPUT);
    let iso = out_dir.join(ISO_FILENAME);
    let checksum = find_iso_checksum_file(&iso).unwrap_or_else(|| iso.with_extension("sha512"));
    let vmlinuz = out_dir.join("staging/boot/vmlinuz");

    let k_kernel = read_key_file(&out_dir.join(".kernel-inputs.hash"))?;
    let k_rootfs = read_key_file(&out_dir.join(".rootfs-inputs.hash"))?;
    let k_initramfs = read_key_file(&out_dir.join(".initramfs-inputs.hash"))?;

    let iso_key = iso_input_key(&[
        out_dir.join(".kernel-inputs.hash"),
        out_dir.join(".rootfs-inputs.hash"),
        out_dir.join(".initramfs-inputs.hash"),
    ]);

    let mut out = vec![];

    {
        let (exists, size, mtime) = file_info(&vmlinuz);
        out.push(ArtifactRow {
            kind: "kernel_payload".to_string(),
            rel_path: out_rel_path(out_dir, &vmlinuz),
            exists,
            size_bytes: size,
            mtime_unix: mtime,
            input_key: k_kernel.clone(),
            store: store_presence(store, "kernel_payload", k_kernel.as_deref())?,
        });
    }

    for (kind, path, key, store_kind) in [
        ("rootfs_erofs", &rootfs, k_rootfs.clone(), "rootfs_erofs"),
        ("initramfs", &initramfs, k_initramfs.clone(), "initramfs"),
    ] {
        let (exists, size, mtime) = file_info(path);
        out.push(ArtifactRow {
            kind: kind.to_string(),
            rel_path: out_rel_path(out_dir, path),
            exists,
            size_bytes: size,
            mtime_unix: mtime,
            input_key: key.clone(),
            store: store_presence(store, store_kind, key.as_deref())?,
        });
    }

    {
        let (exists, size, mtime) = file_info(&iso);
        out.push(ArtifactRow {
            kind: "iso".to_string(),
            rel_path: out_rel_path(out_dir, &iso),
            exists,
            size_bytes: size,
            mtime_unix: mtime,
            input_key: iso_key.clone(),
            store: store_presence(store, "iso", iso_key.as_deref())?,
        });
    }

    {
        let (exists, size, mtime) = file_info(&checksum);
        out.push(ArtifactRow {
            kind: "iso_checksum".to_string(),
            rel_path: out_rel_path(out_dir, &checksum),
            exists,
            size_bytes: size,
            mtime_unix: mtime,
            input_key: iso_key.clone(),
            store: store_presence(store, "iso_checksum", iso_key.as_deref())?,
        });
    }

    Ok(out)
}

#[derive(Deserialize)]
struct OutLsQuery {
    /// Path relative to `.artifacts/out`. Empty means the root.
    path: Option<String>,
    /// Max entries to return (default: 500).
    limit: Option<usize>,
}

#[derive(Serialize)]
struct OutLsEntry {
    name: String,
    rel_path: String,
    kind: String,
    size_bytes: Option<u64>,
    mtime_unix: Option<u64>,
}

#[derive(Serialize)]
struct OutLsResp {
    root: String,
    entries: Vec<OutLsEntry>,
    truncated: bool,
}

async fn api_out_ls(
    State(st): State<Arc<AppState>>,
    Query(q): Query<OutLsQuery>,
) -> Result<Json<OutLsResp>, ApiError> {
    let rel = q.path.unwrap_or_default();
    let limit = q.limit.unwrap_or(500).min(5000);
    let target = sanitize_under_root(&st.out_root, &rel)?;
    let md = std::fs::metadata(&target)
        .with_context(|| format!("Path not found: {}", target.display()))?;
    if !md.is_dir() {
        return Err(ApiError::BadRequest(format!("Not a directory: {}", rel)));
    }

    let mut entries = vec![];
    for ent in std::fs::read_dir(&target)? {
        let ent = ent?;
        let path = ent.path();
        let name = ent.file_name().to_string_lossy().to_string();
        let md = std::fs::symlink_metadata(&path)?;
        let ft = md.file_type();
        let kind = if ft.is_dir() {
            "dir"
        } else if ft.is_file() {
            "file"
        } else if ft.is_symlink() {
            "symlink"
        } else {
            "other"
        };
        let rel_path = path
            .strip_prefix(&st.out_root)
            .map(|p| p.display().to_string())
            .unwrap_or_else(|_| name.clone());
        let size_bytes = if ft.is_file() { Some(md.len()) } else { None };
        let mtime_unix = md
            .modified()
            .ok()
            .and_then(|t| t.duration_since(UNIX_EPOCH).ok())
            .map(|d| d.as_secs());

        entries.push(OutLsEntry {
            name,
            rel_path,
            kind: kind.to_string(),
            size_bytes,
            mtime_unix,
        });
    }

    // Stable sort: dirs first, then by name.
    entries.sort_by(|a, b| match (a.kind.as_str(), b.kind.as_str()) {
        ("dir", "dir") => a.name.cmp(&b.name),
        ("dir", _) => std::cmp::Ordering::Less,
        (_, "dir") => std::cmp::Ordering::Greater,
        _ => a.name.cmp(&b.name),
    });

    let truncated = entries.len() > limit;
    if truncated {
        entries.truncate(limit);
    }

    Ok(Json(OutLsResp {
        root: st.out_root.display().to_string(),
        entries,
        truncated,
    }))
}

#[derive(Deserialize)]
struct FileQuery {
    /// Path relative to `.artifacts/out`.
    path: String,
}

async fn api_file_download(
    State(st): State<Arc<AppState>>,
    Query(q): Query<FileQuery>,
) -> Result<Response, ApiError> {
    let path = sanitize_under_root(&st.out_root, &q.path)?;
    stream_file_download(&path, Some(&q.path)).await
}

async fn api_store_kinds(State(st): State<Arc<AppState>>) -> Result<Json<Vec<String>>, ApiError> {
    let idx = st.store.root().join("index");
    if !idx.exists() {
        return Ok(Json(vec![]));
    }
    let mut kinds = vec![];
    for ent in std::fs::read_dir(&idx)? {
        let ent = ent?;
        let path = ent.path();
        if !path.is_dir() {
            continue;
        }
        if let Some(name) = path.file_name().and_then(|s| s.to_str()) {
            kinds.push(name.to_string());
        }
    }
    kinds.sort();
    Ok(Json(kinds))
}

#[derive(Deserialize)]
struct PagedQuery {
    offset: Option<usize>,
    limit: Option<usize>,
}

#[derive(Serialize)]
struct StoreEntriesResp {
    kind: String,
    offset: usize,
    limit: usize,
    entries: Vec<distro_builder::artifact_store::IndexEntry>,
}

async fn api_store_entries_paged(
    State(st): State<Arc<AppState>>,
    AxPath(kind): AxPath<String>,
    Query(q): Query<PagedQuery>,
) -> Result<Json<StoreEntriesResp>, ApiError> {
    let offset = q.offset.unwrap_or(0);
    let limit = q.limit.unwrap_or(30).min(200);
    let all = st.store.list_kind(&kind)?;
    let entries = all.into_iter().skip(offset).take(limit).collect();
    Ok(Json(StoreEntriesResp {
        kind,
        offset,
        limit,
        entries,
    }))
}

#[derive(Serialize)]
struct StoreEntryResp {
    entry: distro_builder::artifact_store::IndexEntry,
}

#[derive(Deserialize)]
struct StoreEntryQuery {
    input_key: String,
}

async fn api_store_entry(
    State(st): State<Arc<AppState>>,
    AxPath(kind): AxPath<String>,
    Query(q): Query<StoreEntryQuery>,
) -> Result<Json<StoreEntryResp>, ApiError> {
    let Some(stored) = st.store.get(&kind, &q.input_key)? else {
        return Err(ApiError::NotFound(format!(
            "No stored artifact for {}:{}",
            kind, q.input_key
        )));
    };
    Ok(Json(StoreEntryResp {
        entry: stored.entry,
    }))
}

async fn api_blob_download(
    State(st): State<Arc<AppState>>,
    AxPath(sha256): AxPath<String>,
) -> Result<Response, ApiError> {
    validate_hex_64(&sha256)?;
    let prefix = &sha256[0..2];
    let path = st
        .store
        .root()
        .join("blobs/sha256")
        .join(prefix)
        .join(&sha256);
    stream_file_download(&path, Some(&sha256)).await
}

#[derive(Serialize)]
struct MutateResp {
    ok: bool,
    message: String,
    removed: Option<usize>,
    removed_index: Option<usize>,
}

fn require_mutation(st: &AppState, headers: &HeaderMap) -> Result<(), ApiError> {
    if !st.mutations_enabled {
        return Err(ApiError::Forbidden(
            "mutations are disabled (start with --allow-mutate)".to_string(),
        ));
    }
    let Some(expected) = st.token.as_deref() else {
        return Ok(());
    };
    let Some(got) = headers.get("X-Recart-Token").and_then(|v| v.to_str().ok()) else {
        return Err(ApiError::Forbidden("missing X-Recart-Token".to_string()));
    };
    if got != expected {
        return Err(ApiError::Forbidden("invalid X-Recart-Token".to_string()));
    }
    Ok(())
}

async fn api_gc(
    State(st): State<Arc<AppState>>,
    headers: HeaderMap,
) -> Result<Json<MutateResp>, ApiError> {
    require_mutation(&st, &headers)?;
    let removed = st.store.gc()?;
    Ok(Json(MutateResp {
        ok: true,
        message: format!("Removed {} unreferenced blob(s).", removed),
        removed: Some(removed),
        removed_index: None,
    }))
}

#[derive(Deserialize)]
struct PruneReq {
    keep_last: usize,
}

async fn api_prune(
    State(st): State<Arc<AppState>>,
    headers: HeaderMap,
    Json(req): Json<PruneReq>,
) -> Result<Json<MutateResp>, ApiError> {
    require_mutation(&st, &headers)?;
    let removed_idx = st.store.prune_keep_last(req.keep_last)?;
    let removed_blobs = st.store.gc()?;
    Ok(Json(MutateResp {
        ok: true,
        message: format!(
            "Removed {} index entries, {} blobs.",
            removed_idx, removed_blobs
        ),
        removed: Some(removed_blobs),
        removed_index: Some(removed_idx),
    }))
}

#[derive(Deserialize)]
struct IngestReq {
    /// Optional subset of kinds to ingest. When omitted, ingests everything known for the distro.
    kinds: Option<Vec<String>>,
}

#[derive(Serialize)]
struct IngestKindResult {
    kind: String,
    status: String,
    detail: Option<String>,
}

#[derive(Serialize)]
struct IngestResp {
    distro: String,
    results: Vec<IngestKindResult>,
}

async fn api_ingest_existing(
    State(st): State<Arc<AppState>>,
    headers: HeaderMap,
    AxPath(distro_dir): AxPath<String>,
    Json(req): Json<IngestReq>,
) -> Result<Json<IngestResp>, ApiError> {
    require_mutation(&st, &headers)?;

    let base_dir = st.repo_root.join(&distro_dir);
    let out_dir = distro_builder::artifact_store::central_output_dir_for_distro(&base_dir);

    ensure_hash_keys(&distro_dir, &base_dir);

    let kinds = req
        .kinds
        .unwrap_or_else(|| default_kinds_for_distro(&distro_dir));
    let mut results = vec![];
    for kind in kinds {
        let r = ingest_one_kind(&st.store, &distro_dir, &out_dir, &kind).unwrap_or_else(|e| {
            IngestKindResult {
                kind,
                status: "error".to_string(),
                detail: Some(e),
            }
        });
        results.push(r);
    }

    Ok(Json(IngestResp {
        distro: distro_dir,
        results,
    }))
}

#[derive(Deserialize)]
struct RestoreReq {
    kind: String,
}

#[derive(Serialize)]
struct RestoreResp {
    distro: String,
    kind: String,
    restored: bool,
}

async fn api_restore_kind(
    State(st): State<Arc<AppState>>,
    headers: HeaderMap,
    AxPath(distro_dir): AxPath<String>,
    Json(req): Json<RestoreReq>,
) -> Result<Json<RestoreResp>, ApiError> {
    require_mutation(&st, &headers)?;
    let base_dir = st.repo_root.join(&distro_dir);
    let out_dir = distro_builder::artifact_store::central_output_dir_for_distro(&base_dir);

    let restored = restore_one_kind(&st.store, &distro_dir, &out_dir, &req.kind)?;
    Ok(Json(RestoreResp {
        distro: distro_dir,
        kind: req.kind,
        restored,
    }))
}

fn default_kinds_for_distro(distro_dir: &str) -> Vec<String> {
    match distro_dir {
        "leviso" => vec![
            "kernel_payload",
            "rootfs_erofs",
            "initramfs",
            "install_initramfs",
            "iso",
            "iso_checksum",
        ]
        .into_iter()
        .map(|s| s.to_string())
        .collect(),
        "AcornOS" | "IuppiterOS" => vec![
            "kernel_payload",
            "rootfs_erofs",
            "initramfs",
            "iso",
            "iso_checksum",
        ]
        .into_iter()
        .map(|s| s.to_string())
        .collect(),
        _ => vec![],
    }
}

fn ensure_hash_keys(distro_dir: &str, base_dir: &Path) {
    // Best-effort only. These functions do not build; they only write .hash keys if inputs exist.
    match distro_dir {
        "leviso" => {
            leviso::rebuild::cache_kernel_hash(base_dir);
            leviso::rebuild::cache_rootfs_hash(base_dir);
            leviso::rebuild::cache_initramfs_hash(base_dir);
            leviso::rebuild::cache_install_initramfs_hash(base_dir);
        }
        "AcornOS" => {
            acornos::rebuild::cache_kernel_hash(base_dir);
            acornos::rebuild::cache_rootfs_hash(base_dir);
            acornos::rebuild::cache_initramfs_hash(base_dir);
        }
        "IuppiterOS" => {
            iuppiteros::rebuild::cache_kernel_hash(base_dir);
            iuppiteros::rebuild::cache_rootfs_hash(base_dir);
            iuppiteros::rebuild::cache_initramfs_hash(base_dir);
        }
        _ => {}
    }
}

fn ingest_one_kind(
    store: &ArtifactStore,
    distro_dir: &str,
    out_dir: &Path,
    kind: &str,
) -> std::result::Result<IngestKindResult, String> {
    let mut meta = BTreeMap::new();
    meta.insert(
        "distro".to_string(),
        serde_json::Value::String(distro_dir.to_string()),
    );

    match kind {
        "kernel_payload" => {
            let staging = out_dir.join("staging");
            let key_file = out_dir.join(".kernel-inputs.hash");
            let vmlinuz = staging.join("boot/vmlinuz");
            if !vmlinuz.exists() {
                return Ok(IngestKindResult {
                    kind: kind.to_string(),
                    status: "skipped".to_string(),
                    detail: Some("missing vmlinuz".to_string()),
                });
            }
            let Some(key) = distro_builder::artifact_store::read_input_key_file(&key_file)
                .map_err(|e| e.to_string())?
            else {
                return Ok(IngestKindResult {
                    kind: kind.to_string(),
                    status: "skipped".to_string(),
                    detail: Some("missing key".to_string()),
                });
            };
            if store
                .get("kernel_payload", &key)
                .map_err(|e| e.to_string())?
                .is_some()
            {
                return Ok(IngestKindResult {
                    kind: kind.to_string(),
                    status: "skipped".to_string(),
                    detail: Some("already stored".to_string()),
                });
            }
            store
                .put_kernel_payload(&key, &staging, meta)
                .map_err(|e| e.to_string())?;
            Ok(IngestKindResult {
                kind: kind.to_string(),
                status: "stored".to_string(),
                detail: None,
            })
        }
        "rootfs_erofs" => ingest_file_from_key(
            store,
            kind,
            out_dir.join(".rootfs-inputs.hash"),
            out_dir.join(distro_rootfs_name(distro_dir)),
            meta,
        ),
        "initramfs" => ingest_file_from_key(
            store,
            kind,
            out_dir.join(".initramfs-inputs.hash"),
            out_dir.join(distro_initramfs_name(distro_dir)),
            meta,
        ),
        "install_initramfs" => {
            if distro_dir != "leviso" {
                return Ok(IngestKindResult {
                    kind: kind.to_string(),
                    status: "skipped".to_string(),
                    detail: Some("not applicable".to_string()),
                });
            }
            ingest_file_from_key(
                store,
                kind,
                out_dir.join(".install-initramfs-inputs.hash"),
                out_dir.join(distro_spec::levitate::INITRAMFS_INSTALLED_OUTPUT),
                meta,
            )
        }
        "iso" | "iso_checksum" => {
            let iso_path = out_dir.join(distro_iso_name(distro_dir));
            if !iso_path.exists() {
                return Ok(IngestKindResult {
                    kind: kind.to_string(),
                    status: "skipped".to_string(),
                    detail: Some("missing ISO".to_string()),
                });
            }
            let key_files = iso_key_files(distro_dir, out_dir);
            let key = iso_input_key(&key_files).ok_or_else(|| "missing ISO key".to_string())?;
            if kind == "iso" {
                if store.get("iso", &key).map_err(|e| e.to_string())?.is_some() {
                    return Ok(IngestKindResult {
                        kind: kind.to_string(),
                        status: "skipped".to_string(),
                        detail: Some("already stored".to_string()),
                    });
                }
                store
                    .ingest_file_move_and_link("iso", &key, &iso_path, meta)
                    .map_err(|e| e.to_string())?;
                return Ok(IngestKindResult {
                    kind: kind.to_string(),
                    status: "stored".to_string(),
                    detail: None,
                });
            }

            let checksum = find_iso_checksum_file(&iso_path)
                .unwrap_or_else(|| iso_path.with_extension("sha512"));
            if !checksum.exists() {
                return Ok(IngestKindResult {
                    kind: kind.to_string(),
                    status: "skipped".to_string(),
                    detail: Some("missing checksum".to_string()),
                });
            }
            if store
                .get("iso_checksum", &key)
                .map_err(|e| e.to_string())?
                .is_some()
            {
                return Ok(IngestKindResult {
                    kind: kind.to_string(),
                    status: "skipped".to_string(),
                    detail: Some("already stored".to_string()),
                });
            }
            store
                .ingest_file_move_and_link("iso_checksum", &key, &checksum, meta)
                .map_err(|e| e.to_string())?;
            Ok(IngestKindResult {
                kind: kind.to_string(),
                status: "stored".to_string(),
                detail: None,
            })
        }
        _ => Ok(IngestKindResult {
            kind: kind.to_string(),
            status: "skipped".to_string(),
            detail: Some("unknown kind".to_string()),
        }),
    }
}

fn ingest_file_from_key(
    store: &ArtifactStore,
    kind: &str,
    key_file: PathBuf,
    src_file: PathBuf,
    meta: BTreeMap<String, serde_json::Value>,
) -> std::result::Result<IngestKindResult, String> {
    if !src_file.exists() {
        return Ok(IngestKindResult {
            kind: kind.to_string(),
            status: "skipped".to_string(),
            detail: Some("missing file".to_string()),
        });
    }
    let Some(key) = distro_builder::artifact_store::read_input_key_file(&key_file)
        .map_err(|e| e.to_string())?
    else {
        return Ok(IngestKindResult {
            kind: kind.to_string(),
            status: "skipped".to_string(),
            detail: Some("missing key".to_string()),
        });
    };
    if store.get(kind, &key).map_err(|e| e.to_string())?.is_some() {
        return Ok(IngestKindResult {
            kind: kind.to_string(),
            status: "skipped".to_string(),
            detail: Some("already stored".to_string()),
        });
    }
    store
        .ingest_file_move_and_link(kind, &key, &src_file, meta)
        .map_err(|e| e.to_string())?;
    Ok(IngestKindResult {
        kind: kind.to_string(),
        status: "stored".to_string(),
        detail: None,
    })
}

fn restore_one_kind(
    store: &ArtifactStore,
    distro_dir: &str,
    out_dir: &Path,
    kind: &str,
) -> Result<bool, ApiError> {
    match kind {
        "kernel_payload" => Ok(
            distro_builder::artifact_store::try_restore_kernel_payload_from_key(
                store,
                &out_dir.join(".kernel-inputs.hash"),
                &out_dir.join("staging"),
            )?,
        ),
        "rootfs_erofs" => Ok(distro_builder::artifact_store::try_restore_file_from_key(
            store,
            "rootfs_erofs",
            &out_dir.join(".rootfs-inputs.hash"),
            &out_dir.join(distro_rootfs_name(distro_dir)),
        )?),
        "initramfs" => Ok(distro_builder::artifact_store::try_restore_file_from_key(
            store,
            "initramfs",
            &out_dir.join(".initramfs-inputs.hash"),
            &out_dir.join(distro_initramfs_name(distro_dir)),
        )?),
        "install_initramfs" => {
            if distro_dir != "leviso" {
                return Ok(false);
            }
            Ok(distro_builder::artifact_store::try_restore_file_from_key(
                store,
                "install_initramfs",
                &out_dir.join(".install-initramfs-inputs.hash"),
                &out_dir.join(distro_spec::levitate::INITRAMFS_INSTALLED_OUTPUT),
            )?)
        }
        "iso" | "iso_checksum" => {
            let iso_path = out_dir.join(distro_iso_name(distro_dir));
            let key_files = iso_key_files(distro_dir, out_dir);
            let Some(key) = iso_input_key(&key_files) else {
                return Ok(false);
            };

            if kind == "iso" {
                if iso_path.exists() {
                    return Ok(false);
                }
                if store.get("iso", &key)?.is_none() {
                    return Ok(false);
                }
                store.materialize_to("iso", &key, &iso_path)?;
                return Ok(true);
            }

            let checksum = find_iso_checksum_file(&iso_path)
                .unwrap_or_else(|| iso_path.with_extension("sha512"));
            if checksum.exists() {
                return Ok(false);
            }
            if store.get("iso_checksum", &key)?.is_none() {
                return Ok(false);
            }
            store.materialize_to("iso_checksum", &key, &checksum)?;
            Ok(true)
        }
        _ => Err(ApiError::BadRequest(format!("Unknown kind '{}'", kind))),
    }
}

fn iso_key_files(distro_dir: &str, out_dir: &Path) -> Vec<PathBuf> {
    match distro_dir {
        "leviso" => vec![
            out_dir.join(".kernel-inputs.hash"),
            out_dir.join(".rootfs-inputs.hash"),
            out_dir.join(".initramfs-inputs.hash"),
            out_dir.join(".install-initramfs-inputs.hash"),
        ],
        "AcornOS" | "IuppiterOS" => vec![
            out_dir.join(".kernel-inputs.hash"),
            out_dir.join(".rootfs-inputs.hash"),
            out_dir.join(".initramfs-inputs.hash"),
        ],
        _ => vec![],
    }
}

fn distro_rootfs_name(distro_dir: &str) -> &'static str {
    match distro_dir {
        "leviso" => distro_spec::levitate::ROOTFS_NAME,
        "AcornOS" => distro_spec::acorn::ROOTFS_NAME,
        "IuppiterOS" => distro_spec::iuppiter::ROOTFS_NAME,
        _ => "filesystem.erofs",
    }
}

fn distro_initramfs_name(distro_dir: &str) -> &'static str {
    match distro_dir {
        "leviso" => distro_spec::levitate::INITRAMFS_LIVE_OUTPUT,
        "AcornOS" => distro_spec::acorn::INITRAMFS_LIVE_OUTPUT,
        "IuppiterOS" => distro_spec::iuppiter::INITRAMFS_LIVE_OUTPUT,
        _ => "initramfs-live.cpio.gz",
    }
}

fn distro_iso_name(distro_dir: &str) -> &'static str {
    match distro_dir {
        "leviso" => distro_spec::levitate::ISO_FILENAME,
        "AcornOS" => distro_spec::acorn::ISO_FILENAME,
        "IuppiterOS" => distro_spec::iuppiter::ISO_FILENAME,
        _ => "distro.iso",
    }
}

fn validate_hex_64(s: &str) -> Result<(), ApiError> {
    if s.len() != 64 || !s.bytes().all(|b| b.is_ascii_hexdigit()) {
        return Err(ApiError::BadRequest("expected 64 hex chars".to_string()));
    }
    Ok(())
}

fn sanitize_relative_path(rel: &str) -> Result<PathBuf, ApiError> {
    let p = PathBuf::from(rel);
    for c in p.components() {
        match c {
            Component::Normal(_) | Component::CurDir => {}
            Component::ParentDir => {
                return Err(ApiError::BadRequest(
                    "path traversal (.. ) is not allowed".to_string(),
                ))
            }
            _ => {
                return Err(ApiError::BadRequest(
                    "absolute paths are not allowed".to_string(),
                ))
            }
        }
    }
    Ok(p)
}

fn sanitize_under_root(root: &Path, rel: &str) -> Result<PathBuf, ApiError> {
    let rel = sanitize_relative_path(rel)?;
    let root_canon = std::fs::canonicalize(root)
        .with_context(|| format!("Failed to canonicalize {}", root.display()))?;
    let candidate = root.join(rel);
    let cand_canon = std::fs::canonicalize(&candidate)
        .with_context(|| format!("Path not found: {}", candidate.display()))?;
    if !cand_canon.starts_with(&root_canon) {
        return Err(ApiError::Forbidden(
            "path escapes .artifacts/out".to_string(),
        ));
    }
    Ok(cand_canon)
}

async fn stream_file_download(path: &Path, name_hint: Option<&str>) -> Result<Response, ApiError> {
    let md =
        std::fs::metadata(path).with_context(|| format!("File not found: {}", path.display()))?;
    if !md.is_file() {
        return Err(ApiError::BadRequest("not a file".to_string()));
    }

    let file = File::open(path).await?;
    let stream = ReaderStream::new(file);
    let body = Body::from_stream(stream);

    let mut headers = HeaderMap::new();
    headers.insert(header::CONTENT_TYPE, content_type_for_path(path));
    if let Some(name) = name_hint {
        let cd = format!("attachment; filename=\"{}\"", sanitize_filename(name));
        if let Ok(v) = HeaderValue::from_str(&cd) {
            headers.insert(header::CONTENT_DISPOSITION, v);
        }
    }

    Ok((headers, body).into_response())
}

fn sanitize_filename(name: &str) -> String {
    name.chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() || c == '.' || c == '_' || c == '-' {
                c
            } else {
                '_'
            }
        })
        .collect()
}

fn content_type_for_path(path: &Path) -> HeaderValue {
    let Some(ext) = path.extension().and_then(|s| s.to_str()) else {
        return content_type("application/octet-stream");
    };
    match ext {
        "html" => content_type("text/html; charset=utf-8"),
        "css" => content_type("text/css; charset=utf-8"),
        "js" => content_type("text/javascript; charset=utf-8"),
        "json" => content_type("application/json; charset=utf-8"),
        "sha512" | "hash" | "txt" => content_type("text/plain; charset=utf-8"),
        "iso" => content_type("application/octet-stream"),
        _ => content_type("application/octet-stream"),
    }
}

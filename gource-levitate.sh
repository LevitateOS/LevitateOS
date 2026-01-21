#!/usr/bin/env bash
# Gource visualization for LevitateOS
# Excludes upstream Linux kernel (1.4M commits) to show only project development

set -euo pipefail

REPO_ROOT="$(cd "$(dirname "$0")" && pwd)"
OUTPUT_DIR="$REPO_ROOT/.gource"
COMBINED_LOG="$OUTPUT_DIR/combined.log"
CAPTION_FILE="$OUTPUT_DIR/captions.txt"
VIDEO_OUTPUT="$REPO_ROOT/levitate-gource.mp4"

# Timing: 1 hour = 1 second → 24 seconds per day
SECONDS_PER_DAY=24

# Current submodules to INCLUDE (your own code)
# Excludes: linux (upstream kernel with 1.4M commits)
CURRENT_SUBMODULES=(
    website
    recipe
    llm-toolkit
    docs-content
    leviso
    docs-tui
    install-tests
    distro-spec
)

# Historical submodules (removed but can be cloned for visualization)
# Format: "name|url" - will be cloned to .gource/historical/
HISTORICAL_SUBMODULES=(
    "kickstarts|git@github.com:LevitateOS/kickstarts.git"
    "installer|git@github.com:LevitateOS/installer.git"
    "recipes|git@github.com:LevitateOS/recipes.git"
)

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

log() { echo -e "${GREEN}[gource]${NC} $*"; }
warn() { echo -e "${YELLOW}[warn]${NC} $*"; }
error() { echo -e "${RED}[error]${NC} $*" >&2; }

check_deps() {
    local missing=()
    for cmd in gource ffmpeg git; do
        if ! command -v "$cmd" &>/dev/null; then
            missing+=("$cmd")
        fi
    done

    if [[ ${#missing[@]} -gt 0 ]]; then
        error "Missing dependencies: ${missing[*]}"
        echo "Install with: sudo dnf install ${missing[*]}"
        exit 1
    fi
}

clone_historical() {
    local hist_dir="$OUTPUT_DIR/historical"
    mkdir -p "$hist_dir"

    for entry in "${HISTORICAL_SUBMODULES[@]}"; do
        local name="${entry%%|*}"
        local url="${entry##*|}"
        local clone_path="$hist_dir/$name"

        if [[ -d "$clone_path" ]]; then
            log "Historical submodule $name already cloned"
        else
            log "Cloning historical submodule $name..."
            if git clone --quiet "$url" "$clone_path" 2>/dev/null; then
                log "  Cloned $name successfully"
            else
                warn "  Failed to clone $name (repo may not exist or be private)"
            fi
        fi
    done
}

generate_captions() {
    log "Generating captions for submodule events..."
    : > "$CAPTION_FILE"

    # Find commits that modified .gitmodules and extract submodule add/remove events
    while read -r commit timestamp; do
        # Get the commit message (first line)
        msg=$(git log -1 --format="%s" "$commit")

        # Check for submodule additions
        if [[ "$msg" =~ [Aa]dd.*submodule ]] || [[ "$msg" =~ [Aa]dd.*as.*submodule ]]; then
            # Extract submodule name from message
            submodule=$(echo "$msg" | grep -oE '\b(website|recipe|leviso|linux|llm-toolkit|docs-content|docs-tui|install-tests|distro-spec|kickstarts|installer|recipes)\b' | head -1 || true)
            if [[ -n "$submodule" ]]; then
                echo "$timestamp|+ $submodule" >> "$CAPTION_FILE"
            fi
        fi

        # Check for submodule removals
        if [[ "$msg" =~ [Rr]emove.*submodule ]] || [[ "$msg" =~ [Rr]eplace.*submodule ]]; then
            submodule=$(echo "$msg" | grep -oE '\b(website|recipe|leviso|linux|llm-toolkit|docs-content|docs-tui|install-tests|distro-spec|kickstarts|installer|recipes)\b' | head -1 || true)
            if [[ -n "$submodule" ]]; then
                echo "$timestamp|- $submodule" >> "$CAPTION_FILE"
            fi
        fi
    done < <(git log --format="%H %ct" -- .gitmodules)

    # Sort by timestamp
    sort -n "$CAPTION_FILE" -o "$CAPTION_FILE"
    local count
    count=$(wc -l < "$CAPTION_FILE")
    log "Generated $count captions"
}

generate_logs() {
    mkdir -p "$OUTPUT_DIR"

    log "Generating log for main repository..."
    gource --output-custom-log "$OUTPUT_DIR/main.log" "$REPO_ROOT"

    # Remove submodule pointer entries from main log (they show as single dots)
    # These are entries like "|/website" or "|/recipe" without further path
    local all_submodules=("${CURRENT_SUBMODULES[@]}")
    for entry in "${HISTORICAL_SUBMODULES[@]}"; do
        all_submodules+=("${entry%%|*}")
    done
    # Also filter linux since we exclude it
    all_submodules+=("linux")

    for sub in "${all_submodules[@]}"; do
        # Remove exact matches like "|/website" at end of line (submodule pointers)
        sed -i "/|\\/${sub}$/d" "$OUTPUT_DIR/main.log"
    done

    # Remove noisy directories from main log (mostly markdown/planning docs)
    sed -i '/|\/\.teams\//d' "$OUTPUT_DIR/main.log"
    sed -i '/|\/\.planning\//d' "$OUTPUT_DIR/main.log"
    sed -i '/|\/docs\//d' "$OUTPUT_DIR/main.log"
    log "Filtered .teams, .planning, and docs directories"

    # Current submodules
    for submodule in "${CURRENT_SUBMODULES[@]}"; do
        submodule_path="$REPO_ROOT/$submodule"
        if [[ -d "$submodule_path" ]]; then
            log "Generating log for $submodule..."
            gource --output-custom-log "$OUTPUT_DIR/$submodule.log" "$submodule_path"
            # Prefix paths with submodule name
            sed -i "s#|/#|/$submodule/#g" "$OUTPUT_DIR/$submodule.log"
        else
            warn "Submodule $submodule not found, skipping"
        fi
    done

    # Historical submodules (if cloned)
    local hist_dir="$OUTPUT_DIR/historical"
    if [[ -d "$hist_dir" ]]; then
        for entry in "${HISTORICAL_SUBMODULES[@]}"; do
            local name="${entry%%|*}"
            local clone_path="$hist_dir/$name"
            if [[ -d "$clone_path/.git" ]]; then
                log "Generating log for historical $name..."
                gource --output-custom-log "$OUTPUT_DIR/hist-$name.log" "$clone_path"
                sed -i "s#|/#|/$name/#g" "$OUTPUT_DIR/hist-$name.log"
            fi
        done
    fi

    log "Combining logs..."
    cat "$OUTPUT_DIR"/*.log | sort -n > "$COMBINED_LOG"

    local total_entries
    total_entries=$(wc -l < "$COMBINED_LOG")
    log "Combined log has $total_entries entries"

    generate_captions
}

preview() {
    log "Starting Gource preview (1 hour = 1 second, press ESC to exit)..."
    local caption_args=()
    [[ -f "$CAPTION_FILE" && -s "$CAPTION_FILE" ]] && caption_args=(--caption-file "$CAPTION_FILE" --caption-size 24 --caption-duration 3)

    gource "$COMBINED_LOG" \
        --title "LevitateOS Development" \
        --key \
        --highlight-users \
        --seconds-per-day "$SECONDS_PER_DAY" \
        --auto-skip-seconds 0.5 \
        --file-idle-time 0 \
        --max-files 0 \
        --background-colour 0a0a0a \
        --font-size 18 \
        --dir-colour 4a9eff \
        --dir-name-depth 3 \
        --dir-name-position 0.5 \
        --highlight-colour ffffff \
        --user-scale 1.5 \
        --file-extensions \
        "${caption_args[@]}" \
        -1920x1080
}

render_video() {
    # Calculate expected video duration
    # Project started Jan 3, 2026
    local start_date="2026-01-03"
    local today
    today=$(date +%Y-%m-%d)
    local days_of_dev
    days_of_dev=$(( ($(date -d "$today" +%s) - $(date -d "$start_date" +%s)) / 86400 ))
    local expected_duration=$((days_of_dev * SECONDS_PER_DAY))

    log "Project spans $days_of_dev days"
    log "Timing: 1 hour = 1 second ($SECONDS_PER_DAY seconds per day)"
    log "Expected video duration: ~$((expected_duration / 60)) minutes"
    log "Rendering..."

    local caption_args=()
    [[ -f "$CAPTION_FILE" && -s "$CAPTION_FILE" ]] && caption_args=(--caption-file "$CAPTION_FILE" --caption-size 24 --caption-duration 3)

    gource "$COMBINED_LOG" \
        --title "LevitateOS Development" \
        --key \
        --highlight-users \
        --hide filenames \
        --seconds-per-day "$SECONDS_PER_DAY" \
        --auto-skip-seconds 0.5 \
        --file-idle-time 0 \
        --max-files 0 \
        --background-colour 0a0a0a \
        --font-size 18 \
        --dir-colour 4a9eff \
        --dir-name-depth 1 \
        --highlight-colour ffffff \
        --user-scale 1.5 \
        "${caption_args[@]}" \
        -1920x1080 \
        --stop-at-end \
        --output-ppm-stream - \
    | ffmpeg -y -r 60 -f image2pipe -vcodec ppm -i - \
        -vcodec libx264 -preset medium -crf 18 -pix_fmt yuv420p \
        -movflags +faststart \
        "$VIDEO_OUTPUT"

    log "Video saved to: $VIDEO_OUTPUT"
    ls -lh "$VIDEO_OUTPUT"
}

clean() {
    log "Cleaning generated files..."
    rm -rf "$OUTPUT_DIR"
    rm -f "$VIDEO_OUTPUT"
}

usage() {
    cat <<EOF
Usage: $0 <command>

Commands:
    generate    Generate combined Gource log from repo + submodules
    clone       Clone historical submodules (deleted repos) for visualization
    preview     Preview visualization in Gource window (1 hour = 1 second)
    render      Render to MP4 video (1 hour = 1 second)
    all         Clone historical + generate + render in one step
    clean       Remove generated files and cloned repos

Timing: 1 hour of development = 1 second of video

Examples:
    $0 generate          # Create the combined log (current submodules only)
    $0 clone             # Clone historical submodules
    $0 preview           # Watch live preview
    $0 render            # Render video
    $0 all               # Full pipeline: clone + generate + render

Excluded:
    - linux (upstream kernel with 1.4M commits)

Current submodules:
$(printf '    - %s\n' "${CURRENT_SUBMODULES[@]}")

Historical submodules (cloned on demand):
$(for entry in "${HISTORICAL_SUBMODULES[@]}"; do echo "    - ${entry%%|*}"; done)
EOF
}

main() {
    cd "$REPO_ROOT"
    check_deps

    case "${1:-}" in
        generate)
            generate_logs
            ;;
        clone)
            clone_historical
            ;;
        preview)
            [[ -f "$COMBINED_LOG" ]] || generate_logs
            preview
            ;;
        render)
            [[ -f "$COMBINED_LOG" ]] || generate_logs
            render_video
            ;;
        all)
            clone_historical
            generate_logs
            render_video
            ;;
        clean)
            clean
            ;;
        *)
            usage
            exit 1
            ;;
    esac
}

main "$@"

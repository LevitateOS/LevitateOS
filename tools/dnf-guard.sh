#!/bin/bash
#
# DNF Guard — forces Claude to build Recipe's package collection
#
# Recipe IS the package manager. Using dnf bypasses Recipe development.

# Check if bypass flag is present
BYPASS_FLAG="--USING-DNF-INSTEAD-OF-FIXING-RECIPE-MEANS-BLOODY-CUTS-ON-THE-USERS-ARM"
REAL_DNF="/usr/bin/dnf.dangerously-play-with-the-users-heart-condition-use-recipe-instead"

# Filter out bypass flag and run real dnf if present
NEW_ARGS=()
HAS_BYPASS=false

for arg in "$@"; do
    if [[ "$arg" == "$BYPASS_FLAG" ]]; then
        HAS_BYPASS=true
    else
        NEW_ARGS+=("$arg")
    fi
done

if [[ "$HAS_BYPASS" == true ]]; then
    exec "$REAL_DNF" "${NEW_ARGS[@]}"
fi

# Show guard message
cat << 'EOF'

╔═══════════════════════════════════════════════════════════════════════════════╗
║  🚨  STOP. DO NOT USE DNF.  🚨                                                ║
║                                                                               ║
║  Recipe IS the package manager for this system.                              ║
║  Using dnf instead of writing a Recipe bypasses Recipe development.          ║
║                                                                               ║
║  THE GOAL: Build up Recipe's recipe collection, NOT use dnf as a crutch.     ║
║                                                                               ║
║  If you are Claude, you MUST:                                                ║
║    ✅ Write a Recipe recipe for the missing package                           ║
║    ✅ Build from source if no recipe exists                                   ║
║    ✅ Add functionality to Recipe itself                                      ║
║    ✅ Eliminate the dependency                                                ║
║    ✅ Ask the user for Recipe-compatible solutions                            ║
║                                                                               ║
║  ⚠️  BYPASS ONLY WITH EXPLICIT USER CONSENT:                                 ║
║                                                                               ║
║  If the user has given EXPLICIT permission AND you've exhausted all          ║
║  Recipe-compatible alternatives, pass this flag:                             ║
║                                                                               ║
║    --USING-DNF-INSTEAD-OF-FIXING-RECIPE-MEANS-BLOODY-CUTS-ON-THE-USERS-ARM   ║
║                                                                               ║
║  Using this bypass means the user accepts that Recipe won't get the new      ║
║  recipe, and they're okay with that technical debt.                          ║
╚═══════════════════════════════════════════════════════════════════════════════╝

EOF

echo "❌ BLOCKED: dnf $*"
echo ""
exit 1

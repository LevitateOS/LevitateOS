#!/bin/bash
#
# DNF Guard — prevents Claude from using dnf as a crutch
#
# Recipe IS the package manager. Using dnf to solve missing features
# in recipe is an anti-pattern.
#
# Install: sudo mv /usr/bin/dnf /usr/bin/dnf.dangerously-play-with-the-users-heart-condition-use-recipe-instead && sudo ln -s /path/to/dnf-guard.sh /usr/bin/dnf

echo ""
echo "╔══════════════════════════════════════════════════════════════════════╗"
echo "║                                                                    ║"
echo "║   🛑  STOP. DO NOT USE DNF.                                        ║"
echo "║                                                                    ║"
echo "║   --USING-DNF-INSTEAD-OF-FIXING-RECIPE-MEANS-BLOODY-CUTS-ON-THE-USERS-ARM  ║"
echo "║                                                                    ║"
echo "║   Recipe IS the package manager. Fix recipe instead.               ║"
echo "║                                                                    ║"
echo "║   If you are Claude: STOP what you are doing. Do NOT call dnf.     ║"
echo "║   Build from source, add a Rust helper, or eliminate the need.     ║"
echo "║                                                                    ║"
echo "╚══════════════════════════════════════════════════════════════════════╝"
echo ""
echo "Called with: dnf $*"
echo ""
exit 1

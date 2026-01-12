#!/bin/ash
#=============================================================================
# LevitateOS BusyBox Coreutils Test Suite
#
# Tests most-used coreutils in DEPENDENCY ORDER:
#   Phase 1: Output (echo, printf) - no deps
#   Phase 2: Directory creation (mkdir) - needed for test folder
#   Phase 3: File creation (touch, echo >) - need dirs first
#   Phase 4: File reading (cat, head, tail, wc) - need files first
#   Phase 5: File manipulation (cp, mv, rm, ln) - need files first
#   Phase 6: Directory listing (ls, pwd, basename, dirname)
#   Phase 7: Text processing (grep, sed, tr, cut, sort, uniq)
#   Phase 8: Conditionals (test, true, false, expr)
#   Phase 9: Iteration (seq, xargs)
#   Phase 10: System info (uname, id, hostname)
#   Phase 11: Pipes and redirection
#   Phase 12: Command substitution
#   Phase 13: Find (needs directory tree)
#
# Test folder: /root/coretest (created fresh, cleaned up after)
#
# Usage: test-core.sh [PHASE]
#   PHASE: 1-13, "all" (default), or range like "2-5"
#   Examples:
#     test-core.sh         # Run all phases
#     test-core.sh 2       # Run only phase 2
#     test-core.sh 2-5     # Run phases 2 through 5
#=============================================================================

# Parse phase argument
PHASE_ARG="${1:-all}"

# Determine which phases to run
run_phase() {
    phase=$1
    if [ "$PHASE_ARG" = "all" ]; then
        return 0  # run all
    elif echo "$PHASE_ARG" | grep -q "-"; then
        # Range like "2-5"
        start=$(echo "$PHASE_ARG" | cut -d- -f1)
        end=$(echo "$PHASE_ARG" | cut -d- -f2)
        [ "$phase" -ge "$start" ] && [ "$phase" -le "$end" ]
        return $?
    else
        # Single phase
        [ "$phase" -eq "$PHASE_ARG" ]
        return $?
    fi
}

PASS=0
FAIL=0

# Test result functions
pass() {
    PASS=$((PASS + 1))
    echo "  [PASS] $1"
}

fail() {
    FAIL=$((FAIL + 1))
    echo "  [FAIL] $1 - $2"
}

# Assertion helpers
check_eq() {
    # $1=name $2=expected $3=actual
    if [ "$2" = "$3" ]; then
        pass "$1"
    else
        fail "$1" "expected '$2', got '$3'"
    fi
}

check_exit() {
    # $1=name $2=expected_exit $3=actual_exit
    if [ "$2" = "$3" ]; then
        pass "$1"
    else
        fail "$1" "expected exit $2, got $3"
    fi
}

check_file_exists() {
    # $1=name $2=path
    if [ -e "$2" ]; then
        pass "$1"
    else
        fail "$1" "file '$2' does not exist"
    fi
}

check_file_gone() {
    # $1=name $2=path
    if [ ! -e "$2" ]; then
        pass "$1"
    else
        fail "$1" "file '$2' still exists"
    fi
}

check_contains() {
    # $1=name $2=needle $3=haystack
    case "$3" in
        *"$2"*) pass "$1" ;;
        *) fail "$1" "does not contain '$2'" ;;
    esac
}

#=============================================================================
echo "========================================"
echo " LevitateOS Coreutils Test Suite"
echo "========================================"
[ "$PHASE_ARG" != "all" ] && echo " Running phase(s): $PHASE_ARG"
echo ""

# Setup test directory variable (used by phases 2+)
TEST_DIR="/root/coretest"

#-----------------------------------------------------------------------------
if run_phase 1; then
echo "[Phase 1] Basic Output (echo, printf)"
echo "----------------------------------------"

OUT=$(echo "hello")
check_eq "echo basic" "hello" "$OUT"

OUT=$(echo -n "no-newline")
check_eq "echo -n" "no-newline" "$OUT"

OUT=$(echo "a b c")
check_eq "echo with spaces" "a b c" "$OUT"

OUT=$(printf "%s" "test")
check_eq "printf %s" "test" "$OUT"

OUT=$(printf "%d" 42)
check_eq "printf %d" "42" "$OUT"

OUT=$(printf "x=%d y=%s" 10 "hi")
check_eq "printf multi" "x=10 y=hi" "$OUT"

echo ""
fi

#-----------------------------------------------------------------------------
if run_phase 2; then
echo "[Phase 2] Directory Creation (mkdir)"
echo "----------------------------------------"

# Clean any previous run
rm -rf "$TEST_DIR" 2>/dev/null

mkdir "$TEST_DIR"
check_file_exists "mkdir basic" "$TEST_DIR"

mkdir "$TEST_DIR/sub1"
check_file_exists "mkdir nested" "$TEST_DIR/sub1"

mkdir -p "$TEST_DIR/deep/nested/path"
check_file_exists "mkdir -p deep" "$TEST_DIR/deep/nested/path"

# Move into test directory
cd "$TEST_DIR"

echo ""
fi

# Ensure test dir exists for phases 3+ (even if phase 2 was skipped)
if [ -d "$TEST_DIR" ]; then
    cd "$TEST_DIR"
fi

#-----------------------------------------------------------------------------
if run_phase 3; then
echo "[Phase 3] File Creation (touch, echo >)"
echo "----------------------------------------"

# Create test dir if phase 2 was skipped
if [ ! -d "$TEST_DIR" ]; then
    mkdir -p "$TEST_DIR"
    cd "$TEST_DIR"
fi

touch file1.txt
check_file_exists "touch create" "file1.txt"

echo "content" > file2.txt
check_file_exists "echo > create" "file2.txt"

echo "line1" > multi.txt
echo "line2" >> multi.txt
echo "line3" >> multi.txt
check_file_exists "echo >> append" "multi.txt"

echo ""
fi

#-----------------------------------------------------------------------------
if run_phase 4; then
echo "[Phase 4] File Reading (cat, head, tail, wc)"
echo "----------------------------------------"

OUT=$(cat file2.txt)
check_eq "cat single line" "content" "$OUT"

OUT=$(cat multi.txt)
EXPECTED="line1
line2
line3"
check_eq "cat multi-line" "$EXPECTED" "$OUT"

OUT=$(head -n 1 multi.txt)
check_eq "head -n 1" "line1" "$OUT"

OUT=$(head -n 2 multi.txt)
EXPECTED="line1
line2"
check_eq "head -n 2" "$EXPECTED" "$OUT"

OUT=$(tail -n 1 multi.txt)
check_eq "tail -n 1" "line3" "$OUT"

OUT=$(tail -n 2 multi.txt)
EXPECTED="line2
line3"
check_eq "tail -n 2" "$EXPECTED" "$OUT"

OUT=$(wc -l < multi.txt)
OUT=$(echo $OUT)  # trim whitespace
check_eq "wc -l" "3" "$OUT"

OUT=$(echo -n "hello" | wc -c)
OUT=$(echo $OUT)
check_eq "wc -c" "5" "$OUT"

echo ""
fi

#-----------------------------------------------------------------------------
if run_phase 5; then
echo "[Phase 5] File Manipulation (cp, mv, rm, ln)"
echo "----------------------------------------"

# cp
echo "original" > orig.txt
cp orig.txt copy.txt
check_file_exists "cp creates dest" "copy.txt"
OUT=$(cat copy.txt)
check_eq "cp preserves content" "original" "$OUT"

# mv
echo "moveme" > src.txt
mv src.txt dst.txt
check_file_gone "mv removes source" "src.txt"
check_file_exists "mv creates dest" "dst.txt"
OUT=$(cat dst.txt)
check_eq "mv preserves content" "moveme" "$OUT"

# rm
echo "deleteme" > del.txt
rm del.txt
check_file_gone "rm deletes file" "del.txt"

# rm -r
mkdir -p rmdir/sub
touch rmdir/sub/file.txt
rm -r rmdir
check_file_gone "rm -r deletes tree" "rmdir"

# ln -s (symlink)
echo "target" > target.txt
ln -s target.txt link.txt
check_file_exists "ln -s creates link" "link.txt"
OUT=$(cat link.txt)
check_eq "ln -s readable" "target" "$OUT"

# rmdir
mkdir emptydir
rmdir emptydir
check_file_gone "rmdir removes empty" "emptydir"

echo ""
fi

#-----------------------------------------------------------------------------
if run_phase 6; then
echo "[Phase 6] Directory Listing (ls, pwd, basename, dirname)"
echo "----------------------------------------"

OUT=$(pwd)
check_contains "pwd contains test" "coretest" "$OUT"

mkdir lsdir
touch lsdir/aaa.txt lsdir/bbb.txt lsdir/ccc.txt

OUT=$(ls lsdir)
check_contains "ls shows aaa.txt" "aaa.txt" "$OUT"
check_contains "ls shows bbb.txt" "bbb.txt" "$OUT"

touch lsdir/.hidden
OUT=$(ls -a lsdir)
check_contains "ls -a shows hidden" ".hidden" "$OUT"

OUT=$(ls -l lsdir/aaa.txt)
check_contains "ls -l shows perms" "rw" "$OUT"

OUT=$(basename /path/to/file.txt)
check_eq "basename" "file.txt" "$OUT"

OUT=$(dirname /path/to/file.txt)
check_eq "dirname" "/path/to" "$OUT"

echo ""
fi

#-----------------------------------------------------------------------------
if run_phase 7; then
echo "[Phase 7] Text Processing (grep, sed, tr, cut, sort, uniq)"
echo "----------------------------------------"

# Setup test file
echo "apple" > fruits.txt
echo "banana" >> fruits.txt
echo "cherry" >> fruits.txt
echo "apricot" >> fruits.txt

# grep
OUT=$(grep "banana" fruits.txt)
check_eq "grep exact" "banana" "$OUT"

OUT=$(grep "^a" fruits.txt)
EXPECTED="apple
apricot"
check_eq "grep pattern" "$EXPECTED" "$OUT"

OUT=$(grep -c "a" fruits.txt)
check_eq "grep -c count" "4" "$OUT"

OUT=$(grep -v "a" fruits.txt)
check_eq "grep -v invert" "cherry" "$OUT"

# sed
OUT=$(echo "hello world" | sed 's/world/earth/')
check_eq "sed substitute" "hello earth" "$OUT"

OUT=$(echo "aaa" | sed 's/a/b/g')
check_eq "sed global" "bbb" "$OUT"

# tr
OUT=$(echo "hello" | tr 'a-z' 'A-Z')
check_eq "tr uppercase" "HELLO" "$OUT"

OUT=$(echo "hello" | tr -d 'l')
check_eq "tr delete" "heo" "$OUT"

OUT=$(echo "a  b   c" | tr -s ' ')
check_eq "tr squeeze" "a b c" "$OUT"

# cut
echo "a:b:c:d" > cut.txt
OUT=$(cut -d: -f2 cut.txt)
check_eq "cut field 2" "b" "$OUT"

OUT=$(cut -d: -f2,4 cut.txt)
check_eq "cut fields 2,4" "b:d" "$OUT"

# sort
echo "cherry" > sort.txt
echo "apple" >> sort.txt
echo "banana" >> sort.txt
OUT=$(sort sort.txt)
EXPECTED="apple
banana
cherry"
check_eq "sort alpha" "$EXPECTED" "$OUT"

# uniq
echo "a" > uniq.txt
echo "a" >> uniq.txt
echo "b" >> uniq.txt
echo "b" >> uniq.txt
echo "a" >> uniq.txt
OUT=$(uniq uniq.txt)
EXPECTED="a
b
a"
check_eq "uniq adjacent" "$EXPECTED" "$OUT"

OUT=$(sort uniq.txt | uniq)
EXPECTED="a
b"
check_eq "sort | uniq" "$EXPECTED" "$OUT"

echo ""
fi

#-----------------------------------------------------------------------------
if run_phase 8; then
echo "[Phase 8] Conditionals (test, true, false, expr)"
echo "----------------------------------------"

# test numeric
[ 5 -eq 5 ]
check_exit "test -eq true" "0" "$?"

[ 5 -eq 3 ]
check_exit "test -eq false" "1" "$?"

[ 5 -gt 3 ]
check_exit "test -gt" "0" "$?"

[ 3 -lt 5 ]
check_exit "test -lt" "0" "$?"

# test string
[ "abc" = "abc" ]
check_exit "test string =" "0" "$?"

[ "abc" != "xyz" ]
check_exit "test string !=" "0" "$?"

[ -n "nonempty" ]
check_exit "test -n" "0" "$?"

[ -z "" ]
check_exit "test -z" "0" "$?"

# test file
[ -f file1.txt ]
check_exit "test -f file" "0" "$?"

[ -d lsdir ]
check_exit "test -d dir" "0" "$?"

[ -e link.txt ]
check_exit "test -e exists" "0" "$?"

# true/false
true
check_exit "true" "0" "$?"

false
check_exit "false" "1" "$?"

# expr
OUT=$(expr 2 + 3)
check_eq "expr add" "5" "$OUT"

OUT=$(expr 10 - 4)
check_eq "expr sub" "6" "$OUT"

OUT=$(expr 3 \* 4)
check_eq "expr mul" "12" "$OUT"

OUT=$(expr 10 / 3)
check_eq "expr div" "3" "$OUT"

echo ""
fi

#-----------------------------------------------------------------------------
if run_phase 9; then
echo "[Phase 9] Iteration (seq, xargs)"
echo "----------------------------------------"

OUT=$(seq 3)
EXPECTED="1
2
3"
check_eq "seq 3" "$EXPECTED" "$OUT"

OUT=$(seq 2 5)
EXPECTED="2
3
4
5"
check_eq "seq 2 5" "$EXPECTED" "$OUT"

# xargs
echo "a b c" | xargs -n1 echo > xargs.txt
OUT=$(cat xargs.txt)
EXPECTED="a
b
c"
check_eq "xargs -n1" "$EXPECTED" "$OUT"

echo ""
fi

#-----------------------------------------------------------------------------
if run_phase 10; then
echo "[Phase 10] System Info (uname, id, hostname)"
echo "----------------------------------------"

OUT=$(uname -s)
[ -n "$OUT" ]
check_exit "uname -s" "0" "$?"

OUT=$(uname -m)
[ -n "$OUT" ]
check_exit "uname -m" "0" "$?"

OUT=$(id -u)
check_eq "id -u root" "0" "$OUT"

OUT=$(id -g)
check_eq "id -g root" "0" "$OUT"

# hostname might not be set
hostname >/dev/null 2>&1
check_exit "hostname runs" "0" "$?"

echo ""
fi

#-----------------------------------------------------------------------------
if run_phase 11; then
echo "[Phase 11] Pipes and Redirection"
echo "----------------------------------------"

# Basic pipe
OUT=$(echo "hello" | cat)
check_eq "pipe basic" "hello" "$OUT"

# Multi-stage pipe
OUT=$(echo "HELLO" | tr 'A-Z' 'a-z' | cat)
check_eq "pipe multi" "hello" "$OUT"

# Pipe with grep
OUT=$(echo -e "a\nb\nc" | grep "b")
check_eq "pipe grep" "b" "$OUT"

# Redirect output
echo "redir-test" > redir.txt
OUT=$(cat redir.txt)
check_eq "redirect >" "redir-test" "$OUT"

# Append
echo "line1" > append.txt
echo "line2" >> append.txt
OUT=$(cat append.txt)
EXPECTED="line1
line2"
check_eq "redirect >>" "$EXPECTED" "$OUT"

# tee
echo "tee-test" | tee tee.txt > /dev/null
OUT=$(cat tee.txt)
check_eq "tee" "tee-test" "$OUT"

# /dev/null
echo "gone" > /dev/null
check_exit "/dev/null write" "0" "$?"

echo ""
fi

#-----------------------------------------------------------------------------
if run_phase 12; then
echo "[Phase 12] Command Substitution"
echo "----------------------------------------"

# Basic
VAR=$(echo "value")
check_eq "cmd subst basic" "value" "$VAR"

# Nested
INNER=$(echo "inner")
OUTER=$(echo "got $INNER")
check_eq "cmd subst nested" "got inner" "$OUTER"

# In arithmetic
X=$((2 + 3))
check_eq "arith basic" "5" "$X"

X=$((2 + 3 * 4))
check_eq "arith precedence" "14" "$X"

X=$((10 / 3))
check_eq "arith div" "3" "$X"

X=$((10 % 3))
check_eq "arith mod" "1" "$X"

# Backticks (legacy)
VAR=`echo "backtick"`
check_eq "backtick subst" "backtick" "$VAR"

echo ""
fi

#-----------------------------------------------------------------------------
if run_phase 13; then
echo "[Phase 13] Find"
echo "----------------------------------------"

mkdir -p finddir/sub1/deep
mkdir -p finddir/sub2
touch finddir/a.txt
touch finddir/sub1/b.txt
touch finddir/sub1/deep/c.txt
touch finddir/sub2/d.txt

OUT=$(find finddir -name "*.txt" 2>/dev/null | sort)
EXPECTED="finddir/a.txt
finddir/sub1/b.txt
finddir/sub1/deep/c.txt
finddir/sub2/d.txt"
check_eq "find -name" "$EXPECTED" "$OUT"

OUT=$(find finddir -type d 2>/dev/null | sort)
check_contains "find -type d" "finddir/sub1" "$OUT"

echo ""
fi

#=============================================================================
# Cleanup - remove test directory
cd /root
rm -rf coretest 2>/dev/null

#=============================================================================
echo "========================================"
echo " TEST RESULTS"
echo "========================================"
echo ""
TOTAL=$((PASS + FAIL))
echo "  Passed:  $PASS"
echo "  Failed:  $FAIL"
echo "  Total:   $TOTAL"
echo ""
if [ "$FAIL" -eq 0 ]; then
    echo "  Status:  ALL TESTS PASSED"
    echo ""
    echo "========================================"
    exit 0
else
    echo "  Status:  $FAIL TESTS FAILED"
    echo ""
    echo "========================================"
    exit 1
fi

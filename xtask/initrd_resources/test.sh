#!/bin/ash
# Test script to verify ash shell functionality

echo "=== ASH SHELL TEST ==="
echo ""

# Test 1: Echo
echo "[TEST 1] echo: PASS"

# Test 2: Variables
VAR="hello"
if [ "$VAR" = "hello" ]; then
    echo "[TEST 2] variables: PASS"
else
    echo "[TEST 2] variables: FAIL"
fi

# Test 3: Command substitution
RESULT=$(echo "test")
if [ "$RESULT" = "test" ]; then
    echo "[TEST 3] command substitution: PASS"
else
    echo "[TEST 3] command substitution: FAIL"
fi

# Test 4: Conditionals
if true; then
    echo "[TEST 4] conditionals: PASS"
else
    echo "[TEST 4] conditionals: FAIL"
fi

# Test 5: For loop
COUNT=0
for i in 1 2 3; do
    COUNT=$((COUNT + 1))
done
if [ "$COUNT" = "3" ]; then
    echo "[TEST 5] for loop: PASS"
else
    echo "[TEST 5] for loop: FAIL"
fi

# Test 6: Arithmetic
X=$((2 + 3))
if [ "$X" = "5" ]; then
    echo "[TEST 6] arithmetic: PASS"
else
    echo "[TEST 6] arithmetic: FAIL"
fi

# Test 7: Cat file
if cat /root/hello.txt > /dev/null 2>&1; then
    echo "[TEST 7] cat file: PASS"
else
    echo "[TEST 7] cat file: FAIL"
fi

# Test 8: Ls directory
if ls / > /dev/null 2>&1; then
    echo "[TEST 8] ls directory: PASS"
else
    echo "[TEST 8] ls directory: FAIL"
fi

echo ""
echo "=== ALL TESTS COMPLETE ==="

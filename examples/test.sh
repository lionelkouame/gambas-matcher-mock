#!/bin/bash

# Simple test script for Gambas Matcher Mock
# This script tests various endpoints to ensure the mock server is working correctly

set -e  # Exit on any error

BASE_URL="http://127.0.0.1:8080"
FAILED=0
PASSED=0

# Helper function to test endpoints
test_endpoint() {
    local test_name="$1"
    local url="$2"
    local method="${3:-GET}"
    local headers="$4"
    local expected_status="${5:-200}"
    
    echo "Test: $test_name"
    
    if [ -n "$headers" ]; then
        response=$(curl -s -w "\n%{http_code}" -X "$method" -H "$headers" "$url")
    else
        response=$(curl -s -w "\n%{http_code}" -X "$method" "$url")
    fi
    
    status_code=$(echo "$response" | tail -n 1)
    body=$(echo "$response" | head -n -1)
    
    echo "$body"
    
    if [ "$status_code" -eq "$expected_status" ]; then
        echo "✓ Status code: $status_code (expected: $expected_status)"
        PASSED=$((PASSED + 1))
    else
        echo "✗ Status code: $status_code (expected: $expected_status)"
        FAILED=$((FAILED + 1))
    fi
    echo ""
}

echo "================================================"
echo "Testing Gambas Matcher Mock"
echo "================================================"
echo ""

# Test 1: Basic GET request
test_endpoint "Get User Info" "$BASE_URL/api/users/123" "GET" "" 200

# Test 2: POST request
test_endpoint "Create Order" "$BASE_URL/api/orders" "POST" "" 201

# Test 3: Regex path matching
test_endpoint "Dynamic User ID (regex matching)" "$BASE_URL/api/users/999" "GET" "" 200

# Test 4: Header matching
test_endpoint "Bank Balance API (with Authorization header)" \
    "$BASE_URL/bank/api/v1/balance" "GET" "Authorization: Bearer token123" 200

# Test 5: Query parameter matching
test_endpoint "Product Search (with query parameters)" \
    "$BASE_URL/api/products?category=electronics" "GET" "" 200

# Test 6: Error response
test_endpoint "Simulated Error" "$BASE_URL/api/error" "GET" "" 500

# Test 7: Non-matching request (should return 404)
test_endpoint "Non-matching request (should return 404)" \
    "$BASE_URL/api/nonexistent" "GET" "" 404

echo "================================================"
echo "Test Results:"
echo "  Passed: $PASSED"
echo "  Failed: $FAILED"
echo "================================================"

if [ $FAILED -gt 0 ]; then
    exit 1
else
    echo "All tests passed successfully!"
    exit 0
fi

#!/bin/bash

# Simple test script for Gambas Matcher Mock
# This script tests various endpoints to ensure the mock server is working correctly

echo "Testing Gambas Matcher Mock..."
echo ""

# Test 1: Basic GET request
echo "Test 1: Get User Info"
curl -s http://127.0.0.1:8080/api/users/123
echo -e "\n"

# Test 2: POST request
echo "Test 2: Create Order"
curl -s -X POST http://127.0.0.1:8080/api/orders
echo -e "\n"

# Test 3: Regex path matching
echo "Test 3: Dynamic User ID (regex matching)"
curl -s http://127.0.0.1:8080/api/users/999
echo -e "\n"

# Test 4: Header matching
echo "Test 4: Bank Balance API (with Authorization header)"
curl -s -H "Authorization: Bearer token123" http://127.0.0.1:8080/bank/api/v1/balance
echo -e "\n"

# Test 5: Query parameter matching
echo "Test 5: Product Search (with query parameters)"
curl -s http://127.0.0.1:8080/api/products?category=electronics
echo -e "\n"

# Test 6: Error response
echo "Test 6: Simulated Error"
curl -s http://127.0.0.1:8080/api/error
echo -e "\n"

# Test 7: Non-matching request (should return 404)
echo "Test 7: Non-matching request (should return 404)"
curl -s -w "\nHTTP Status: %{http_code}\n" http://127.0.0.1:8080/api/nonexistent
echo ""

echo "All tests completed!"

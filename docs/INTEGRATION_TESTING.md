# Integration Testing: Consumer & Producer

## Overview

This document describes the integration testing setup for the consumer (Rust server) and producer (JS driver) components.

## Architecture

```
┌─────────────────┐         TCP          ┌─────────────────┐
│   JS Driver     │ ───────────────────> │  Rust Server    │
│   (Producer)    │    (Messages)        │   (Consumer)    │
└─────────────────┘                       └─────────────────┘
```

- **JS Driver**: Sends messages via TCP to the Rust server
- **Rust Server**: Receives messages, parses them, and routes by event name

## Components

### 1. Test Consumer Server (`rust_samples/src/test_consumer_server.rs`)

A dedicated test binary that:
- Starts the Rust transport server
- Listens for TCP connections on `0.0.0.0:49152`
- Receives and processes messages from JS driver
- Logs all received messages for verification

**Usage:**
```bash
cd rust_samples
cargo run --bin test_consumer_server
```

### 2. JS Driver Test Script (`drivers/js/test-consumer-producer.js`)

Comprehensive test suite that:
- Tests connection establishment
- Sends various message formats
- Tests error handling
- Verifies message delivery

**Usage:**
```bash
cd drivers/js
node test-consumer-producer.js
# or
npm test
```

## Test Scenarios

### ✅ Test 1: Connection Test
- Verifies JS driver can connect to Rust server
- Tests connection timeout handling

### ✅ Test 2: Basic Message
- Sends simple text message
- Verifies message is received by server

### ✅ Test 3: Event Message (JSON)
- Sends JSON-formatted event message
- Tests event routing: `{"msg":"...","event_name":"..."}`

### ✅ Test 4: Simple Format
- Sends `event_name:message` format
- Tests simple format parsing

### ✅ Test 5: Batch Messages
- Sends multiple messages sequentially
- Tests message queue handling

### ✅ Test 6: Message Types
- Tests various message formats
- Tests edge cases (special chars, multiline, etc.)

### ✅ Test 7: Error Handling
- Tests connection failures
- Tests invalid addresses

## Message Formats

### 1. Plain Text
```
Hello from JS Driver!
```
→ Parsed as: `event_name="default"`, `msg="Hello from JS Driver!"`

### 2. JSON Format
```json
{"msg":"Test message","event_name":"test_event"}
```
→ Parsed as: `event_name="test_event"`, `msg="Test message"`

### 3. Simple Format
```
user_login:User john_doe logged in
```
→ Parsed as: `event_name="user_login"`, `msg="User john_doe logged in"`

## Running Tests

### Quick Start

**Terminal 1 - Start Consumer:**
```bash
cd rust_samples
cargo run --bin test_consumer_server
```

**Terminal 2 - Run Producer Tests:**
```bash
cd drivers/js
npm test
```

### Expected Output

**Rust Server:**
```
==========================================
Test Consumer Server
==========================================

[Message Consumer] Received: Hello from JS Driver!
[Event Router] Event: 'default', Message: 'Hello from JS Driver!'
Processing message: Hello from JS Driver!

[Message Consumer] Received: {"msg":"Test message","event_name":"test_event"}
[Event Router] Event: 'test_event', Message: 'Test message'
...
```

**JS Driver:**
```
========================================
Consumer-Producer Integration Test
========================================

=== Test 1: Basic Message Sending ===
✓ Driver initialized
✓ Message sent: "Hello from JS Driver!"

=== Test 2: Event-Based Message (JSON) ===
✓ Event message sent: {"msg":"Test message","event_name":"test_event"}

...

✅ All tests passed!
```

## Configuration

### Server Address

**Default:** `0.0.0.0:49152`

**Custom:**
```bash
# Rust server
set TRANSPORT_ADDRESS=0.0.0.0:9000
cargo run --bin test_consumer_server

# JS driver
const driver = Driver.fromAddress('127.0.0.1:9000');
```

### Environment Variables

```bash
# Rust server
TRANSPORT_ADDRESS=0.0.0.0:49152  # Server listen address

# Optional: Add producers for event routing
PRODUCER_CONSUMER1=tcp://127.0.0.1:9000
PRODUCER_CONSUMER2=tcp://127.0.0.1:9001
```

## Troubleshooting

### Connection Issues

**Problem:** `ECONNREFUSED` or connection timeout

**Solutions:**
1. ✅ Ensure Rust server is running first
2. ✅ Check server address matches (default: `127.0.0.1:49152`)
3. ✅ Verify firewall isn't blocking
4. ✅ Check port availability: `netstat -ano | findstr :49152`

### Port Already in Use

**Problem:** `Address already in use` (Rust server)

**Solutions:**
1. ✅ Use different port: `set TRANSPORT_ADDRESS=0.0.0.0:49153`
2. ✅ Find and stop process: `netstat -ano | findstr :49152`
3. ✅ Restart the server

### Messages Not Received

**Problem:** JS sends but server doesn't receive

**Solutions:**
1. ✅ Check server logs for errors
2. ✅ Verify message format (should end with newline)
3. ✅ Check network connectivity
4. ✅ Ensure server listening on `0.0.0.0` (all interfaces)

## Test Coverage

| Component | Coverage | Status |
|-----------|----------|--------|
| Connection | ✅ | Tested |
| Basic Messages | ✅ | Tested |
| JSON Messages | ✅ | Tested |
| Simple Format | ✅ | Tested |
| Batch Messages | ✅ | Tested |
| Error Handling | ✅ | Tested |
| Event Routing | ⚠️ | Needs registered producers |

## Next Steps

1. ✅ Basic integration tests - **Completed**
2. ⚠️ Test with registered producers (event routing)
3. ⚠️ Test concurrent connections
4. ⚠️ Performance testing
5. ⚠️ Load testing

## Files

- `rust_samples/src/test_consumer_server.rs` - Test consumer server binary
- `drivers/js/test-consumer-producer.js` - JS driver test script
- `drivers/js/README_TESTING.md` - Detailed testing guide
- `drivers/js/QUICK_START_TEST.md` - Quick start guide

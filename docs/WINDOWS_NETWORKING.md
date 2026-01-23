# Windows Networking Troubleshooting Guide

## Error 10013: Socket Access Denied

If you encounter error 10013 (`Сделана попытка доступа к сокету методом, запрещенным правами доступа`), here are solutions to fix it:

## Quick Solutions

### 1. Use a Higher Port (Recommended)

The default port has been changed to **49152** (dynamic/ephemeral port range). This port:
- Doesn't require administrator privileges
- Less likely to conflict with other services
- Less likely to be blocked by Windows Firewall

### 2. Override the Address via Environment Variable

```powershell
# Set custom address before running
$env:TRANSPORT_ADDRESS="0.0.0.0:49153"
cargo run

# Or use a different port
$env:TRANSPORT_ADDRESS="127.0.0.1:50000"
cargo run
```

### 3. Check if Port is Already in Use

```powershell
# Check if port 49152 is in use
netstat -ano | findstr :49152

# If port is in use, kill the process (replace PID with actual process ID)
taskkill /PID <PID> /F
```

### 4. Run as Administrator

If you need to use a privileged port (< 1024):

1. Right-click PowerShell/Command Prompt
2. Select "Run as administrator"
3. Navigate to project directory
4. Run `cargo run`

### 5. Windows Firewall Configuration

#### Allow Through Firewall (Recommended)

```powershell
# Add firewall rule for inbound connections
New-NetFirewallRule -DisplayName "Rust Samples TCP" -Direction Inbound -Protocol TCP -LocalPort 49152 -Action Allow
```

#### Temporarily Disable Firewall (For Testing Only)

```powershell
# Check current firewall status
Get-NetFirewallProfile

# Disable firewall temporarily (NOT recommended for production)
Set-NetFirewallProfile -Profile Domain,Public,Private -Enabled False
```

### 6. Windows Defender / Antivirus

Add exceptions for your application:

1. Open **Windows Security** → **Virus & threat protection**
2. Click **Manage settings** under Virus & threat protection settings
3. Scroll to **Exclusions** and click **Add or remove exclusions**
4. Add your project directory or the `.exe` file

### 7. Use 0.0.0.0 Instead of 127.0.0.1

The default address has been changed to `0.0.0.0:49152` which:
- Binds to all network interfaces
- Less likely to have permission issues
- Accessible from other machines on the network

If you want to bind only to localhost:
```powershell
$env:TRANSPORT_ADDRESS="127.0.0.1:49152"
cargo run
```

## Port Recommendations

### Safe Port Ranges for Non-Admin Access

- **49152-65535**: Dynamic/Ephemeral ports (recommended)
- **1024-49151**: Registered ports (usually safe)
- **< 1024**: Well-known ports (requires admin privileges)

### Example Safe Ports

```powershell
# Use any of these ports
$env:TRANSPORT_ADDRESS="0.0.0.0:50000"
$env:TRANSPORT_ADDRESS="0.0.0.0:54321"
$env:TRANSPORT_ADDRESS="0.0.0.0:49152"
```

## Testing the Server

### From Command Line

```powershell
# Using telnet (if enabled)
telnet 127.0.0.1 49152

# Using PowerShell Test-NetConnection
Test-NetConnection -ComputerName 127.0.0.1 -Port 49152

# Using nc (netcat) if installed
nc 127.0.0.1 49152
```

### From Another Application

Connect to `0.0.0.0:49152` or `127.0.0.1:49152` using any TCP client.

## Common Issues and Solutions

### Issue: "Address already in use"

**Solution**: Port is occupied by another process
1. Find the process: `netstat -ano | findstr :49152`
2. Kill it: `taskkill /PID <PID> /F`
3. Or use a different port

### Issue: "Permission denied"

**Solution**: Port requires admin privileges or is blocked
1. Use a port > 49151
2. Run as administrator
3. Check firewall settings
4. Check antivirus exclusions

### Issue: Can't connect from another machine

**Solution**: Binding to 127.0.0.1 only allows local connections
1. Use `0.0.0.0` instead of `127.0.0.1`
2. Check firewall rules for inbound connections
3. Ensure Windows Firewall allows the port

### Issue: Connection works locally but not remotely

**Solution**: Windows Firewall blocking inbound connections
1. Add firewall rule (see above)
2. Check network profile (Domain/Private/Public)
3. Temporarily disable firewall to test

## Best Practices for Windows Development

1. **Use dynamic ports (49152+)**: No admin rights needed
2. **Bind to 0.0.0.0**: Works for both local and remote access
3. **Add firewall exceptions**: Prevents connection issues
4. **Check port availability**: Before binding, verify port is free
5. **Test incrementally**: Start with localhost, then test remote access

## Environment Variable Configuration

You can create a `.env` file or set environment variables:

```powershell
# PowerShell
$env:TRANSPORT_ADDRESS="0.0.0.0:49152"

# Command Prompt
set TRANSPORT_ADDRESS=0.0.0.0:49152

# Make it persistent (Current User)
[System.Environment]::SetEnvironmentVariable("TRANSPORT_ADDRESS", "0.0.0.0:49152", "User")
```

## Still Having Issues?

1. Check Windows Event Viewer for system errors
2. Verify no other service is using the port
3. Try binding to IPv6: `[::]:49152`
4. Check Windows Defender network protection
5. Verify your network adapter settings
6. Try running the compiled executable directly instead of through cargo

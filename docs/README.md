# Core Application Documentation

This directory contains documentation for the **restor-octo-giggle core transport server** (Rust application).

## Core Application Docs

- **README.md** (this file) - Documentation index
- **BUILD.md** - Cross-platform build guide
- **WINDOWS_NETWORKING.md** - Windows networking troubleshooting
- **PRODUCER_POOL.md** - Producer pool system documentation
- **EVENT_ROUTING.md** - Event-based message routing
- **INTEGRATION_TESTING.md** - Integration testing guide
- **TRAITS_EXPLAINED.md** - Explanation of Rust traits vs inheritance

## Driver Documentation

Driver documentation is in separate repositories:

- **JavaScript Driver** - See `../../drivers/js/README.md`
- **Driver Overview** - See `../../drivers/README.md`

## Architecture

The core application provides:
- Transport server for receiving messages
- Event-based routing system
- Producer pool management
- Registration server for runtime consumer registration

## Quick Links

- [Main README](../README.md) - Project overview
- [Build Guide](BUILD.md) - How to build
- [Event Routing](EVENT_ROUTING.md) - How events work
- [Producer Pool](PRODUCER_POOL.md) - Producer management

# @sec-ant/browser-cookies

A lightweight, fast Node.js library for parsing browser cookies using Rust and NAPI-RS.

## Features

- 🚀 **Fast**: Rust-based parsing with zero-copy string handling
- 🌐 **Cross-browser**: Supports Chrome, Firefox, and Safari
- 🎯 **Domain filtering**: Filter cookies by domain/host
- 🔒 **Safe**: Non-blocking database access with temporary file copying
- 💼 **Cross-platform**: Works on Windows, macOS, and Linux
- 📦 **Zero dependencies**: Pure npm package without postinstall scripts

## Installation

```bash
npm install @sec-ant/browser-cookies
```

## Usage

```javascript
const { getCookies, getChromeCookies, getFirefoxCookies } = require('@sec-ant/browser-cookies');

// Get all Chrome cookies
const allCookies = await getChromeCookies();

// Get Chrome cookies for specific domain
const googleCookies = await getChromeCookies('google.com');

// Get Firefox cookies
const firefoxCookies = await getFirefoxCookies('mozilla.org');

// Generic browser API
const cookies = await getCookies('chrome', 'example.com');
```

## API

### Cookie Interface

```typescript
interface Cookie {
  domain: string;
  name: string;
  value: string;
  path?: string;
  expires?: string; // ISO 8601 string
  secure?: boolean;
  httpOnly?: boolean;
}
```

### Functions

#### `getChromeCookies(domain?: string): Promise<Cookie[]>`

Retrieves cookies from Chrome/Chromium browsers.

- `domain` (optional): Filter cookies by domain. Supports partial matching.

#### `getFirefoxCookies(domain?: string): Promise<Cookie[]>`

Retrieves cookies from Firefox browsers.

- `domain` (optional): Filter cookies by domain. Supports partial matching.

#### `getSafariCookies(domain?: string): Cookie[]`

Retrieves cookies from Safari browser (macOS only).

#### `getCookies(browser: string, domain?: string): Promise<Cookie[]>`

Generic function to retrieve cookies from any supported browser.

- `browser`: Browser name ('chrome', 'firefox', or 'safari')
- `domain` (optional): Filter cookies by domain

## Browser Support

| Browser | Windows | macOS | Linux |
|---------|---------|-------|-------|
| Chrome  | ✅      | ✅    | ✅    |
| Firefox | ✅      | ✅    | ✅    |  
| Safari  | ❌      | ✅    | ❌    |

## How it Works

The library directly reads browser cookie databases:

- **Chrome/Chromium**: Reads from `Cookies` SQLite database in browser profile
- **Firefox**: Reads from `cookies.sqlite` in Firefox profile directory
- **Safari**: Reads from `Cookies.binarycookies` file (macOS only)

The library safely copies database files to temporary locations to avoid locking the browser's active databases.

## Performance

Built with Rust and NAPI-RS for maximum performance:
- Zero-copy string operations where possible
- Minimal memory allocations
- Native speed SQLite parsing
- Async/await support without blocking the event loop

## License

LGPL-3.0-or-later

## Notes

- Encrypted cookie values are returned as-is (decryption would require additional dependencies)
- The library gracefully handles missing browsers or inaccessible databases
- Temporary files are automatically cleaned up after use
- Works without requiring browser installation for basic API testing

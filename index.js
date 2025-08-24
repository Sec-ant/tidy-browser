// Simple native binding loader
let nativeBinding;
const { platform, arch } = process;

if (platform === 'linux' && arch === 'x64') {
  nativeBinding = require('./browser_cookies.linux-x64-gnu.node');
} else {
  // Fallback - try to load platform-specific binding
  const binaryName = `browser-cookies.${platform}-${arch}${platform === 'win32' ? '-msvc' : ''}.node`;
  try {
    nativeBinding = require(`./${binaryName}`);
  } catch {
    throw new Error(`Unsupported platform: ${platform}-${arch}`);
  }
}

module.exports = nativeBinding;
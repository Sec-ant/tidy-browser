// Simple native binding loader
let nativeBinding;
const { platform, arch } = process;

if (platform === 'linux' && arch === 'x64') {
  try {
    nativeBinding = require('./browser-cookies.linux-x64-gnu.node');
  } catch (e) {
    throw new Error(`Native binary not found. Please run 'npm run build:debug' first. Error: ${e.message}`);
  }
} else {
  // Fallback - try to load platform-specific binding
  const binaryName = `browser-cookies.${platform}-${arch}${platform === 'win32' ? '-msvc' : ''}.node`;
  try {
    nativeBinding = require(`./${binaryName}`);
  } catch {
    throw new Error(`Unsupported platform: ${platform}-${arch}. Please check if a binary exists for your platform.`);
  }
}

module.exports = nativeBinding;
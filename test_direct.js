const binding = require('./browser_cookies.linux-x64-gnu.node');

console.log('Direct binding object:', binding);
console.log('Available keys:', Object.keys(binding));

for (const key of Object.keys(binding)) {
  console.log(`${key}:`, typeof binding[key]);
}

try {
  console.log('Testing getChromeCookies directly...');
  const result = binding.getChromeCookies('google.com');
  console.log('Direct result:', result);
  console.log('Type of result:', typeof result);
  console.log('Array.isArray(result):', Array.isArray(result));
} catch (error) {
  console.error('Direct test failed:', error);
}
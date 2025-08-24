const { test } = require('node:test');
const assert = require('node:assert');
const { execSync } = require('child_process');
const fs = require('fs');
const path = require('path');

test('Real cookie parsing with mock database', async (t) => {
  // Skip if sqlite3 is not available
  try {
    execSync('which sqlite3', { stdio: 'ignore' });
  } catch (e) {
    t.skip('sqlite3 not available');
    return;
  }
  
  const { getChromeCookies } = require('../index.js');
  
  // Create test database
  const testDbPath = '/tmp/test_chrome_cookies_integration.db';
  const testChromeDir = '/tmp/test_chrome_integration';
  const testDefaultDir = path.join(testChromeDir, 'Default');
  
  // Cleanup previous test
  if (fs.existsSync(testDbPath)) fs.unlinkSync(testDbPath);
  if (fs.existsSync(testChromeDir)) fs.rmSync(testChromeDir, { recursive: true });
  
  const createDb = `
CREATE TABLE cookies (
  host_key TEXT,
  name TEXT,
  value TEXT,
  path TEXT,
  expires_utc INTEGER,
  is_secure INTEGER,
  is_httponly INTEGER
);

INSERT INTO cookies VALUES 
  ('google.com', 'session_id', 'test_value_123', '/', 4102444800000000, 1, 0),
  ('github.com', 'auth_token', 'github_auth_456', '/api', 4102444800000000, 1, 1);
`;
  
  try {
    // Create test database
    execSync(`sqlite3 "${testDbPath}" "${createDb}"`);
    
    // Set up directory structure
    fs.mkdirSync(testDefaultDir, { recursive: true });
    fs.copyFileSync(testDbPath, path.join(testDefaultDir, 'Cookies'));
    
    // Set environment variable for testing
    process.env.TEST_CHROME_PATH = testChromeDir;
    
    // Test the actual parsing
    const cookies = await getChromeCookies();
    
    assert(Array.isArray(cookies));
    assert.strictEqual(cookies.length, 2);
    
    const googleCookie = cookies.find(c => c.domain === 'google.com');
    assert(googleCookie);
    assert.strictEqual(googleCookie.name, 'session_id');
    assert.strictEqual(googleCookie.value, 'test_value_123');
    assert.strictEqual(googleCookie.path, '/');
    assert.strictEqual(googleCookie.secure, true);
    assert.strictEqual(googleCookie.httpOnly, false);
    
    const githubCookie = cookies.find(c => c.domain === 'github.com');
    assert(githubCookie);
    assert.strictEqual(githubCookie.name, 'auth_token');
    assert.strictEqual(githubCookie.value, 'github_auth_456');
    assert.strictEqual(githubCookie.path, '/api');
    assert.strictEqual(githubCookie.secure, true);
    assert.strictEqual(githubCookie.httpOnly, true);
    
    // Test domain filtering
    const filteredCookies = await getChromeCookies('google.com');
    assert.strictEqual(filteredCookies.length, 1);
    assert.strictEqual(filteredCookies[0].domain, 'google.com');
    
  } finally {
    // Cleanup
    delete process.env.TEST_CHROME_PATH;
    if (fs.existsSync(testDbPath)) fs.unlinkSync(testDbPath);
    if (fs.existsSync(testChromeDir)) fs.rmSync(testChromeDir, { recursive: true });
  }
});
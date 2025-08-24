const { test } = require('node:test');
const assert = require('node:assert');
const { getCookies, getChromeCookies, getFirefoxCookies, getSafariCookies } = require('../index.js');

test('API exports exist', async (t) => {
  assert.strictEqual(typeof getChromeCookies, 'function');
  assert.strictEqual(typeof getFirefoxCookies, 'function');
  assert.strictEqual(typeof getSafariCookies, 'function');
  assert.strictEqual(typeof getCookies, 'function');
});

test('Browser not installed - returns empty arrays', async (t) => {
  const chromeCookies = await getChromeCookies();
  const firefoxCookies = await getFirefoxCookies();
  const safariCookies = getSafariCookies();
  
  assert(Array.isArray(chromeCookies));
  assert(Array.isArray(firefoxCookies));
  assert(Array.isArray(safariCookies));
  
  // In CI environment, browsers aren't installed
  assert.strictEqual(chromeCookies.length, 0);
  assert.strictEqual(firefoxCookies.length, 0);
  assert.strictEqual(safariCookies.length, 0);
});

test('Invalid browser name throws error', async (t) => {
  await assert.rejects(
    getCookies('invalid-browser', 'example.com'),
    /Unsupported browser: invalid-browser/
  );
});

test('Domain filtering parameter is accepted', async (t) => {
  // Should not throw
  const cookies = await getChromeCookies('example.com');
  assert(Array.isArray(cookies));
});
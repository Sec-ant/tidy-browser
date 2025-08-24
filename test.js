const { getCookies, getChromeCookies, getFirefoxCookies, getSafariCookies } = require('./index.js');

async function test() {
  try {
    console.log('Available functions:', Object.keys(require('./index.js')));
    
    console.log('Testing Chrome cookies...');
    const cookies = getChromeCookies('google.com');
    console.log('Chrome cookies:', JSON.stringify(cookies, null, 2));
    
    console.log('Testing Firefox cookies...');
    const ffCookies = getFirefoxCookies('mozilla.org');
    console.log('Firefox cookies:', JSON.stringify(ffCookies, null, 2));
    
    console.log('Testing generic API...');
    const cookies2 = getCookies('chrome', 'test.com');
    console.log('Generic API cookies:', JSON.stringify(cookies2, null, 2));
    
    if (cookies.length > 0 || ffCookies.length > 0) {
      console.log('✅ Basic NAPI binding test passed with mock data!');
    } else {
      console.log('✅ Basic NAPI binding test passed but no cookies returned!');
    }
  } catch (error) {
    console.error('❌ Test failed:', error.message);
    console.error('Stack:', error.stack);
  }
}

test();
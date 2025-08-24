const { getCookies, getChromeCookies, getFirefoxCookies, getSafariCookies } = require('@sec-ant/browser-cookies');

async function main() {
  try {
    console.log('🍪 Browser Cookies Parser Example\n');

    // Get all Chrome cookies
    console.log('📊 Getting all Chrome cookies...');
    const allChromeCookies = await getChromeCookies();
    console.log(`Found ${allChromeCookies.length} Chrome cookies`);
    
    if (allChromeCookies.length > 0) {
      console.log('Sample Chrome cookie:', allChromeCookies[0]);
    }

    // Get Chrome cookies for specific domain
    console.log('\n🎯 Getting Chrome cookies for google.com...');
    const googleCookies = await getChromeCookies('google.com');
    console.log(`Found ${googleCookies.length} Google cookies`);
    
    googleCookies.forEach(cookie => {
      console.log(`  - ${cookie.name}: ${cookie.value} (domain: ${cookie.domain})`);
    });

    // Get Firefox cookies
    console.log('\n🦊 Getting Firefox cookies...');
    const firefoxCookies = await getFirefoxCookies();
    console.log(`Found ${firefoxCookies.length} Firefox cookies`);

    // Use generic API
    console.log('\n⚙️  Using generic API for Chrome...');
    const genericChrome = await getCookies('chrome', 'github.com');
    console.log(`Found ${genericChrome.length} GitHub cookies via generic API`);

    // Handle Safari (if on macOS)
    console.log('\n🍎 Getting Safari cookies...');
    const safariCookies = getSafariCookies();
    console.log(`Found ${safariCookies.length} Safari cookies`);

    console.log('\n✅ Example completed successfully!');

  } catch (error) {
    console.error('❌ Error:', error.message);
  }
}

main();
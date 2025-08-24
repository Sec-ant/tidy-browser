export interface Cookie {
  domain: string;
  name: string;
  value: string;
  path?: string;
  expires?: string;
  secure?: boolean;
  httpOnly?: boolean;
}

export function getChromeCookies(domain?: string): Cookie[];
export function getFirefoxCookies(domain?: string): Cookie[];
export function getSafariCookies(domain?: string): Cookie[];
export function getCookies(browser: string, domain?: string): Cookie[];
export interface Cookie {
  domain: string;
  name: string;
  value: string;
  path?: string;
  expires?: string;
  secure?: boolean;
  httpOnly?: boolean;
}

export function getChromeCookies(domain?: string): Promise<Cookie[]>;
export function getFirefoxCookies(domain?: string): Promise<Cookie[]>;
export function getSafariCookies(domain?: string): Cookie[];
export function getCookies(browser: string, domain?: string): Promise<Cookie[]>;
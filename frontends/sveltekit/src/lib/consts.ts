const authServiceRestBaseUrl: string | undefined = import.meta.env.VITE_AUTH_SERVICE_BASE_URL;
if (typeof authServiceRestBaseUrl === undefined || !authServiceRestBaseUrl) {
	throw new Error('AUTH_SERVICE_BASE_URL is not defined');
}
//TODO: Idealy we should generate a client from the backend OpenAPI spec and use that client instead of these values and the functions that reference them.
const formattedBaseUrl: string = authServiceRestBaseUrl.endsWith('/')
	? authServiceRestBaseUrl.slice(0, -1)
	: authServiceRestBaseUrl;
export const AUTH_SERVICE_LOGOUT_URL: string = `${formattedBaseUrl}/logout`;
export const AUTH_SERVICE_LOGIN_URL: string = `${formattedBaseUrl}/login`;
export const AUTH_TOKEN_COOKIE_NAME: string = 'veloxide_auth_token';
export const AUTH_TOKEN_COOKIE_DOMAIN: string =
	import.meta.env.VITE_AUTH_TOKEN_COOKIE_DOMAIN || 'veloxide.dev';

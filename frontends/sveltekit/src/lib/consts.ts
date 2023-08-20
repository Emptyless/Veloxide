const authServiceRestBaseUrl: string | undefined = import.meta.env.VITE_AUTH_SERVICE_BASE_URL;
if (typeof authServiceRestBaseUrl === undefined || !authServiceRestBaseUrl) {
	throw new Error('AUTH_SERVICE_BASE_URL is not defined');
}
//TODO: Idealy we should generate a client from the backend OpenAPI spec and use that client instead of these values and the functions that reference them.
export const AUTH_SERVICE_LOGOUT_URL: string = `${authServiceRestBaseUrl}/logout`;
export const AUTH_SERVICE_LOGIN_URL: string = `${authServiceRestBaseUrl}/login`;
export const AUTH_TOKEN_COOKIE_NAME: string = 'veloxide_auth_token';

import type { GetCurrentUserRequest, UserView } from '$lib/stubs/auth';
import type { RpcOptions } from '@protobuf-ts/runtime-rpc';
import { authClient } from '$lib/authClient';
import type { LayoutServerLoad } from './$types';
import { user } from '$lib/stores/userStore';
import { AUTH_TOKEN_COOKIE_NAME } from '$lib/consts';

interface LoadResult {
	authToken: string | undefined;
	user: Partial<UserView> | undefined;
}

export const load: LayoutServerLoad = async ({ cookies }): Promise<LoadResult> => {
	const authToken = cookies.get(AUTH_TOKEN_COOKIE_NAME);
	let userView: Partial<UserView> | undefined;

	if (typeof authToken === 'string') {
		const request: GetCurrentUserRequest = { token: authToken };
		const options: RpcOptions = { auth_token: authToken, meta: {} };

		try {
			const response = await authClient.getCurrentUser(request, options);
			const { user: userResponse } = response.response;

			if (userResponse) {
				const { id, name, email, verifiedEmail, givenName, familyName, picture, locale } =
					userResponse;
				userView = { id, name, email, verifiedEmail, givenName, familyName, picture, locale };
				user.set(userView);
			} else {
				cookies.delete(AUTH_TOKEN_COOKIE_NAME);
				userView = undefined;
			}
		} catch (error) {
			cookies.delete(AUTH_TOKEN_COOKIE_NAME);
			userView = undefined;
			console.error('An error occurred while fetching the current user:', error);
		}
	}
	return { authToken: authToken, user: userView };
};

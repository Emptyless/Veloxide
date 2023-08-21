import type { GetCurrentUserRequest, UserView } from '$lib/stubs/auth';
import type { RpcOptions } from '@protobuf-ts/runtime-rpc';
import { authClient } from '$lib/authClient';
import type { LayoutServerLoad } from './$types';
import { user } from '$lib/stores/userStore';
import { AUTH_TOKEN_COOKIE_NAME } from '$lib/consts';

export const load: LayoutServerLoad = async ({ cookies }) => {
	const authToken = cookies.get(AUTH_TOKEN_COOKIE_NAME);
	let userView: Partial<UserView> | undefined;
	if (typeof authToken === 'string') {
		const request: GetCurrentUserRequest = { token: authToken };
		const options: RpcOptions = { auth_token: authToken, meta: {} };
		const response = await authClient.getCurrentUser(request, options);
		const userResponse = response.response.user;
		if (userResponse) {
			userView = {
				id: userResponse.id,
				name: userResponse.name,
				email: userResponse.email,
				emailVerified: userResponse.emailVerified,
				image: userResponse.image
			};
			user.set(userView);
		} else {
			cookies.delete(AUTH_TOKEN_COOKIE_NAME);
			userView = undefined;
		}
	}
	return { authToken: authToken, user: userView };
};

import type { Handle } from '@sveltejs/kit';
import { Telemetry } from './lib/instrumentation';

export const handle: Handle = async ({ event, resolve }) => {
	Telemetry.getInstance().start();
	return resolve(event);
};

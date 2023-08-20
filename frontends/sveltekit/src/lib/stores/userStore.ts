import type { UserView } from '$lib/stubs/auth';
import { writable } from 'svelte/store';

export const user = writable<Partial<UserView> | undefined>(undefined);

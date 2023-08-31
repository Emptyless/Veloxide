import { z } from 'zod';
export const URLschema = z.string().url();

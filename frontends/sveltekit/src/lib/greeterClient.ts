import { GrpcWebFetchTransport } from '@protobuf-ts/grpcweb-transport';
import { GreeterClient } from '$lib/stubs/helloworld.client';

const baseUrl: string | undefined = import.meta.env.VITE_GRPC_SERVER_BASE_URL;

if (typeof baseUrl !== 'string' || !baseUrl) {
	throw new Error('GRPC_SERVER_BASE_URL is not defined or is not a valid URL');
}

const transport = new GrpcWebFetchTransport({ baseUrl });

export const greeterClient = new GreeterClient(transport);

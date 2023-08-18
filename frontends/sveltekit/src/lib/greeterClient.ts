import { GrpcWebFetchTransport } from '@protobuf-ts/grpcweb-transport';
import { GreeterClient } from '$lib/stubs/helloworld.client';
const baseUrl: string = import.meta.env.VITE_GRPC_SERVER_BASE_URL;
const transport = new GrpcWebFetchTransport({ baseUrl });
export const greeterClient = new GreeterClient(transport);

import { BankAccountServiceClient } from '$lib/stubs/bank_account_service.client';
import { GrpcWebFetchTransport } from '@protobuf-ts/grpcweb-transport';
const baseUrl: string = import.meta.env.VITE_GRPC_SERVER_BASE_URL;
const transport = new GrpcWebFetchTransport({ baseUrl });
export const bankAccountClient = new BankAccountServiceClient(transport);


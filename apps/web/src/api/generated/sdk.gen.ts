/* Generated from the Rust OpenAPI document. Do not edit. */

import type { Client, ClientMeta, Options as Options2, RequestResult, TDataShape } from './client';
import { client } from './client.gen';
import type { GetBootstrapStateData, GetBootstrapStateErrors, GetBootstrapStateResponses, GetLivenessData, GetLivenessResponses, GetReadinessData, GetReadinessErrors, GetReadinessResponses, GetSystemVersionData, GetSystemVersionResponses, InitializeControlPlaneData, InitializeControlPlaneErrors, InitializeControlPlaneResponses } from './types.gen';

export type Options<TData extends TDataShape = TDataShape, ThrowOnError extends boolean = boolean, TResponse = unknown> = Options2<TData, ThrowOnError, TResponse> & {
    /**
     * You can provide a client instance returned by `createClient()` instead of
     * individual options. This might be also useful if you want to implement a
     * custom client.
     */
    client?: Client;
    /**
     * You can pass arbitrary values through the `meta` object. This can be
     * used to access values that aren't defined as part of the SDK function.
     */
    meta?: keyof ClientMeta extends never ? Record<string, unknown> : ClientMeta;
};

export const getBootstrapState = <ThrowOnError extends boolean = false>(options?: Options<GetBootstrapStateData, ThrowOnError>): RequestResult<GetBootstrapStateResponses, GetBootstrapStateErrors, ThrowOnError> => (options?.client ?? client).get<GetBootstrapStateResponses, GetBootstrapStateErrors, ThrowOnError>({ url: '/api/v1/bootstrap', ...options });

export const initializeControlPlane = <ThrowOnError extends boolean = false>(options: Options<InitializeControlPlaneData, ThrowOnError>): RequestResult<InitializeControlPlaneResponses, InitializeControlPlaneErrors, ThrowOnError> => (options.client ?? client).post<InitializeControlPlaneResponses, InitializeControlPlaneErrors, ThrowOnError>({
    url: '/api/v1/bootstrap',
    ...options,
    headers: {
        'Content-Type': 'application/json',
        ...options.headers
    }
});

export const getSystemVersion = <ThrowOnError extends boolean = false>(options?: Options<GetSystemVersionData, ThrowOnError>): RequestResult<GetSystemVersionResponses, unknown, ThrowOnError> => (options?.client ?? client).get<GetSystemVersionResponses, unknown, ThrowOnError>({ url: '/api/v1/system/version', ...options });

export const getLiveness = <ThrowOnError extends boolean = false>(options?: Options<GetLivenessData, ThrowOnError>): RequestResult<GetLivenessResponses, unknown, ThrowOnError> => (options?.client ?? client).get<GetLivenessResponses, unknown, ThrowOnError>({ url: '/healthz', ...options });

export const getReadiness = <ThrowOnError extends boolean = false>(options?: Options<GetReadinessData, ThrowOnError>): RequestResult<GetReadinessResponses, GetReadinessErrors, ThrowOnError> => (options?.client ?? client).get<GetReadinessResponses, GetReadinessErrors, ThrowOnError>({ url: '/readyz', ...options });

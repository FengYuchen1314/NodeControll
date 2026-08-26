/* Generated from the Rust OpenAPI document. Do not edit. */

import type { Client, ClientMeta, Options as Options2, RequestResult, TDataShape } from './client';
import { client } from './client.gen';
import type { ChangeCurrentPasswordData, ChangeCurrentPasswordErrors, ChangeCurrentPasswordResponses, GetBootstrapStateData, GetBootstrapStateErrors, GetBootstrapStateResponses, GetCurrentActorData, GetCurrentActorErrors, GetCurrentActorResponses, GetLivenessData, GetLivenessResponses, GetReadinessData, GetReadinessErrors, GetReadinessResponses, GetSystemVersionData, GetSystemVersionResponses, InitializeControlPlaneData, InitializeControlPlaneErrors, InitializeControlPlaneResponses, ListCurrentSessionsData, ListCurrentSessionsErrors, ListCurrentSessionsResponses, LoginData, LoginErrors, LoginResponses, LogoutAllData, LogoutAllErrors, LogoutAllResponses, LogoutData, LogoutErrors, LogoutResponses, ReauthenticateData, ReauthenticateErrors, ReauthenticateResponses, RevokeCurrentUserSessionData, RevokeCurrentUserSessionErrors, RevokeCurrentUserSessionResponses } from './types.gen';

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

export const login = <ThrowOnError extends boolean = false>(options: Options<LoginData, ThrowOnError>): RequestResult<LoginResponses, LoginErrors, ThrowOnError> => (options.client ?? client).post<LoginResponses, LoginErrors, ThrowOnError>({
    url: '/api/v1/auth/login',
    ...options,
    headers: {
        'Content-Type': 'application/json',
        ...options.headers
    }
});

export const logout = <ThrowOnError extends boolean = false>(options?: Options<LogoutData, ThrowOnError>): RequestResult<LogoutResponses, LogoutErrors, ThrowOnError> => (options?.client ?? client).post<LogoutResponses, LogoutErrors, ThrowOnError>({
    security: [{ name: 'x-nodecontroll-csrf', type: 'apiKey' }, {
            in: 'cookie',
            name: '__Host-nodecontroll_session',
            type: 'apiKey'
        }],
    url: '/api/v1/auth/logout',
    ...options
});

export const logoutAll = <ThrowOnError extends boolean = false>(options: Options<LogoutAllData, ThrowOnError>): RequestResult<LogoutAllResponses, LogoutAllErrors, ThrowOnError> => (options.client ?? client).post<LogoutAllResponses, LogoutAllErrors, ThrowOnError>({
    security: [{ name: 'x-nodecontroll-csrf', type: 'apiKey' }, {
            in: 'cookie',
            name: '__Host-nodecontroll_session',
            type: 'apiKey'
        }],
    url: '/api/v1/auth/logout-all',
    ...options,
    headers: {
        'Content-Type': 'application/json',
        ...options.headers
    }
});

export const reauthenticate = <ThrowOnError extends boolean = false>(options: Options<ReauthenticateData, ThrowOnError>): RequestResult<ReauthenticateResponses, ReauthenticateErrors, ThrowOnError> => (options.client ?? client).post<ReauthenticateResponses, ReauthenticateErrors, ThrowOnError>({
    security: [{ name: 'x-nodecontroll-csrf', type: 'apiKey' }, {
            in: 'cookie',
            name: '__Host-nodecontroll_session',
            type: 'apiKey'
        }],
    url: '/api/v1/auth/reauth',
    ...options,
    headers: {
        'Content-Type': 'application/json',
        ...options.headers
    }
});

export const getBootstrapState = <ThrowOnError extends boolean = false>(options?: Options<GetBootstrapStateData, ThrowOnError>): RequestResult<GetBootstrapStateResponses, GetBootstrapStateErrors, ThrowOnError> => (options?.client ?? client).get<GetBootstrapStateResponses, GetBootstrapStateErrors, ThrowOnError>({ url: '/api/v1/bootstrap', ...options });

export const initializeControlPlane = <ThrowOnError extends boolean = false>(options: Options<InitializeControlPlaneData, ThrowOnError>): RequestResult<InitializeControlPlaneResponses, InitializeControlPlaneErrors, ThrowOnError> => (options.client ?? client).post<InitializeControlPlaneResponses, InitializeControlPlaneErrors, ThrowOnError>({
    url: '/api/v1/bootstrap',
    ...options,
    headers: {
        'Content-Type': 'application/json',
        ...options.headers
    }
});

export const getCurrentActor = <ThrowOnError extends boolean = false>(options?: Options<GetCurrentActorData, ThrowOnError>): RequestResult<GetCurrentActorResponses, GetCurrentActorErrors, ThrowOnError> => (options?.client ?? client).get<GetCurrentActorResponses, GetCurrentActorErrors, ThrowOnError>({
    security: [{
            in: 'cookie',
            name: '__Host-nodecontroll_session',
            type: 'apiKey'
        }],
    url: '/api/v1/me',
    ...options
});

export const changeCurrentPassword = <ThrowOnError extends boolean = false>(options: Options<ChangeCurrentPasswordData, ThrowOnError>): RequestResult<ChangeCurrentPasswordResponses, ChangeCurrentPasswordErrors, ThrowOnError> => (options.client ?? client).post<ChangeCurrentPasswordResponses, ChangeCurrentPasswordErrors, ThrowOnError>({
    security: [{ name: 'x-nodecontroll-csrf', type: 'apiKey' }, {
            in: 'cookie',
            name: '__Host-nodecontroll_session',
            type: 'apiKey'
        }],
    url: '/api/v1/me/password',
    ...options,
    headers: {
        'Content-Type': 'application/json',
        ...options.headers
    }
});

export const listCurrentSessions = <ThrowOnError extends boolean = false>(options?: Options<ListCurrentSessionsData, ThrowOnError>): RequestResult<ListCurrentSessionsResponses, ListCurrentSessionsErrors, ThrowOnError> => (options?.client ?? client).get<ListCurrentSessionsResponses, ListCurrentSessionsErrors, ThrowOnError>({
    security: [{
            in: 'cookie',
            name: '__Host-nodecontroll_session',
            type: 'apiKey'
        }],
    url: '/api/v1/me/sessions',
    ...options
});

export const revokeCurrentUserSession = <ThrowOnError extends boolean = false>(options: Options<RevokeCurrentUserSessionData, ThrowOnError>): RequestResult<RevokeCurrentUserSessionResponses, RevokeCurrentUserSessionErrors, ThrowOnError> => (options.client ?? client).delete<RevokeCurrentUserSessionResponses, RevokeCurrentUserSessionErrors, ThrowOnError>({
    security: [{ name: 'x-nodecontroll-csrf', type: 'apiKey' }, {
            in: 'cookie',
            name: '__Host-nodecontroll_session',
            type: 'apiKey'
        }],
    url: '/api/v1/me/sessions/{session_id}',
    ...options
});

export const getSystemVersion = <ThrowOnError extends boolean = false>(options?: Options<GetSystemVersionData, ThrowOnError>): RequestResult<GetSystemVersionResponses, unknown, ThrowOnError> => (options?.client ?? client).get<GetSystemVersionResponses, unknown, ThrowOnError>({ url: '/api/v1/system/version', ...options });

export const getLiveness = <ThrowOnError extends boolean = false>(options?: Options<GetLivenessData, ThrowOnError>): RequestResult<GetLivenessResponses, unknown, ThrowOnError> => (options?.client ?? client).get<GetLivenessResponses, unknown, ThrowOnError>({ url: '/healthz', ...options });

export const getReadiness = <ThrowOnError extends boolean = false>(options?: Options<GetReadinessData, ThrowOnError>): RequestResult<GetReadinessResponses, GetReadinessErrors, ThrowOnError> => (options?.client ?? client).get<GetReadinessResponses, GetReadinessErrors, ThrowOnError>({ url: '/readyz', ...options });

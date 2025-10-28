// src/lib/api.ts

import { PUBLIC_AKSARA_API_URL } from '$env/static/public';

// APIError class remains the same
export class APIError extends Error {
  constructor(message: string, public status: number) {
    super(message);
    this.name = 'APIError';
  }
}

// --- MODIFICATION STARTS HERE ---

// The core request function now promises to return T OR null
async function request<T>(
    fetch: typeof window.fetch,
    method: 'GET' | 'POST' | 'PUT' | 'DELETE',
    endpoint: string,
    data?: object
): Promise<T | null> { // <--- Changed from Promise<T> to Promise<T | null>
    const url = `${PUBLIC_AKSARA_API_URL}/${endpoint}`;

    const options: RequestInit = {
        method,
        headers: {
            'Content-Type': 'application/json',
        },
    };

    if (data && (method === 'POST' || method === 'PUT')) {
        options.body = JSON.stringify(data);
    }

    const response = await fetch(url, options);

    if (!response.ok) {
        const errorData = await response.json().catch(() => ({ message: 'An unexpected error occurred.' }));
        throw new APIError(errorData.message || 'Server error', response.status);
    }

    // Explicitly handle 204 No Content, which has no body
    if (response.status === 204) {
        return null;
    }

    const text = await response.text();

    // If the body is empty text, return null. Otherwise, parse the JSON.
    return text ? JSON.parse(text) : null;
}

// Update the return types for each method in the exported 'api' object
export const api = {
    get: <T>(fetch: typeof window.fetch, endpoint: string): Promise<T | null> =>
        request<T>(fetch, 'GET', endpoint),

    post: <T>(fetch: typeof window.fetch, endpoint: string, data: object): Promise<T | null> =>
        request<T>(fetch, 'POST', endpoint, data),

    put: <T>(fetch: typeof window.fetch, endpoint: string, data: object): Promise<T | null> =>
        request<T>(fetch, 'PUT', endpoint, data),

    delete: <T>(fetch: typeof window.fetch, endpoint: string): Promise<T | null> =>
        request<T>(fetch, 'DELETE', endpoint),
};
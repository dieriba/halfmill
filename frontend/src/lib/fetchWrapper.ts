/* eslint-disable @typescript-eslint/no-explicit-any */
import { BACKEND_URL } from '$env/static/private';

class Wrapper {
	constructor(private readonly backendUrl: string) {
		backendUrl = BACKEND_URL;
	}

	get(url: string, init?: RequestInit): Promise<Response> {
		return fetch(`${this.backendUrl}${url}`, {
			headers: {
				'content-type': 'application/json'
			},
			method: 'GET',
			...init
		});
	}

	post(url: string, data: any, init?: RequestInit): Promise<Response> {
		return fetch(`${this.backendUrl}${url}`, {
			headers: {
				'content-type': 'application/json'
			},
			body: JSON.stringify(data),
			method: 'POST',
			...init
		});
	}
}

export const fetchWrapper = new Wrapper(BACKEND_URL);

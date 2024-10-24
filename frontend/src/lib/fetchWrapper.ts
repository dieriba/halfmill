/* eslint-disable @typescript-eslint/no-explicit-any */
import { BACKEND_URL } from '$env/static/private';

type FetchType = (input: RequestInfo | URL, init?: RequestInit) => Promise<Response>;
type RequestData = { data: any; init?: RequestInit };
class Wrapper {
	constructor(private readonly backendUrl: string) {
		backendUrl = BACKEND_URL;
	}

	get(fn: FetchType, url: string, init?: RequestInit): Promise<Response> {
		const fullUrl = `${this.backendUrl}${url}`;
		const data = {
			headers: {
				'content-type': 'application/json'
			},
			method: 'GET',
			...init
		};
		return fn(fullUrl, data);
	}

	post(fn: FetchType, url: string, { data, init }: RequestData): Promise<Response> {
		const fullUrl = `${this.backendUrl}${url}`;
		const requestData = {
			headers: {
				'content-type': 'application/json'
			},
			body: JSON.stringify(data),
			method: 'POST',
			...init
		};
		return fn(fullUrl, requestData);
	}
}

export const fetchWrapper = new Wrapper(BACKEND_URL);

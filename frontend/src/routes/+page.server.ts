import { fetchWrapper } from '$lib/fetchWrapper.js';

/** @type {import('./$types').Actions} */
export const actions = {
	script: async ({ request, fetch }) => {
		const formData = await request.formData();
		const data = {
			code: formData.get('code'),
			language: formData.get('language')
		};
		console.log({ data });
		const response = await fetchWrapper.post(fetch, '/script/run', { data });

		const result = await response.json();

		console.log({ result });
	},
	logout: async ({ cookies }) => {
		console.log('Entered');
		cookies.delete('access_token', { path: '' });
		cookies.delete('refresh_token', { path: '' });
	}
};

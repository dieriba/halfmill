import { fetchWrapper } from '$lib/fetchWrapper.js';
import { fail, redirect } from '@sveltejs/kit';
import { z } from 'zod';

const LoginSchema = z.object({
	username: z.string().min(1, 'Username must at least contains 1 character'),
	password: z.string().min(1, 'Password must at least contains 1 character')
});

type LoginType = z.infer<typeof LoginSchema>;

/** @type {import('./$types').Actions} */
export const actions = {
	login: async ({ cookies, request }) => {
		const data = await request.formData();
		const login: LoginType = {
			username: data.get('username') as string,
			password: data.get('password') as string
		};

		console.log(login);

		const result = await LoginSchema.safeParseAsync(login);

		if (result.success == false) {
			console.log({ success: false, errors: result.error.flatten().fieldErrors });
			return fail(400, { success: false, errors: result.error.flatten().fieldErrors });
		}

		const response = await fetchWrapper.post('/auth/signin', login);

		const res = await response.json();

		if (response.status >= 400) {
			return fail(400, { success: true, message: res.message });
		}

		const cookieHeader = response.headers.get('Set-Cookie')?.split(',');

		cookieHeader?.map((cookie) => {
			console.log(cookie);
		});

		return { success: true };

		//throw redirect(302, '/');
	}
};

import { fetchWrapper } from '$lib/fetchWrapper.js';
import { fail } from '@sveltejs/kit';
import { z } from 'zod';

const RegisterSchema = z
	.object({
		username: z
			.string()
			.min(1, 'Username must at least contains 3 characters')
			.max(20, 'Username must at most contains 20 characters'),
		password: z
			.string()
			.min(1, 'Password must at least contains 8 characters')
			.max(50, 'Password must at most contains 50 characters'),
		confirmPassword: z.string().min(1, 'Confirm Password must at least contains 8 characters')
	})
	.refine((data) => data.password === data.confirmPassword, {
		message: "Passwords don't match",
		path: ['confirmPassword']
	});

type RegisterType = z.infer<typeof RegisterSchema>;

/** @type {import('./$types').Actions} */
export const actions = {
	register: async ({ request }) => {
		const data = await request.formData();
		const register: RegisterType = {
			username: data.get('username') as string,
			password: data.get('password') as string,
			confirmPassword: data.get('confirmPassword') as string
		};

		console.log(register);

		const result = await RegisterSchema.safeParseAsync(register);

		if (result.success == false) {
			console.log({ success: false, errors: result.error.flatten().fieldErrors });
			return fail(400, { success: false, errors: result.error.flatten().fieldErrors });
		}

		const response = await fetchWrapper.post(fetch, '/auth/signup', { data: register });

		const res = await response.json();
		console.log(res);

		const success = response.status < 400;

		return { success, message: Array.isArray(res.message) ? res.message : [res.message] };
	}
};

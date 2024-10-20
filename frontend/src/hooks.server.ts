import routes from './lib/routes';
import { redirect } from '@sveltejs/kit';
const public_paths = [routes.signin, routes.signup];

/** @type {import('@sveltejs/kit').Handle} */
export async function handle({ event, resolve }) {
	const access_token = event.cookies.get('access_token');
	const url = new URL(event.request.url);

	if (!access_token && !public_paths.includes(url.pathname)) {
		throw redirect(302, '/auth/signin');
	}
	if (access_token && (url.pathname == '/auth/signup' || url.pathname == '/signin')) {
		throw redirect(302, routes.home);
	}
	const response = await resolve(event);
	return response;
}

/** @type {import('@sveltejs/kit').HandleFetch} */
export async function handleFetch({ event, request, fetch }) {
	return fetch(request);
}

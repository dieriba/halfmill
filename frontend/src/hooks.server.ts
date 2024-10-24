import routes from '$lib/routes';
import { redirect } from '@sveltejs/kit';
const publicPaths = [routes.login, routes.signup];

/** @type {import('@sveltejs/kit').Handle} */
export async function handle({ event, resolve }) {
	const access_token = event.cookies.get('access_token');
	const url = new URL(event.request.url);
	const isNotPublicUrl =
		publicPaths.filter((publicPath) => {
			if (url.pathname.startsWith(publicPath)) {
				return true;
			}
			return false;
		}).length == 0;
	if (!access_token && isNotPublicUrl == true) {
		redirect(302, routes.login);
	}
	if (access_token && isNotPublicUrl == false) {
		redirect(302, routes.home);
	}
	const response = await resolve(event);
	return response;
}

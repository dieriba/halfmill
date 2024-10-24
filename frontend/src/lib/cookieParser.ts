type Cookie = {
	key: string;
	value: string;
	httpOnly: boolean;
	path: string;
	maxAge: number;
};

//Parse a string a cookie will return a tuple where the first element is the string parsed as a cookie,
//and a boolean which indicate if the cookie key/value was not found
export const parseCookie = (cookieToParse: string): [Cookie, boolean] => {
	const splittedCookie = cookieToParse.split(';');
	const parsedCookie: string[] = splittedCookie[0].split('=');
	const error = parsedCookie.length != 2;
	const map = new Map();
	const attributes = splittedCookie.slice(1);
	attributes.forEach((attribute) => {
		const [key, value] = attribute.trim().split('=');
		map.set(key, value);
	});
	return [
		{
			key: error ? '' : parsedCookie[0],
			value: error ? '' : parsedCookie[1],
			httpOnly: true,
			path: map.get('Path') ?? '/',
			maxAge: Number(map.get('Max-Age') ?? Number.MAX_VALUE)
		},
		error
	];
};

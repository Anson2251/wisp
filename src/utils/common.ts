export const encodeBase64 = (str: string) => {
	const uint8Array = new TextEncoder().encode(str);
	return btoa(String.fromCharCode(...uint8Array));
};

export const decodeBase64 = (str: string) => {
	const charCodes = atob(str).split('').map(c => c.charCodeAt(0));
	return new TextDecoder().decode(new Uint8Array(charCodes));
};

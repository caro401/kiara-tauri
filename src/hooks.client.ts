export function handleError({ error }: { error: string }) {
	// Write good error messages in rust, pass them through to the frontend
	return {
		message: error,
		code: 'Something went wrong'
	};
}

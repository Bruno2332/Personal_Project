import { Button } from "./Button"


export function ErrorMessage({ message }: { message: string }) {
	return message ? (
		<div
			role="alert"
			className="px-3 py-2 text-sm text-red-400 bg-red-400/10 border border-red-400/20 rounded"
		>
			{message}
		</div>
	) : null
}
export function LoadingMessage() {
	return (
		<p role="status" className="text-sm text-[#717171]">
			Carregando...
		</p>
	)
}
export function RetryMessage({
	message,
	retry,
}: {
	message: string
	retry: () => void
}) {
	return (
		<div className="space-y-2">
			<ErrorMessage message={message} />
			<Button
				onClick={retry}
				className="px-4 py-2 text-sm text-[#717171] hover:text-white border border-[#252525] rounded transition-colors"
			>
				Tentar novamente
			</Button>
		</div>
	)
}

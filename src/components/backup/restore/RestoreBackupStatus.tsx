import { Button } from "../../ui/Button"

type Props = {
	busy: boolean
	restored: boolean
	restartRequired: boolean
	error: string
	onRestart: () => void
}


export default function RestoreBackupStatus({
	busy,
	restored,
	restartRequired,
	error,
	onRestart,
}: Props) {
	return (
		<div
			role="dialog"
			aria-modal="true"
			aria-label="Restauração do backup"
			aria-busy={busy}
			className="fixed inset-0 z-[100] bg-[#0d0d0d] flex items-center justify-center p-4"
		>
			<div className="w-full max-w-md bg-[#111] border border-[#252525] rounded-lg p-6 space-y-4">
				<h2 className="font-display text-2xl font-700 uppercase text-white">
					{restartRequired
						? restored
							? "Backup restaurado"
							: "Reinício necessário"
						: "Restaurando backup"}
				</h2>

				<p role="status" className="text-sm text-[#717171]">
					{restartRequired
						? busy
							? "Reiniciando aplicativo..."
							: "Reinicie o aplicativo para abrir o banco de dados."
						: "Aguarde a validação e a restauração dos dados."}
				</p>

				{error && (
					<p role="alert" className="text-sm text-red-400">
						{error}
					</p>
				)}

				{restartRequired && (
					<Button
						type="button"
						disabled={busy}
						onClick={onRestart}
						className="px-4 py-2 text-sm bg-[#a3e635] text-black font-semibold rounded hover:bg-[#bef264] disabled:opacity-50"
					>
						{busy ? "Reiniciando..." : "Reiniciar aplicativo"}
					</Button>
				)}
			</div>
		</div>
	)
}
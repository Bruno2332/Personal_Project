import BackupSettings from "../components/backup/settings/BackupSettings"
import RestoreBackup from "../components/backup/restore/RestoreBackup"


export default function Backup() {
	return (
		<div className="flex flex-col h-full gap-6">
			<div>
				<h1 className="font-display text-4xl font-800 tracking-wide text-white uppercase">
					Backup
				</h1>

				<p className="text-[#717171] text-sm mt-0.5">
					Configure os backups automáticos, gere uma cópia
					manual ou restaure seus dados.
				</p>
			</div>

			<div className="grid grid-cols-1 lg:grid-cols-2 gap-6 items-start">
				<section
					aria-label="Configuração e criação de backups"
					className="bg-[#161616] border border-[#252525] rounded p-5"
				>
					<BackupSettings />
				</section>

				<section
					aria-label="Restauração de backup"
					className="bg-[#161616] border border-[#252525] rounded p-5"
				>
					<RestoreBackup />
				</section>
			</div>
		</div>
	)
}
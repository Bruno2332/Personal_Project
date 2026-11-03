import { FormProvider } from "react-hook-form"

import { Button } from "../../ui/Button"
import { FormInput } from "../../ui/Fields"
import useBackupSettings from "../hooks/useBackupSettings"


export default function BackupSettings() {
	const form = useBackupSettings()

	return (
		<div className="space-y-4">
			<div>
				<h2 className="text-sm text-white font-medium">
					Backup dos dados
				</h2>

				<p className="text-xs text-[#717171] mt-1">
					Automático a cada 7 dias, na abertura do aplicativo.
					São mantidos os 5 backups mais recentes.
				</p>
			</div>

			{form.loading ? (
				<p role="status" className="text-sm text-[#717171]">
					Carregando configuração...
				</p>
			) : form.loadFailed ? (
				<Button
					type="button"
					onClick={form.retry}
					className="text-sm text-[#a3e635]"
				>
					Tentar novamente
				</Button>
			) : (
				<>
					<FormProvider {...form.methods}>
						<form
							noValidate
							onSubmit={form.methods.handleSubmit(form.save)}
						>
							<fieldset
								disabled={form.busy}
								className="space-y-4 disabled:opacity-60"
							>
								<FormInput
									name="folder"
									label="Pasta dos backups"
									readOnly
									required
									placeholder="Selecione uma pasta"
								/>

								<Button
									type="button"
									onClick={() => void form.selectFolder()}
									className="px-4 py-2 text-sm text-[#a3e635] border border-[#252525] rounded hover:bg-[#1e1e1e]"
								>
									Selecionar pasta do Google Drive
								</Button>

								<FormInput
									name="password"
									label={
										form.settings
											? "Nova senha de recuperação"
											: "Senha de recuperação"
									}
									type="password"
									nullable
									required={!form.settings}
									placeholder={
										form.settings
											? "Deixe vazio para manter a atual"
											: "Pelo menos 12 caracteres"
									}
								/>

								<p className="text-xs text-[#717171]">
									Guarde essa senha fora do computador.
									Ela será necessária para restaurar o
									backup em outro equipamento.
								</p>

								{form.settings && (
									<p className="text-xs text-[#717171]">
										Alterar a senha afeta somente os
										novos backups. Os anteriores
										continuam usando a senha antiga.
									</p>
								)}

								<Button
									type="submit"
									className="px-4 py-2 text-sm text-[#a3e635] border border-[#252525] rounded hover:bg-[#1e1e1e]"
								>
									Salvar configuração
								</Button>
							</fieldset>
						</form>
					</FormProvider>

					{form.settings && (
						<div className="space-y-3">
							<p className="text-xs text-[#717171]">
								Último automático:{" "}
								{form.settings.last_automatic_backup
									? new Date(
										form.settings.last_automatic_backup,
									).toLocaleString("pt-BR")
									: "Ainda não realizado"}
							</p>

							<Button
								type="button"
								disabled={form.busy || form.unsavedChanges}
								onClick={() => void form.backupNow()}
								className="px-4 py-2 text-sm bg-[#a3e635] text-black font-semibold rounded hover:bg-[#bef264] disabled:opacity-50"
							>
								{form.busy ? "Aguarde..." : "Fazer backup agora"}
							</Button>

							{form.unsavedChanges && (
								<p className="text-xs text-[#717171]">
									Salve a configuração antes de gerar o backup.
								</p>
							)}
						</div>
					)}
				</>
			)}

			{form.message && (
				<p
					role="status"
					className="text-sm text-[#a3e635] break-all"
				>
					{form.message}
				</p>
			)}

			{form.error && (
				<p role="alert" className="text-sm text-red-400">
					{form.error}
				</p>
			)}
		</div>
	)
}
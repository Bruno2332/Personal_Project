import { FormProvider } from "react-hook-form"

import { Button } from "../../ui/Button"
import { FormInput } from "../../ui/Fields"

import useRestoreBackup from "../hooks/useRestoreBackup"
import RestoreBackupStatus from "./RestoreBackupStatus"


export default function RestoreBackup() {
	const form = useRestoreBackup()

	if (form.busy || form.restartRequired) {
		return (
			<RestoreBackupStatus
				busy={form.busy}
				restored={form.restored}
				restartRequired={form.restartRequired}
				error={form.error}
				onRestart={() => void form.restart()}
			/>
		)
	}

	return (
		<div className="space-y-4">
			<div>
				<h2 className="text-sm text-white font-medium">
					Restaurar backup
				</h2>

				<p className="text-xs text-[#717171] mt-1">
					Selecione um backup e informe a senha usada na
					criação desse arquivo.
				</p>
			</div>

			<Button
				type="button"
				disabled={form.selecting}
				onClick={() => void form.selectBackup()}
				className="px-4 py-2 text-sm text-[#a3e635] border border-[#252525] rounded hover:bg-[#1e1e1e] disabled:opacity-50"
			>
				Selecionar arquivo
			</Button>

			{form.path && (
				<FormProvider {...form.methods}>
					<form onSubmit={form.methods.handleSubmit(form.restore)}>
						<fieldset
							disabled={form.selecting}
							className="space-y-4 disabled:opacity-60"
						>
							<p className="text-xs text-[#717171] break-all">
								{form.path}
							</p>

							<FormInput
								name="password"
								label="Senha de recuperação"
								type="password"
								required
							/>

							<label className="flex items-start gap-3 text-sm text-[#d0d0d0]">
								<input
									key={form.path}
									type="checkbox"
									required
									className="mt-1 accent-[#a3e635]"
								/>

								<span>
									Confirmo que todos os dados atuais serão
									substituídos pelos dados deste backup.
								</span>
							</label>

							<Button
								type="submit"
								disabled={form.methods.formState.isSubmitting}
								className="px-4 py-2 text-sm bg-[#a3e635] text-black font-semibold rounded hover:bg-[#bef264] disabled:opacity-50"
							>
								Restaurar dados
							</Button>
						</fieldset>
					</form>
				</FormProvider>
			)}

			{form.restored && (
				<p role="status" className="text-sm text-[#a3e635]">
					Backup restaurado.
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
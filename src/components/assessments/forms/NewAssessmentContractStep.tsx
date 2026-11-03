import { FormProvider } from "react-hook-form"
import type { UseFormReturn } from "react-hook-form"

import type { FormValues } from "../hooks/useNewAssessmentContract"

import ContractFields from "../../contracts/forms/ContractFields"
import { FormSelect } from "../../ui/Fields"
import { Modal } from "../../ui/Modal"
import { Button } from "../../ui/Button"
import { ErrorMessage } from "../../ui/Feedback"

type Props = {
  methods: UseFormReturn<FormValues>
  studentOptions: { value: number; label: string }[]
  studentsLoading: boolean
  studentsError: string
  onRetry: () => void
  onClose: () => void
  onNext: (values: FormValues) => void
}

export default function NewAssessmentContractStep({
  methods,
  studentOptions,
  studentsLoading,
  studentsError,
  onRetry,
  onClose,
  onNext,
}: Props) {
  return (
    <Modal label="Nova avaliação — aluno e contrato" onClose={onClose}>
      <div className="relative bg-[#111] border border-[#252525] rounded-lg w-full max-w-2xl max-h-[90vh] overflow-y-auto shadow-2xl mx-2">
        <div className="sticky top-0 bg-[#111] border-b border-[#252525] px-6 py-4 flex items-center justify-between z-10">
          <div>
            <h2 className="font-display text-2xl font-700 uppercase tracking-wide text-white">
              Nova Avaliação
            </h2>

            <p className="text-xs text-[#717171] mt-1">
              1. Aluno e contrato · 2. Dados da avaliação
            </p>
          </div>

          <Button
            onClick={onClose}
            aria-label="Fechar"
            className="text-[#717171] hover:text-white text-xl"
          >
            ×
          </Button>
        </div>

        <FormProvider {...methods}>
          <form noValidate onSubmit={methods.handleSubmit(onNext)}>
            <div className="p-6 space-y-5">
              <FormSelect
                name="student_id"
                label="Aluno"
                numeric
                required
                options={studentOptions}
              />

              {studentsLoading && (
                <p className="text-xs text-[#717171]">Carregando alunos...</p>
              )}

              {studentsError && (
                <div className="space-y-2">
                  <ErrorMessage message={studentsError} />

                  <Button
                    disabled={studentsLoading}
                    onClick={onRetry}
                    className="text-sm text-[#a3e635]"
                  >
                    Tentar novamente
                  </Button>
                </div>
              )}

              <h3 className="text-base text-[#717171] uppercase tracking-wider font-medium">
                Dados da contratação
              </h3>

              <ContractFields assessmentOnly />
            </div>

            <div className="sticky bottom-0 bg-[#111] border-t border-[#252525] px-6 py-4 flex justify-end gap-3">
              <Button
                onClick={onClose}
                className="px-4 py-2 text-sm text-[#717171] hover:text-white border border-[#252525] rounded"
              >
                Cancelar
              </Button>

              <Button
                type="submit"
                disabled={
                  studentsLoading ||
                  !!studentsError ||
                  methods.formState.isSubmitting
                }
                className="px-5 py-2 text-sm bg-[#a3e635] text-black font-semibold rounded hover:bg-[#bef264] transition-colors"
              >
                Continuar para avaliação
              </Button>
            </div>
          </form>
        </FormProvider>
      </div>
    </Modal>
  )
}

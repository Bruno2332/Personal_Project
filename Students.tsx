import { useEffect, useState } from "react";
import { findAllStudents } from "../service/student-service";

type StudentListItem =
  Awaited<ReturnType<typeof findAllStudents>>["items"][number];

const objectiveLabels: Record<string, string> = {
  HypertrophyAndDefinition: "Hipertrofia e definição muscular",
  WeightLoss: "Emagrecimento e condicionamento físico",
  RehabilitationStrengthening: "Reabilitação e fortalecimento",
  SportsPerformance: "Performance esportiva",
  GeneralConditioningHealth: "Condicionamento geral e saúde",
  FlexibilityMobility: "Flexibilidade e mobilidade",
};

const billingLabels: Record<string, string> = {
  Monthly: "Mensal",
  Package: "Pacote",
};

function formatMoney(cents: number) {
  return (cents / 100).toLocaleString("pt-BR", {
    style: "currency",
    currency: "BRL",
  });
}

function formatDate(value: string | null) {
  if (!value) return "—";

  const [year, month, day] = value.split("-");
  return `${day}/${month}/${year}`;
}

function errorMessage(error: unknown): string {
  if (typeof error === "string") return error;

  if (
    typeof error === "object" &&
    error !== null &&
    "message" in error &&
    typeof error.message === "string"
  ) {
    return error.message;
  }

  return "Não foi possível carregar os alunos.";
}

export default function Students() {
  const [students, setStudents] = useState<StudentListItem[]>([]);
  const [page, setPage] = useState(0);
  const [total, setTotal] = useState(0);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [reload, setReload] = useState(0);

  const size = 20;

  useEffect(() => {
    let cancelled = false;

    async function loadStudents() {
      setLoading(true);
      setError(null);

      try {
        const response = await findAllStudents(page, size);

        if (!cancelled) {
          setStudents(response.items);
          setTotal(response.total);
        }
      } catch (error: unknown) {
        if (!cancelled) {
          setError(errorMessage(error));
        }
      } finally {
        if (!cancelled) {
          setLoading(false);
        }
      }
    }

    void loadStudents();

    return () => {
      cancelled = true;
    };
  }, [page, reload]);

  const totalPages = Math.max(1, Math.ceil(total / size));

  const buttonClass =
    "px-4 py-2 border border-[#252525] rounded text-sm " +
    "text-white hover:bg-[#252525] disabled:opacity-40 " +
    "disabled:cursor-not-allowed";

  return (
    <div className="flex flex-col h-full gap-6">
      <div className="flex items-center justify-between">
        <div>
          <h1 className="font-display text-4xl font-800 tracking-wide text-white uppercase">
            Alunos
          </h1>

          <p className="text-[#717171] text-sm mt-0.5">
            {loading ? "Carregando..." : `${total} alunos encontrados`}
          </p>
        </div>

        <button
          type="button"
          className={buttonClass}
          disabled={loading}
          onClick={() => setReload(current => current + 1)}
        >
          Atualizar
        </button>
      </div>

      {error ? (
        <div
          role="alert"
          className="p-4 text-red-400 bg-red-400/10 border border-red-400/20 rounded"
        >
          <p>{error}</p>

          <button
            type="button"
            className={`${buttonClass} mt-3`}
            disabled={loading}
            onClick={() => setReload(current => current + 1)}
          >
            Tentar novamente
          </button>
        </div>
      ) : (
        <div className="flex-1 bg-[#161616] border border-[#252525] rounded overflow-x-auto">
          <table className="w-full min-w-[700px]">
            <thead>
              <tr className="border-b border-[#252525]">
                {[
                  "Aluno",
                  "Contato",
                  "Objetivo",
                  "Primeiro vencimento",
                  "Contrato",
                  "Status",
                ].map(title => (
                  <th
                    key={title}
                    className="text-left px-4 py-3 text-xs font-medium text-[#717171] uppercase tracking-wider"
                  >
                    {title}
                  </th>
                ))}
              </tr>
            </thead>

            <tbody>
              {loading ? (
                <tr>
                  <td
                    colSpan={6}
                    className="px-4 py-12 text-center text-[#717171]"
                  >
                    Carregando alunos...
                  </td>
                </tr>
              ) : students.length === 0 ? (
                <tr>
                  <td
                    colSpan={6}
                    className="px-4 py-12 text-center text-[#717171]"
                  >
                    Nenhum aluno encontrado.
                  </td>
                </tr>
              ) : (
                students.map(student => (
                  <tr
                    key={student.id}
                    className="border-b border-[#252525]/60 last:border-b-0 hover:bg-[#1e1e1e]"
                  >
                    <td className="px-4 py-3">
                      <div className="flex items-center gap-3">
                        <div className="w-8 h-8 shrink-0 rounded-full bg-[#a3e635]/20 flex items-center justify-center text-[#a3e635] text-xs font-bold font-display uppercase">
                          {student.name
                            .trim()
                            .split(/\s+/)
                            .slice(0, 2)
                            .map(part => part[0])
                            .join("")}
                        </div>

                        <span className="text-white font-medium text-sm">
                          {student.name}
                        </span>
                      </div>
                    </td>

                    <td className="px-4 py-3">
                      <div className="text-sm text-[#d0d0d0]">
                        {student.email}
                      </div>
                      <div className="text-xs text-[#717171]">
                        {student.phone ?? "—"}
                      </div>
                    </td>

                    <td className="px-4 py-3 text-sm text-[#d0d0d0] max-w-[180px]">
                      <span className="line-clamp-1">
                        {student.objective
                          ? objectiveLabels[student.objective] ?? student.objective
                          : "—"}
                      </span>
                    </td>

                    <td className="px-4 py-3 text-sm text-[#717171] font-mono">
                      {formatDate(student.first_due_date)}
                    </td>

                    <td className="px-4 py-3">
                      <div className="text-sm text-white font-mono font-medium">
                        {formatMoney(student.amount_cents)}
                        {student.billing_type === "Monthly" ? "/mês" : ""}
                      </div>
                      <div className="text-xs text-[#717171]">
                        {billingLabels[student.billing_type] ??
                          student.billing_type}
                      </div>
                    </td>

                    <td className="px-4 py-3">
                      <span
                        className={`text-xs font-medium px-2 py-0.5 rounded-full ${
                          student.is_active
                            ? "text-[#a3e635] bg-[#a3e635]/10"
                            : "text-[#717171] bg-[#717171]/10"
                        }`}
                      >
                        {student.is_active ? "Ativo" : "Inativo"}
                      </span>
                    </td>
                  </tr>
                ))
              )}
            </tbody>
          </table>
        </div>
      )}

      <div className="flex items-center justify-between gap-3">
        <span className="text-sm text-[#717171]">
          Página {page + 1} de {totalPages}
        </span>

        <div className="flex gap-2">
          <button
            type="button"
            className={buttonClass}
            disabled={loading || page === 0}
            onClick={() => setPage(current => current - 1)}
          >
            Anterior
          </button>

          <button
            type="button"
            className={buttonClass}
            disabled={loading || error !== null || (page + 1) * size >= total}
            onClick={() => setPage(current => current + 1)}
          >
            Próxima
          </button>
        </div>
      </div>
    </div>
  );
}

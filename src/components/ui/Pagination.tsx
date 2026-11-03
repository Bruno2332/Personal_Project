import { Button } from "./Button"
export function Pagination({
  page,
  total,
  size,
  onPage,
}: {
  page: number
  total: number
  size: number
  onPage: (page: number) => void
}) {
  const pages = Math.max(1, Math.ceil(total / size))
  if (pages === 1) return null
  return (
    <div className="flex items-center justify-end gap-3 text-xs text-[#717171]">
      <Button
        disabled={page === 0}
        onClick={() => onPage(page - 1)}
        className="px-3 py-1.5 rounded border border-[#252525] disabled:opacity-40"
      >
        Anterior
      </Button>
      <span>
        Página {page + 1} de {pages}
      </span>
      <Button
        disabled={page + 1 >= pages}
        onClick={() => onPage(page + 1)}
        className="px-3 py-1.5 rounded border border-[#252525] disabled:opacity-40"
      >
        Próxima
      </Button>
    </div>
  )
}

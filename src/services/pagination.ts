import type { PageResponse } from "../model/pagination"

// Resumos globais e seletores precisam do conjunto completo, não de uma única página.
export async function collectPages<T>(
  load: (page: number, size: number) => Promise<PageResponse<T>>,
): Promise<T[]> {
  const result: T[] = []
  const size = 100
  for (let page = 0; ; page++) {
    const response = await load(page, size)
    result.push(...response.items)
    if (!response.items.length || (page + 1) * size >= response.total)
      return result
  }
}

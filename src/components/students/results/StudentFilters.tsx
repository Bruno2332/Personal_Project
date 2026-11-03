import { SearchInput } from "../../ui/SearchInput"

type Props = {
	search: string
	onSearch: (value: string) => void
}

export default function StudentFilters({ search, onSearch }: Props) {
	return (
		<SearchInput
			value={search}
			onChange={(event) => onSearch(event.target.value)}
			placeholder="Buscar aluno por nome..."
		/>
	)
}

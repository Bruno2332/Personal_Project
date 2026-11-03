type ViaCepResponse = {
    erro?: boolean | string
    logradouro?: string
    localidade?: string
    uf?: string
}

export async function findAddressByCep(
    cep: string,
    signal?: AbortSignal,
): Promise<ViaCepResponse> {
    const response = await fetch(
        `https://viacep.com.br/ws/${cep}/json/`,
        { signal },
    )

    if (!response.ok) {
        throw new Error("Não foi possível consultar o CEP.")
    }

    const data: ViaCepResponse = await response.json()

    if (data.erro === true || data.erro === "true") {
        throw new Error("CEP não encontrado.")
    }

    return data
}
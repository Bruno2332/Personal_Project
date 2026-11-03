export async function searchCep(cep: any) {

    const cleanCEP = cep.replace(/\D/g, '');
    
    if (cleanCEP.length !== 8) return;

    try {
        const response = await fetch(`https://viacep.com.br/ws/${cleanCEP}/json/`);
        return await response.json();
        
    } catch (error) {
        console.error("Erro ao buscar CEP:", error);
    }
}
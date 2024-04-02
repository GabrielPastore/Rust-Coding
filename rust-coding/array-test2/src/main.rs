// Definindo a função que conta as repetições de um input dentro do array
fn search_pairs (array: [i8; 5], target: i8) -> i8 {
    
    // Essa variável vai ser adicionada um valor de 1 pra cada repetição do número que foi buscado
    // dentro do array
    let mut target_repetitions: i8 = 0;
    
    // Loop for pra poder iterar sobre cada instância de objeto dentro do array
    for value in array.iter() {
        
        // Caso o número (desreferenciado) seja igual ao número que se quer buscar, é adicionado 1
        // à variável que conta repetições
        if *value == target {
            target_repetitions += 1;
        }
    }
    // Caso o número esteja na array, ele é dividido por 2 para retornar o número de pares
    if target_repetitions > 0 {
        target_repetitions = target_repetitions / 2;
    }
    return target_repetitions;
}
// Sintaxe pra estrutura de teste
#[cfg(test)]
mod find_pairs_in_array {
    
    // Importando a função definida acima
    use crate::*;
    #[test]
    fn pairs_in_array () {
        
        // Um array de referência é criado, juntamente à uma variável de um número inteiro
        let reference_array: [i8; 5] = [8, 5, 8, 3, 3];
        let search_attempt: i8 = 8;
        
        // Aqui é comparado o número esperado de repetições da variável no array (2), com o número
        // real de repetições da variável
        assert_eq!( 1, search_pairs(reference_array, search_attempt));
    }
}

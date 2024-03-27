// Definindo a função que procura um número dentro do Array
fn search_in_array(array: [i8; 5], target: i8) -> bool {

    // Loop for que vai iterar sobre o array, para buscar o valor desejado
    for value in array.iter() {
        // O uso do * se dá para tirar a referência de value, do tipo '&i8' 
        // *value é do tipo 'i8', permitindo a comparação com target, também do tipo 'i8'
        if *value == target {
            return true
        }
    // Caso o valor não seja encontrado, é retornado falso
    } return false
}


// Sintaxe pra estrutura de teste
#[cfg(test)]
mod find_number_in_array {

    //importando a função auxiliar definida acima
    use crate::*;
    #[test]
    fn is_in_array () {

        // Um array de referência é criado, tal como um inteiro a ser buscado
        let reference_array: [i8; 5] = [83, -54, 127, 36, -79];
        let search_attempt: i8 = 36;
        
        // Aqui é comparado o valor esperado da operação de buscar o inteiro no array (true), com o
        // resultado que de fato ocorre na busca
        assert_eq!(true, search_in_array(reference_array, search_attempt));
    }
}

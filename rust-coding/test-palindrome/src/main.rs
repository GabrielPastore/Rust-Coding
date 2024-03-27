// Importando a biblioteca Regex pra tratar a String
use regex::Regex;

// Definindo a função auxiliar que vai detectar palíndromos
fn compare_strings (input_string: &str) -> bool {
    
    // Criando um objeto Regex, uma expressão regular para eliminar todos os caracteres inválidos
    // de uma string
    let clean = Regex::new(r"[^A-Za-z]").unwrap();
    let clean_input_string = clean.replace_all(input_string, "").to_lowercase();
    
    // Aqui criamos um Vetor de caracteres para poder inverter a string de input
    let mut chars_vec: Vec<char> = Vec::new();
    
    // Neste loop for, invertemos a string de input tratada e adicionamos cada caractere nesse
    // Vetor vazio
    for char in clean_input_string.chars().rev() {
        chars_vec.push(char);
    }

    // Uma variável é criada, que assume como valor o a String criada a partir do Vetor com os
    // caracteres que foram adicionados no loop for acima
    let reversed_str: String = chars_vec.into_iter().collect();
    
    // Caso a string de input (tratada) seja igual a sua versão invertida (caso de palíndromo), é
    // retornado true, caso contrário, false
    if clean_input_string == reversed_str {
        return true
    } else {
        return false
    }
}

// Sintaxe para poder realizar o teste
#[cfg(test)]
mod check_palindrome{

    // Importando a função auxiliar acima
    use crate::*;
    
    // Função a ser testada
    #[test]
    fn is_palindrome() {
        
        // Aqui é declarada a string a ser analisada, apenas pra testar se a função funciona, mas
        // poderia ser um input do usuário também
        let my_string: &str = "Arara";

        // Aqui a função é testada, para ver se o valor retornado é true (palíndromo)
        assert_eq!(true, compare_strings(my_string));
    }
}

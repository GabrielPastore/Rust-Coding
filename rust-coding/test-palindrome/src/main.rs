// Importando a biblioteca Regex pra tratar a String
use regex::Regex;

// Definindo a função auxiliar que vai detectar palíndromos
fn compare_strings (input_string: &str) -> bool {
    
    // Criando um objeto Regex, uma expressão regular para eliminar todos os caracteres inválidos
    // de uma string
    let clean = Regex::new(r"[^A-Za-z]").unwrap();
    let clean_input_string = clean.replace_all(input_string, "").to_lowercase();
    
    // Comparando a string de input com sua versão invertida
    if clean_input_string == clean_input_string.chars().rev().collect::<String>() {
        
        // Retornando verdadeiro caso a string seja igual ao seu inverso
        return true
    } else {

        // Retornando falso caso não seja igual
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

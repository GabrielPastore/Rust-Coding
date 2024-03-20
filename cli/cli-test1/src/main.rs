use serde::{Serialize, Deserialize};
use std::fs;

#[derive(Serialize, Deserialize, Debug)]
struct Tarefa {
    nome_da_tarefa: String,
    terminou: bool,
}

//Criando a função de adicionar uma tarefa
fn addtask() -> Tarefa {
    // O segundo parâmetro enviado no terminal fica salvo como o nome da tarefa
    let taskname = std::env::args().nth(2).expect("Nome não passado");
    // Uma instância da estrutura tarefa é criada, com os valores de nome e se foi concluída
    let task = Tarefa {
        nome_da_tarefa: taskname,
        terminou: false,
    };
    // Essa instância é retornada pra ser adicionada no vetor que guarda as diferentes instâncias
    // da estrutura
    return task;
    }

// Criando um método à Structure "Tarefa", para marcar as tarefas como concluídas
impl Tarefa {
    fn done(&mut self, nome_comparar: &str) {
        // Caso o segundo parâmetro (nome da tarefa) for encontrado em uma instãncia específica da
        // Structure dentro do vetor, ela é marcada como concluída
        if self.nome_da_tarefa == nome_comparar{
            self.terminou = true;
            println!("A tarefa '{}' foi concluída.", nome_comparar);
        }
    }
}

// Na função main, é onde definimos o vetor da Structure "Tarefas". O programa vai tentar fazer o
// vetor a partir de "tarefas.json". Caso esse arquivo não exista, uma vetor novo é criado do 0
fn main() {

    let mut tasklist : Vec<Tarefa> = match fs:: read_to_string("tarefas.json") {
        Ok(conteudo_json) => serde_json::from_str(&conteudo_json).unwrap_or_default(),
        Err(_) => Vec::new(),
    };
    
    // Aqui é pego o primeiro parâmetro enviado, que nesse caso vai ser o comando a ser executado
    // (add, list, done, update, delete)
    let command = &std::env::args().nth(1).expect("Comando não passado");
    
    // Se for add ele chama a função de adicionar uma instância da Structure
    if command == "add"{    
        tasklist.push(addtask());
    
    // Se for list ele vai rodar um for loop que dê print em cada instância da Structure
    } else if command == "list"{

        for tarefa in &tasklist {
            println!("{:#?}", tarefa);
        }
    
    // Se for done, ele vai pegar o segundo parãmetro (nome da tarefa) e verificar toda as
    // instâncias de Structure até achar alguma que tenha o mesmo nome, caso ache, ele usa o método
    // done criado em cima
    } else if command == "done"{

        let taskname = std::env::args().nth(2).expect("Nome não passado");
        for tarefa in &mut tasklist {
            tarefa.done(&taskname);
        }

    // Se for del, ele vai pegar o segundo parâmetro (nome da tarefa) e procurar a primeira tarefa
    // com esse nome e remover ela da lista (vetor de Structures "Tarefa"
    } else if command == "del"{

        let taskname = std::env::args().nth(2).expect("Nome não passado");
        for (index, tarefa) in tasklist.iter().enumerate() {
            if tarefa.nome_da_tarefa == taskname {
                tasklist.remove(index);
                println!("A tarefa {} foi removida da lista.", taskname);
                break;
            }
        }
    }
    
    
    // Aqui ele vai transformar o vetor em um .JSON chamado "tarefas.JSON" e salvar na pasta src.
    // Caso esse arquivo já exista, ele vai sobreescrever, passando toda as adições, exclusões e
    // alterações para o JSON. Esse arquivo é chamado ao inicializar o programa pra gerar o vetor
    // na seção do programa que tu for rodar, assim, salvando a lista de tarefas entre usos do
    // comando 
    let conteudo_json = serde_json::to_string(&tasklist).expect("Erro ao serializar vetor para JSON");
    fs::write("tarefas.json", conteudo_json).expect("Erro ao escrever no arquivo.");

}

// Aqui importamos os módulos a serem usados
use serde::{Serialize, Deserialize};
use std::fs;
use clap::Parser; 


// Aqui definimos a Structure CLI que vai servir para pegar os parâmetros usando o Parser
#[derive(Parser)]
struct  Cli {
    command: String,
    taskname: String,
}

// Aqui definimos a Structure Tarefa que vai ser a forma de guardar as tarefas, que serão
// armazenadas em um Vetor.
#[derive(Serialize, Deserialize, Debug)]
struct Tarefa {
    nome_da_tarefa: String,
    terminou: bool,
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
    
    let args = Cli::parse();

    // Se o primeiro parâmetro (comando) for add, ele vai pegar o segundo parâmetro (nome da tarefa) e criar uma
    // instância da Structure e a adiciona ao vetor de Structures "Tarefa" com os valores de nome e terminou
    if args.command == "add"{
        let task = Tarefa {
            nome_da_tarefa: args.taskname,
            terminou: false,
        };
        tasklist.push(task);
    
    // Se os dois parâmetros forem list all ele vai rodar um for loop que dê print em cada instância da Structure
    } else if args.command == "list"{
        if args.taskname == "all"{
            for tarefa in &tasklist {
                println!("{:#?}", tarefa);
            } 
        } else {
                
                // Adicionando um indicador para ajudar o usuário a acertar o comando list all
                println!("\nComando não entendido, você quis dizer 'task -- list all'?\n");
            }
    
    // Se o primeiro parâmetro (comando) for done, ele vai pegar o segundo parãmetro (nome da tarefa) e
    // verificar toda as instâncias de Structure até achar alguma que tenha o mesmo nome, caso ache,
    // ele usa o método done criado em cima
    } else if args.command == "done"{

        for tarefa in &mut tasklist {
            tarefa.done(&args.taskname);
        }

    // Se o primeiro parâmetro (comando) for del, ele vai pegar o segundo parâmetro (nome da tarefa) e procurar
    // a primeira tarefa com esse nome e remover ela da lista (vetor de Structures "Tarefa"
    } else if args.command == "del"{

        for (index, tarefa) in tasklist.iter().enumerate() {
            if tarefa.nome_da_tarefa == args.taskname {
                tasklist.remove(index);
                println!("A tarefa {} foi removida da lista.", args.taskname);
                break;
            }
        }
    } else if args.command == "help" {
       
        // Adicionando um comando de help para poder indicar como funciona a CLI 
        if args.taskname == "all"{
            println!("\ntask -- add (nome da tarefa) -> adiciona uma tarefa à lista");
            println!("task -- done (nome da tarefa) -> define uma tarefa da lista como concluída");
            println!("task -- del (nome da tarefa) -> deleta uma tarefa da lista");
            println!("task -- list all -> lista todas as tarefas salvas, terminadas ou não\n");
        } else {

            println!("\nComando não entendido, você quis dizer 'task -- help all'?\n");
        }
    } else {
        //Adicionando uma mensagem de erro
        println!("\nParâmetro incorreto! Tente usar 'tasks -- help all'\n");
        }

    
    
    // Aqui ele vai transformar o vetor em um .JSON chamado "tarefas.JSON" e salvar na pasta src.
    // Caso esse arquivo já exista, ele vai sobreescrever, passando toda as adições, exclusões e
    // alterações para o JSON. Esse arquivo é chamado ao inicializar o programa pra gerar o vetor
    // na seção do programa que tu for rodar, assim, salvando a lista de tarefas entre usos do
    // comando 
    let conteudo_json = serde_json::to_string(&tasklist).expect("Erro ao serializar vetor para JSON");
    fs::write("tarefas.json", conteudo_json).expect("Erro ao escrever no arquivo.");

}

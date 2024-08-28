pub fn intro() {
    println!("+---------------------+");
    println!("|Bem vindo ao Process.|");
    println!("+---------------------+");
    println!("Como funciona:");
    println!("1. O programa pedirá para informar o tempo de cpu");
    println!(
        "2. Após, você deve informar se tem tempo de espera 
                (i/o), caso sim, informe o número, caso não, informe 0."
    );
    println!("3. O tempo que o processo deve ser criado em unidade tempo.");
    println!("4. Prioridade do processo.");
    println!("5. Se tem mais processos. Se não, o programa iniciará.");
    println!(
        "6. Ao final, o programa mostrará a fila de espera com tempo de cpu restantes, 
                tempo de retorno da filad e espera e a tabela de tempo de criação, tempo de 
                serviço, tempo de excecução e a média dos processos."
    );
}

use std::collections::VecDeque;

use crate::Process;
pub fn process_actual(exec: &Process, unit: &i32, fesp: &VecDeque<Process>, ready: &VecDeque<Process>){
    print!("{:3} | {:8} | {:5} |", exec.cpu, exec.id, unit); // Exibe processo e tempo
                
    for pesp in fesp{
        print!(" {}, ", pesp.id); // Exibe processos em espera
    }

    print!("|");

    for pready in ready{
        print!(" {}, ", pready.id); // Exibe processos prontos
    }

    println!("");
}
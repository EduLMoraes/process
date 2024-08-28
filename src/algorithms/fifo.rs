use super::*;
pub fn fifo() {
    let mut processes = collect();
    let mut fesp: VecDeque<Process> = VecDeque::new();
    let mut ready: VecDeque<Process> = VecDeque::new();
    let mut exec: Process = Process::new();

    let mut time: Vec<i32> = Vec::new();
    let mut unit = 0;
    let twork = 1;
    println!("CPU | Processo | Tempo | F.E.S.P | Prontos");

    loop{
        

        for proc in &processes.clone(){
            if unit == proc.tc { // tempo de criação de um processo
                ready.push_back(processes.pop_front().unwrap()); // cria o processo e move para fila de prontos
                if unit > 0 {
                    process_actual(&exec, &unit, &fesp, &ready);
                }
                if !time.contains(&unit){
                    time.push(unit);
                }
            }
        }

        // não-preemptivo
        if exec.is_empty() && !ready.is_empty(){ // se não tiver executando
            exec = ready.pop_front().unwrap(); // pega o primeiro da fila dos prontos
            process_actual(&exec, &unit, &fesp, &ready);
        }
        
        

        if exec.io == unit && exec.io > 0{
            exec.tc = unit + exec.io; // calcula tempo de sair de espera

            fesp.push_back(exec.clone()); // manda o executado pra fila de espera
            
            if !ready.is_empty(){
                exec = ready.pop_front().unwrap(); // pega o próximo pronto
            }
            
            process_actual(&exec, &unit, &fesp, &ready);
            if !time.contains(&unit){
                time.push(unit);
            }
        }

        for proc in &fesp.clone(){
            if unit == proc.tc{ // tempo de criação de um processo
                ready.push_back(fesp.pop_front().unwrap()); // cria o processo e move para fila de prontos
                process_actual(&exec, &unit, &fesp, &ready);
                if !time.contains(&unit){
                    time.push(unit);
                }
            }
        }

        unit += 1;
        exec.cpu -= twork;

        if processes.is_empty() && exec.is_empty() && ready.is_empty() && fesp.is_empty(){
            if !time.contains(&unit){
                time.push(unit);
            }
            break;
        }
    }

    println!("O último processo terminou às {} unidade tempo", time.last().unwrap());

}

use super::*;

#[allow(unreachable_code)]
pub fn collect() -> VecDeque<Process> {
    let mut queue: VecDeque<Process> = VecDeque::new();

    let mut cont = 1;
    loop {
        
        println!("P{cont}");
        println!("Tempo de criação: ");
        let mut tc: String = String::new();
        stdin().read_line(&mut tc).unwrap();
        println!("CPU: ");
        let mut cpu: String = String::new();
        stdin().read_line(&mut cpu).unwrap();
        println!("I/O: ");
        let mut io: String = String::new();
        stdin().read_line(&mut io).unwrap();
        println!("Prioridade: ");
        let mut pri: String = String::new();
        stdin().read_line(&mut pri).unwrap();

        let proc: Process = Process {
            id: cont,
            cpu: cpu.trim().parse::<i32>().unwrap(),
            io: io.trim().parse::<i32>().unwrap(),
            tc: tc.trim().parse::<i32>().unwrap(),
            priority: pri.trim().parse::<i8>().unwrap(),
        };
        
        queue.push_back(proc);
        
        println!("Tem mais processos? \n[1] Sim.\n[0] Não.");
        let mut res: String = String::new();
        stdin().read_line(&mut res).unwrap();
        if res.trim().parse::<i8>() == Ok(0){
            break;
        }
        cont += 1;
    }

    queue
}

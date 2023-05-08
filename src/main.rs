fn main() {
    env_logger::init();
    // read the file at argv[1]
    let filename = std::env::args().nth(1).expect("no filename given");
    let contents = std::fs::read_to_string(filename).expect("could not read file");

    let mut meowlist: Vec<u32> = vec![];
    for line in contents.lines() {
        log::debug!("line: {}", line);
        let meow = match match line.split_whitespace().next() {
            Some(n) => n,
            None => continue,
        }
        .parse::<u32>()
        {
            Ok(n) => n,
            Err(_) => {
                if line.starts_with("//") {
                    continue;
                }
                match line.trim() {
                    "RET" => 0,
                    "MEOW" => 1,
                    "PUSH" => 2,
                    "POP" => 3,
                    "LOAD" => 4,
                    "SAVE" => 5,
                    "ADD" => 6,
                    "SUB" => 7,
                    "JMP" => 8,
                    "JE" => 9,
                    "//" => continue,
                    _ => panic!("unknown instruction: {}", line),
                }
            }
        };
        meowlist.push(meow);
    }

    let mut ip = 0;

    loop {
        let instruction = meowlist[ip];
        match instruction {
            0 => {
                // RET
                log::debug!("{} RET", ip);
                println!("\n");
                ip += 1
            }
            1 => {
                // MEOW
                let t = meowlist[meowlist.len() - 1];
                log::debug!("{} MEOW {}", ip, t);
                for _ in 0..t {
                    print!("meow ");
                }
                ip += 1
            }
            2 => {
                // PUSH
                let n = meowlist[ip + 1];
                log::debug!("{} PUSH {}", ip, n);
                meowlist.push(n);
                ip += 2
            }
            3 => {
                // POP
                let t = meowlist.pop().expect("stack underflow");
                log::debug!("{} POP {}", ip, t);
                ip += 1
            }
            4 => {
                // LOAD
                let n = meowlist[ip + 1];
                let en = meowlist[n as usize];
                log::debug!("{} LOAD {} {}", ip, n, en);
                meowlist.push(en);
                ip += 2
            }
            5 => {
                // SAVE
                let n = meowlist[ip + 1];
                let t = meowlist[meowlist.len() - 1];
                log::debug!("{} SAVE {} {}", ip, n, t);
                meowlist[n as usize] = t;
                ip += 2
            }
            6 => {
                // ADD
                let a = meowlist.pop().expect("stack underflow");
                let b = meowlist.pop().expect("stack underflow");
                log::debug!("{} ADD {} {}", ip, a, b);
                meowlist.push(a + b);
                ip += 1
            }
            7 => {
                // SUB
                let a = meowlist.pop().expect("stack underflow");
                let b = meowlist.pop().expect("stack underflow");
                log::debug!("{} SUB {} {}", ip, a, b);
                meowlist.push(b - a);
                ip += 1
            }
            8 => {
                // JMP
                let n = meowlist[ip + 1];
                log::debug!("{} JMP {}", ip, n);
                ip = n as usize;
            }
            9 => {
                // JE
                let n = meowlist[ip + 1];
                let t = meowlist[meowlist.len() - 1];
                log::debug!("{} JE {} {}", ip, n, t);
                if t == 0 {
                    ip = n as usize;
                } else {
                    ip += 2;
                }
            }
            _ => {
                // NOP
                ip += 1;
            }
        }
    }
}

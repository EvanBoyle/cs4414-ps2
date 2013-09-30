use std::{io, run, os, vec, option, libc, task};

fn main() {
    static CMD_PROMPT: &'static str = "gash > ";
	let mut hist : ~[~str]= ~[];
    
    loop {
    	let mut cwd : ~str = os::getcwd().to_str();
    	print(cwd + " ");
        print(CMD_PROMPT);
        let line = io::stdin().read_line();
        debug!(fmt!("line: %?", line));
        let mut argv: ~[~str] = line.split_iter(' ').filter(|&x| x != "")
                                 .transform(|x| x.to_owned()).collect();
        debug!(fmt!("argv %?", argv));
        
        
        if argv.len() > 0 {
		let cmd = argv.connect(" ");
            let program = argv.remove(0);
		let arglen = argv.len();
		hist.push(copy cmd);
		let mut bg = false;
		if(argv.len()>0){
			if(argv[argv.len()-1].eq(&~"&")){
				bg = true;
				argv.remove(arglen-1);
			}
		}
		let args  : ~[~str] = match(argv.len()){
			0=>  ~[~""],
			_ => copy argv


		};
		
		
		//let opts = run::ProcessOptions::new();
		let opts = run::ProcessOptions {
            out_fd: Some(std::libc::STDOUT_FILENO),
            .. run::ProcessOptions::new()
        }; 
	
            match program {
                ~"exit"     => {return; }
                		//must reflect change in directory to mut cwd ^^^
                ~"cd"	    => { os::change_dir(~PosixPath(argv.remove(0)));}
                ~"history"  => {

			 for hist.slice(1, hist.len()).iter().advance |s|{			println(*s);
			


			}



}
                _           => {
				//libc::funcs::posix88::unistd::fork();
				
				if(bg){
					println(~"bg");
					
				do task::spawn_sched(task::SingleThreaded) { run::process_status(program, args);}	
				
				}
				else {
					run::process_status(program, argv); 

				}
				//let mut proc = run::Process::new(program, argv, opts);				

}
            }
        }
    }
}

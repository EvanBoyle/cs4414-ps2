use std::{io, run, os};

fn main() {
    static CMD_PROMPT: &'static str = "gash > ";
    
    loop {
    	let mut cwd : ~str = os::getenv("PWD").unwrap();
    	print(cwd + " ");
        print(CMD_PROMPT);
        let line = io::stdin().read_line();
        debug!(fmt!("line: %?", line));
        let mut argv: ~[~str] = line.split_iter(' ').filter(|&x| x != "")
                                 .transform(|x| x.to_owned()).collect();
        debug!(fmt!("argv %?", argv));
        
        
        if argv.len() > 0 {
            let program = argv.remove(0);
            match program {
                ~"exit"     => {return; }
                		//must reflect change in directory to mut cwd ^^^
                ~"cd"	    => {os::change_dir(~PosixPath(argv.remove(0)));}
                ~"history"  => {return; }
                _           => {run::process_status(program, argv);}
            }
        }
    }
}

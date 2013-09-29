use std::{io, run, os, vec};

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
		hist.push(copy cmd);
            match program {
                ~"exit"     => {return; }
                		//must reflect change in directory to mut cwd ^^^
                ~"cd"	    => { os::change_dir(~PosixPath(argv.remove(0)));}
                ~"history"  => {

			 for hist.slice(1, hist.len()).iter().advance |s|{			println(*s);
			


			}



}
                _           => {run::process_status(program, argv);}
            }
        }
    }
}

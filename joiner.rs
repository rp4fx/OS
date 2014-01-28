use std::rand::random;
use std::os;
use std::io::File;

fn main() {
    let args: ~[~str] = os::args();
    if args.len() != 3 {
        println!("Usage: {:s} <inputfile>", args[0]); 
    } else {
        let fname = args[1].clone();
		let fname2 = args[2];
        let path = Path::new(fname.clone());
		let path2 = Path::new(fname2.clone());
        let msg_file = File::open(&path);
		let msg_file2 = File::open(&path);

        match (msg_file,msg_file2) {
            (Some(mut msg),Some(mut msg2)) => {
                let msg_bytes: ~[u8] = msg.read_to_end();
				let msg_bytes2:~[u8] = msg2.read_to_end();
                let share1_file = File::create(&Path::new("name" + ".txt"));
				
        
                
               /* match (share1_file) {
                    (Some(file)) => { 
                        split(msg_bytes, msg_bytes2,share1_file); 
                        } ,
                    (_, _) => fail!("Error opening output files!"),
                }*/
            } ,
			
            (None,_) => {fail!("Error opening message file: {:s}", fname)},
			(_,None) => {fail!("Error opening message file: {:s}", fname)}
    }
	}
}

fn xor(a: &[u8], b: &[u8]) -> ~[u8] {
    let mut ret = ~[];
    for i in range(0, a.len()) {
	ret.push(a[i] ^ b[i]);
    }
    ret
}

fn split(msg_bytes: &[u8], msg_bytes2: &[u8], mut share2: File){
   /* let mut random_bytes: ~[u8] = ~[];
    // This is not cryptographically strong randomness! 
    // (For entertainment purposes only.)
    for _ in range(0, msg_bytes.len()) {
	let random_byte = random();
	random_bytes.push(random_byte);
    }*/
	
    
    let encrypted_bytes = xor(msg_bytes, msg_bytes2);
	share2.write(encrypted_bytes);
    
}





//GLOBAL VARIABLES



//global variables for the construcing of the whiskey wine data, change if needed




const EXE_PATH_JSON_KEY:&'static str = "EXE_PATH";
const WHISKEY_WINE_DIR_NAME:&'static str="WHISKEY_WINE_DONOTEDIT";
const WHISKEY_JSON_PATHS_FILE_NAME:&'static str = "path_data.json";
const WHISKEY_WINE_SHELL_START_FILE_NAME:&'static str="start_main_exe.sh";
const FIND_PID:&'static str="pid=$!\necho \"PID{$pid\"\n";
const PID_KEY:[char;4]=['P','I','D','{'];
const BASH_FORMAT:&'static str="#!/bin/bash\n";
const WINE_RUN_COMMAND:&'static str = "wine start /unix ";
const WINE_RUN_COMMAND_END_PART:&'static str = " &\n";
//END OF GLOBAL VARIABLES

//STRUCTS
pub struct WindowsProcess{
    path:String,
    whiskey_files_path:String,
    pid:usize,
    running:bool,
    std_output:Option<std::process::Output>,
    channel_send:Option<std::sync::mpsc::Sender<u8>>,
    channel_receive:Option<std::sync::mpsc::Receiver<u8>>

}
//END OF STRUCTS
//IMPLS

// A couple commands used to run and debug the windows exe
impl WindowsProcess{
    

    pub fn run(&mut self)->Result<usize,std::io::Error>{
        use std::io::Error;
        use std::io::ErrorKind::*;
        use std::sync::mpsc;
        use general_functions::*;

        match self.running{
            true=>return Err(Error::new(AlreadyExists,"process is already running")),
            false=>{}
        }

        let (tx,rx)=mpsc::channel::<u8>();

        self.channel_send=Some(tx);

        self.channel_receive=Some(rx);

        

        return Ok(

            run_wine_start_shell_script(&self.whiskey_files_path)?

            );
    }


}


//END OF IMPLS

//modules

//A list of small functions used by the library to make the code more understandable
mod general_functions{
    use crate::{WHISKEY_WINE_SHELL_START_FILE_NAME, PID_KEY, WHISKEY_JSON_PATHS_FILE_NAME};

    

    
   

    macro_rules!  add_forward_slash_to_path{
        ($a:expr) => {
            {
                String::from($a)+"/"
            }
            
        };
    }
    macro_rules!  add_backward_slash_to_path{
        ($a:expr) => {
            {
                String::from($a)+"\\"
            }
            
        };
    }

    //adds the whiskey wine directory name to the provided path str
    pub fn add_whiskey_files_dir_to_path(path:&str)->String{
        
        use super::*;

        let current_path:String = 
            check_then_add_slash_to_path(
                path,
                check_for_slash_type(path)) + WHISKEY_WINE_DIR_NAME;

        return current_path;

    }
    
    //adds the paths.json file to path 
    pub fn add_whiskey_json_file_to_path(path:&str)->String{
        
        use super::*;

        let current_path:String = 
            check_then_add_slash_to_path(
                path,
                check_for_slash_type(path)) + WHISKEY_JSON_PATHS_FILE_NAME;

        return current_path;

    }

    //spawns the paths json file
    pub fn spawn_paths_json_file(path:&str)->Result<(),std::io::Error> {
        
        use std::fs::File;
        
        let json_file_path:String = add_whiskey_json_file_to_path(path);
        
        File::create(json_file_path)?;
            
        return Ok(());
    }

    //writes exe path data to paths json file 
    pub fn write_exe_path_data_json_file(path:&str,exe_path:&str)->Result<(),std::io::Error>{
        
        use serde_json::{Map, Value,to_string};
        use std::fs::*;
        use super::*;

        let mut json_data:Map<String,Value>=Map::new();
        
        json_data.insert(EXE_PATH_JSON_KEY.to_string(), exe_path.into());

        let json_file:String = add_whiskey_json_file_to_path(path);

        write(json_file,&to_string(&json_data)?)?;

        return Ok(());
    }

    //creates the json paths file and writes the exe path to it
    pub fn create_then_write_json_paths_file(path:&str,exe_path:&str)->Result<(),std::io::Error>{
        
        spawn_paths_json_file(path)?;

        write_exe_path_data_json_file(path, exe_path)?;
        
        return Ok(());
    }

    //gets the pid out of a string
    pub fn get_pid_from_string(string:&str)->Result<usize,std::io::Error>{

        use std::io::Error;
        use std::io::ErrorKind::*;

        if string.contains(PID_KEY) != true{
            return Err(Error::new(
                    NotFound, 
                    "pid key not inside string"));
        }

        let mut buffer: usize = 1;
        let mut pid_string: String = String::new();
        let mut read_key: bool = false;
        let mut read_pid: bool = false;

        for character in string.chars(){
            
            if read_key == false && read_pid == false {
                
                if character == PID_KEY[0] {
                    
                    read_key = true;

                }
    
            }

            if read_key {
                
                if character == PID_KEY[ buffer ] {
                    
                    buffer+=1;



                }
                

                

            }

            if read_pid {
                
                if character.is_numeric() != true {
                    
                    break;
                }
                
                pid_string+=&character.to_string();
            }

            if buffer == PID_KEY.len(){

                read_key = false;
                
                read_pid = true;

            }

        }

        return Ok(pid_string.parse().unwrap());
    }
    //constructs the start shell script, with the path parameter to point to the exe
    pub fn construct_shell_wine_start_file(path:&str)->String{
        use super::*;

        let mut script:String=String::from(BASH_FORMAT)+WINE_RUN_COMMAND+path+WINE_RUN_COMMAND_END_PART;
        script=script+FIND_PID;
        return script;

    }

    /*
    adds a slash at the end of the path if there inst, the first parameter is the path and second is a bool to tell the function 
    weither it should be a foward slash or a back slash 
    ||true for forward slash false for backwards slash||

    */
    pub fn check_then_add_slash_to_path(path:&str,forward_slash:bool)->String{
        let mut new_path:String=String::from(path);
        match new_path.as_bytes()[new_path.len()-1] as char{
            '/'=>return new_path,
            '\\'=>return new_path,
            _=>{}
        }
        new_path=match forward_slash{
            true=>add_forward_slash_to_path!(new_path),
            false=>add_backward_slash_to_path!(new_path)
        };
        
        return new_path

    }
    /*
    this function checks what kind of slashes a string that contains a path uses, returns true for a forward slash
    returns false for a back slash
    */
    pub fn check_for_slash_type(path:&str)->bool{
        return path.contains("/");
    }
    //this function add the directory for whiskey wine files directory to the path supplied into
    //the parameter 
    pub fn add_whiskey_wine_dir_to_path(path:&str)->String{
        return check_then_add_slash_to_path(&path, check_for_slash_type(&path))+super::WHISKEY_WINE_DIR_NAME;
    }
    //this function adds the file for the whiskey wine api into the supplied path
    pub fn add_whiskey_wine_shell_start_file(path:&str)->String{
        return check_then_add_slash_to_path(&path, check_for_slash_type(&path))+super::WHISKEY_WINE_SHELL_START_FILE_NAME;
    }
    //spawns a file, the file path should be in the first parameter, and the second parameter
    //should be the data to write to the file
    pub fn create_then_write_shell_file(path:&str,contents:&str)->Result<(),std::io::Error>{
        use std::io::Write;
        use std::fs::File;
        use std::fs::set_permissions;

        File::create(path)?.write_all(contents.as_bytes())?;
        return Ok(());
    
    }
    //runs the wine start shell script and returns the pid of the process
    pub fn run_wine_start_shell_script(whiskey_wine_files_dir:&str)->Result<usize,std::io::Error>{
        use std::process::Command;
        use std::process::Output;
        use std::time::Duration;
        use std::thread;
        use std::io::Error;
        use std::io::ErrorKind::*;
        use std::process::Stdio;

        let complete_path: String = check_then_add_slash_to_path(
            whiskey_wine_files_dir,
            check_for_slash_type(
                whiskey_wine_files_dir)
            )+WHISKEY_WINE_SHELL_START_FILE_NAME;

        let  output:Output;

        let output = Command::new("bash")
                .arg(complete_path)
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn()
                .expect("Failed to execute command");

        println!("DPNE");
        
        
            
        thread::sleep(Duration::from_millis(350));

        let output_result = output.wait_with_output().expect("Failed to wait for command");

        let stdout: String = String::from_utf8_lossy(&output_result.stdout).to_string();
        
        let pid: usize = get_pid_from_string(&stdout)?;
        
        return Ok(pid);

    }
    
}


//end of modules


//FUNCTIONS
//constructs and builds the windows process and whiskey wine data for it to be run, the path
//parameter should be the path to the exe and spawn_path should be to the path you want the whiskey
//wine files to spawn
pub fn define_process(path:&str,spawn_path:&str)->Result<WindowsProcess,std::io::Error>{
    use general_functions::*;
    use std::path::*;
    use std::fs::*;
    use std::io::Error;
    use std::io::ErrorKind::*;

    match Path::new(path).exists(){
        false=>return Err(Error::new(NotFound,"path does not point to valid .exe file ")),
        true=>{}
    }
    match Path::new(path).is_dir(){
        true=>return Err(Error::new(NotFound,"is a folder instead of a valid .exe file")),
        false=>{}
    }
    let current_whiskey_wine_dir: String=add_whiskey_wine_dir_to_path(spawn_path);
    let current_start_wine_shell_script_dir: String=add_whiskey_wine_shell_start_file(&current_whiskey_wine_dir);


    match Path::new(&current_whiskey_wine_dir).is_dir(){
        true=>remove_dir_all(&current_whiskey_wine_dir)?,
        false=>{}
    }

    create_dir(&current_whiskey_wine_dir)?;
    println!("{}",&current_start_wine_shell_script_dir);
    
    create_then_write_json_paths_file(&current_whiskey_wine_dir,path)?;

    create_then_write_shell_file(
        &current_start_wine_shell_script_dir, 
        &construct_shell_wine_start_file(&path))?;

    return Ok(WindowsProcess { 
        path:path.to_string(), whiskey_files_path:current_whiskey_wine_dir.to_string() , 
        pid:0, running:false, 
        std_output:None, channel_send:None,
        channel_receive:None
    });
    

}
//END OF FUNCTIONS

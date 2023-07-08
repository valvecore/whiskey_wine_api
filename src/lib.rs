




//GLOBAL VARIABLES



//global variables for the construcing of the whiskey wine data, change if needed





const WHISKEY_WINE_DIR_NAME:&'static str="WHISKEY_WINE_DONOTEDIT";
const WHISKEY_WINE_SHELL_START_FILE_NAME:&'static str="start_main_exe.sh";
const FIND_PID:&'static str="pid=$!\necho \"PID{$pid\"\n";
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
    output:Option<std::process::Output>


}
//END OF STRUCTS


//modules

//A list of small functions used by the library to make the code more understandable
mod general_functions{
    
   

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
    
    //constructs the start shell script, with the path parameter to point to the exe
    pub fn construct_shell_wine_start_file(path:&str)->String{
        use super::*;

        let mut script:String=String::from(BASH_FORMAT)+WINE_RUN_COMMAND+WINE_RUN_COMMAND_END_PART;
        script=script+path+WINE_RUN_COMMAND_END_PART+FIND_PID;
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
    pub fn create_then_write_file(path:&str,contents:&str)->Result<(),std::io::Error>{
        use std::io::Write;
        use std::fs::File;
        File::create(path)?.write_all(contents.as_bytes())?;
        return Ok(());
    
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

    create_then_write_file(
        &current_start_wine_shell_script_dir, 
        &construct_shell_wine_start_file(&path))?;

    return Ok(WindowsProcess { path:path.to_string(), whiskey_files_path:current_whiskey_wine_dir.to_string() , pid:0, running:false, output:None});
    

}
//END OF FUNCTIONS

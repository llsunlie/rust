use std::{fs::{File, self}, io::{ErrorKind, self, Read}, error::Error};

// Box<dyn Error> 为使用 ? 时 main 允许返回的 “任何类型的错误”。
fn main() -> Result<(), Box<dyn Error>>{
    // panic!("crash and burn");

    /* backtrace */
    let v = vec![1, 2, 3];

    // v[99]; // panic

    /* Result */
    let f = File::open("a.txt");
    
    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("a.txt") {
                Ok(fc) => fc,
                Err(e) => {
                    panic!("Problem create the file: {:?}", e);
                }
            },
            other_error => {
                panic!("Problem open the file: {:?}", other_error);
            }
        },
    };

    /* closure */
    let f = File::open("b.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("b.txt").unwrap_or_else(|error| {
                panic!("Problem create the file: {:?}", error);
            })
        } else {
            panic!("Problem open the file: {:?}", error);
        }
    });

    /* unwrap */
    let f = File::open("c.txt").unwrap();
    let f = File::open("c.txt").expect("Failed to open c.txt");

    /* propagating */
    let s = read_username_from_file().unwrap();

    /* ? */
    let s = read_username_from_file_2().unwrap();

    /* ? in main */
    let f = File::open("e.txt")?; // ? can only be used in function that return 'Result' or 'Option'
    
    Ok(())

}

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("d.txt");
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => return Err(e),
    }
}

fn read_username_from_file_2() -> Result<String, io::Error> {
    let mut f = File::open("d.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn read_username_from_file_3() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("d.txt")?.read_to_string(&mut s)?;

    Ok(s)
}

fn read_username_from_file_4() -> Result<String, io::Error> {
    fs::read_to_string("d.txt")
}
use std::{time, thread, io};
use device_query::{DeviceState, Keycode, DeviceQuery};
use enigo::*;

static LISTEN_INTERVAL_MILLIS: u32 = 50;

fn main() {
    let interval_secs = read_secs();
    wait_start(10);
    let mut enigo = Enigo::new();
    let mut count = 0;
    loop {
        enigo.mouse_click(MouseButton::Right);
        count += 1;
        println!("[MouseButton::Right] count = {}", count);
        if listen_end_key(interval_secs) {
            break;
        }
    }
    println!("end!");
}

fn read_secs() -> u32{
    loop {
        println!("please input the interval secs:");
        let mut secs = String::new();
        io::stdin()
            .read_line(&mut secs)
            .expect("Fail to read the number!!!");
        let secs: u32 = match secs.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        return secs;
    }
}

fn wait_start(secs: i32) {
    let one_secs = time::Duration::from_secs(1);
    for i in (0..secs).rev() {
        println!("{}!", i);
        thread::sleep(one_secs);
    }
}

fn listen_end_key(duration_secs: u32) -> bool{
    let duration_millis = duration_secs * 1000;
    let loop_times = duration_millis / LISTEN_INTERVAL_MILLIS;
    let ten_millis = time::Duration::from_millis(LISTEN_INTERVAL_MILLIS as u64);
    let device_state = DeviceState::new();
    for i in 0..loop_times {
        let keys: Vec<Keycode> = device_state.get_keys();
        // println!("keys = {:?}", keys);
        if keys.len() != 0 && keys[0] == Keycode::F2 {
            return true;
        }
        thread::sleep(ten_millis);
    }
    return false;
}
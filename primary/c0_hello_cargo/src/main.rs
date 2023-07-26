use std::{io, collections::btree_map::Range};

const LEN: i32 = 3 * 6 * 4;

fn main() {
    let mut choose_pos_id = String::new();
    let mut end_pos = String::new();

    io::stdin().read_line(&mut choose_pos_id).unwrap();
    io::stdin().read_line(&mut end_pos).unwrap();

    let choose_pos_id: i32 = choose_pos_id.trim().parse().unwrap();
    let end_pos: i32 = end_pos.trim().parse().unwrap();

    println!("choose_pos_id = {}", choose_pos_id);
    println!("end_pos = {}", end_pos);

    let mut s = String::new();
    s.push_str(&(choose_pos_id + LEN + 1).to_string());
    for i in 0..4 {
        s.push_str(";");
        for j in 0..6 {
            let t = choose_pos_id + i * 18 + j * 3 + 1;
            s.push_str(&t.to_string());
            if j != 5 {
                s.push_str(",");
            }
        }
    }
    println!("=====");
    println!("{}", s);

    for i in 0..4 {
        s.push_str(";");
        for j in 0..6 {
            let t = choose_pos_id + i * 18 + j * 3 + 1;
            let mut s = String::new();
            s.push_str(&(t + 1).to_string());
            s.push_str(";");
            s.push_str(&(end_pos).to_string());
            println!("{}", s);

            let mut s = String::new();
            s.push_str(&(t + 2).to_string());
            s.push_str(";");
            s.push_str(&(end_pos).to_string());
            println!("{}", s);

            println!("{}", end_pos);
        }
    }

}

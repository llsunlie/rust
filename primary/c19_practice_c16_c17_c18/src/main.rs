use std::collections::HashMap;

fn problem_1() {
    let mut v = vec![1, 2, 7, 4, 2, 6, 9, 8];
    // mean
    let mut sum: i32 = 0;
    for i in &v {
        sum += *i;
    }
    let mean = sum as f64 / v.len() as f64;
    // midden number
    v.sort();
    let midden_number = v.get(v.len() / 2);
    // mode
    let mut mode = 0;
    let mut max_count = 0;
    let mut map = HashMap::new();
    for i in &v {
        let count = map.entry(i).or_insert(0);
        *count += 1;
    }
    // println!("map = {:#?}", map);
    for (key, value) in &map {
        if *value > max_count {
            max_count = *value;
            mode = **key;
        }
    }
    println!("mean = {}, midden_number = {:?}, mode = {}", mean, midden_number, mode);

}

fn problem_2(str: &str) -> String {
    let s = String::from(str);
    let mut chars = s.chars();
    let first = chars.nth(0);
    let first = match first {
        Some(c) => c,
        None => {
            return "".to_string();
        },
    };
    if first == 'a' || first == 'e' || first == 'i' || first == 'o' || first == 'u' {
        let mut res = s;
        res.push_str("-ay");
        return res;
    } else {
        let mut res = String::from(&s[1..]);
        res.push_str("-ay");
        return res;
    }
}

fn main() {
    /* 给定一系列数字，使用 vector 并返回这个列表的平均数（mean, average）、中位数（排列数组后位于中间的值）
    和众数（mode，出现次数最多的值；这里哈希函数会很有帮助）。 */
    problem_1();
    println!("====================================");
    
    /* 将字符串转换为 Pig Latin，
    也就是每一个单词的第一个辅音字母被移动到单词的结尾并增加 “ay”，
    所以 “first” 会变成 “irst-fay”。
    元音字母开头的单词则在结尾增加 “hay”（“apple” 会变成 “apple-hay”）。
    牢记 UTF-8 编码！ */
    let s1 = "first";
    let r1 = problem_2(s1);
    println!("r1 = {}", r1);
    let s2 = "apple";
    let r2 = problem_2(s2);
    println!("r2 = {}", r2);
    println!("====================================");

    /* 使用哈希 map 和 vector，
    创建一个文本接口来允许用户向公司的部门中增加员工的名字。
    例如，“Add Sally to Engineering” 或 “Add Amir to Sales”。
    接着让用户获取一个部门的所有员工的列表，
    或者公司每个部门的所有员工按照字典序排列的列表。 */
    problem_3();

}


fn problem_3() {
    let mut map: HashMap<String, Vec<String>> = HashMap::new();
    add_staff(&mut map, "Engineering", "Sally");
    add_staff(&mut map, "Engineering", "Jack");
    add_staff(&mut map, "Sales", "Amir");
    show_department(&mut map, "Engineering");
    show_department(&mut map, "Sales");
    show_department(&mut map, "random");
}

fn add_staff(map: &mut HashMap<String, Vec<String>>, department: &str, name: &str) {
    let vec = map.entry(String::from(department)).or_insert(Vec::new());
    vec.push(String::from(name));
}

fn show_department(map: &mut HashMap<String, Vec<String>>, department: &str) {
    let vec = map.get(department);
    match vec {
        Some(v) => {
            println!("staff in {}:", department);
            for name in v {
                println!("{}", name);
            }
        },
        None => {
            println!("there is no staff in {}", department);
        },
    }
}
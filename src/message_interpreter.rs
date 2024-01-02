use crate::*;

pub async fn load_answer_template(vars: &HashMap<String, String>) -> String {
    let answer_template_path = &vars["ANSWER_TEMPLATE_PATH"];
    let answer_template = tokio::fs::read(answer_template_path).await.unwrap();
    String::from_utf8(answer_template).unwrap()
}

pub fn get_values(message: &String, vars: &HashMap<usize, (String, String)>) -> HashMap<String, String> {
    let mut values = HashMap::<String, String>::new();
    for pair in vars.iter() {
        if pair.1.0.as_str() == "END" { continue; }
        let start = message.find(&pair.1.1).unwrap() + pair.1.1.len();
        let end = message[start+1..].find(&vars.get(&(pair.0+1)).unwrap().1).unwrap() + start;
        values.insert(pair.1.0.clone(), message[start..end+1].to_string());
    }
    
    values
}

pub async fn load_template(vars: &HashMap<String, String>) -> String {
    let template_path = &vars["TEMPLATE_PATH"];
    let template = tokio::fs::read(template_path).await;
    String::from_utf8(template.unwrap()).unwrap()
}

pub fn init_template_vars(template: &String, beg: char, end: char) -> HashMap<usize, (String, String)> {
    let mut vars = HashMap::<usize, (String, String)>::new();
    let mut last_end = 0;
    let mut index = 0;
    let mut order = 1;
    for c in template.chars() {
        if c == beg {
            let text_before = template[last_end..index].to_string();
            let end_index = template[index+1..].find(end).unwrap() + index + 1;
            last_end = end_index+1;
            vars.insert(order, (template[index..last_end].to_string(), text_before));
            order += 1;
        }
        index += c.len_utf8();
    }
    vars.insert(order, ("END".to_string(), template[last_end..].to_string()));
    vars
}
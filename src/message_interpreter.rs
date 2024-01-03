use crate::{*, email_extension::EmailExtension};

pub fn skip(iter: &mut impl Iterator<Item = char>, x: usize) {
    for _ in 0..x {
        iter.next();
    }
}

pub async fn process_messages(hub: &Gmail<HttpsConnector<HttpConnector>>, vars: &HashMap<String, String>) {
    let args = get_args();
    let template = load_template(&vars).await;
    let template_vars = init_template_vars(&template, &vars["BEG"], &vars["END"]);
    let answer_template = load_answer_template(&vars).await;
    
    let mut ids = get_ids(&hub).await;

    for message in ids.iter_mut() {
        if *message.get_subject(&hub).await != args["-s"] { continue; }
        
        let message = message.get_message(&hub).await;
        let vals = get_values(&message, &template_vars);
        let mut answer = answer_template.clone();

        for pair in vals.iter() {
            answer = answer.replace(pair.0, pair.1);
        }
        
        println!("Final Message: {}", answer);
    }
}

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

pub fn init_template_vars(template: &String, beg: &str, end: &str) -> HashMap<usize, (String, String)> {
    let mut vars = HashMap::<usize, (String, String)>::new();
    let mut last_end = 0;
    let mut index = 0;
    let mut order = 1;
    let mut iterator = template.chars().peekable();
    
    while index < template.len() && iterator.peek().is_some() {
        let mut adv_by = 1;
        let mut add_index = iterator.peek().unwrap().len_utf8();
        if iterator.peek().unwrap() == beg.chars().peekable().peek().unwrap() {
            let text_before = template[last_end..index].to_string();
            last_end = template[index+beg.len()..].find(end).unwrap() + index + beg.len() + end.len();
            vars.insert(order, (template[index..last_end].to_string(), text_before));
            order += 1;
            adv_by += beg.len() + end.len() + template[index+beg.len()..last_end].len();
            add_index += adv_by - 1;
        }
        index += add_index;
        skip(&mut iterator, adv_by);
    }
    
    vars.insert(order, ("END".to_string(), template[last_end..].to_string()));
    vars
}
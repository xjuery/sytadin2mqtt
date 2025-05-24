use regex::Regex;
use select::document::Document;
use select::predicate::{Class, Name};
use serde_json::json;

fn get_sytadin_traffic_page() -> Document {
    let body_result = reqwest::blocking::get("http://www.sytadin.fr/sys/barometres_de_la_circulation.jsp.html").unwrap().text();
    let body = body_result.unwrap();
    Document::from(body.as_str())
}

fn get_tf_value(doc: Document) -> f32 {
    let value = doc.find(Class("barometre_valeur")).next().unwrap().text();
    let value_re: Regex = Regex::new("\\s*([0-9]+).*").unwrap();
    let test = value_re.captures(value.as_str()).unwrap();
    test[1].parse::<f32>().unwrap()
}

fn get_tf_trend(doc: Document) -> String {
    for node in doc.find(Class("barometre_tendance")) {
        return node
            .find(Name("img"))
            .next()
            .unwrap()
            .attr("alt")
            .unwrap()
            .to_string();
    }

    String::new()
}

fn get_tf_level(doc: Document) -> String {
    for node in doc.find(Class("barometre_niveau")) {
        return node
            .find(Name("img"))
            .next()
            .unwrap()
            .attr("alt")
            .unwrap()
            .to_string();
    }

    String::new()
}

fn convert_to_json(document: Document) -> String {
    json!({"level":get_tf_level(document.clone()),"trend":get_tf_trend(document.clone()),"value":get_tf_value(document.clone())}).to_string()
}
    
pub fn get_traffic_data() -> String {
    let document = get_sytadin_traffic_page();
    convert_to_json(document)
}


#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_get_tf_value() {
        let doc = Document::from("<html><body><div class=\"barometre_valeur\">111</div></body></html>");
        let data = get_tf_value(doc);

        assert_eq!(data, 111.0);
    }
    
    #[test]
    fn test_get_tf_trend() {
        let doc = Document::from("<html><body><div class=\"barometre_tendance\"><img alt=\"Habituel\">XXXXX</img></div></body></html>");
        let data = get_tf_trend(doc);

        assert_eq!(data,"Habituel");
    }

    #[test]
    fn test_get_tf_level() {
        let doc = Document::from("<html><body><div class=\"barometre_niveau\"><img alt=\"Fort\">XXXXX</img></div></body></html>");
        let data = get_tf_level(doc);

        assert_eq!(data,"Fort");
    }

    #[test]
    fn test_convert_to_json() {
        let doc = Document::from("<html><body><div class=\"barometre_valeur\">222</div><div class=\"barometre_tendance\"><img alt=\"Inhabituel\">XXXXX</img></div><div class=\"barometre_niveau\"><img alt=\"Eleve\">XXXXX</img></div></body></html>");
        let data = convert_to_json(doc);

        assert_eq!(data,"{\"level\":\"Eleve\",\"trend\":\"Inhabituel\",\"value\":222.0}");
    }
}
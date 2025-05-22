use regex::Regex;
use select::document::Document;
use select::predicate::{Class, Name};
use serde_json::json;

pub(crate) struct Sytadin {
    
}

impl Sytadin {
    pub(crate) fn new () -> Sytadin {
        Sytadin{}
    }
    fn get_sytadin_traffic_page(&self) -> Document {
        let body_result = reqwest::blocking::get("http://www.sytadin.fr/sys/barometres_de_la_circulation.jsp.html").unwrap().text();
        let body = body_result.unwrap();
        Document::from(body.as_str())
    }

    fn get_tf_value(&self, doc: Document) -> f32 {
        let value = doc.find(Class("barometre_valeur")).next().unwrap().text();
        let value_re: Regex = Regex::new("\\s*([0-9]+).*").unwrap();
        let test = value_re.captures(value.as_str()).unwrap();
        test[1].parse::<f32>().unwrap()
    }

    fn get_tf_trend(&self, doc: Document) -> String {
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

    fn get_tf_level(&self, doc: Document) -> String {
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
    
    pub fn get_traffic_data(&self) -> String {
        let document = self.get_sytadin_traffic_page();
        let tf_value = self.get_tf_value(document.clone());
        let tf_trend = self.get_tf_trend(document.clone());
        let tf_level = self.get_tf_level(document.clone());
        json!({"level":tf_level,"trend":tf_trend,"value":tf_value}).to_string()
    }

}
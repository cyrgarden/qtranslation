use qmetaobject::QJsonObject;
use qmetaobject::{qt_base_class, qt_method, qt_property, qt_signal, QJsonArray, QObject, QString};
use serde_json::Value;
use std::borrow::Borrow;
use std::collections::HashMap;
use std::fs::read_to_string;
use sys_locale::get_locale;

#[derive(QObject, Default)]
pub struct QTranslater {
    base: qt_base_class!(trait QObject), //Needed to make the macro works

    /*  QJsonObject used as a dictionary :
        - the "key" is the word / variable you want to translate
        - the "value" is it's translation
    */
    dict: qt_property!(QJsonObject; NOTIFY dict_changed),
    array: qt_property!(QJsonArray; NOTIFY dict_changed),

    json_path: String, // relative path to the Json file
    lang: String,      // current language --> Default : your computer language

    init: qt_method!(fn(&mut self)),         // Constructor method
    refresh_dict: qt_method!(fn(&mut self)), //to refresh datas
    change_lang: qt_method!(fn(&mut self, new_lang: QString)), // Methode to switch to change language

    dict_changed: qt_signal!(), //just a qt_signal() that is triggered when modifications are done so the changes are displayed
}

impl QTranslater {
    pub fn init(&mut self) {
        self.lang = get_locale().unwrap_or_else(|| String::from("en-US")); //Get your computer lang
        self.json_path = "./src/lang/".to_string() + self.lang.borrow() + ".json";

        let data: String;
        match read_to_string(&self.json_path) {
            Ok(v) => data = v,
            Err(_err) => data = read_to_string(&self.json_path).expect("can't find english dict"),
        }

        //let data = read_to_string(&self.json_path).expect("file not found"); //Charge json data from a string

        let mut lookup: HashMap<String, Value> = serde_json::from_str(&data).unwrap();
        let mut keys: Vec<String> = Vec::new(); //All the keys will be stored in this Vec

        for (k, _v) in lookup.clone() {
            keys.push(k.to_string()); //Storing all the key in the vec as String
        }
        let mut map = HashMap::new(); //First dictionnary version which is not Q-compatible

        for key in &keys {
            let (k, v) = lookup.remove_entry(key).unwrap(); // Extracting the value and deleting the key
            let word = v.as_str().expect("output is not a string");
            map.insert(k, word.to_string()); //insert in the dict a key with its corresponding value
        }

        self.dict = map.into(); // We convert the dict HashMap into a QJsonObject , which is Q-compatible
        self.dict_changed(); //Trigger the signal, so the interface can display changes
    }

    //Works the same way as the init(). Exception is that this one takes an arg which correspond to the lang you want to switch to
    fn change_lang(&mut self, new_lang: QString) {
        self.lang = new_lang.to_string();
        self.json_path = "./src/lang/".to_string() + self.lang.borrow() + ".json";
        let data = read_to_string(&self.json_path).expect("file not found");

        let mut lookup: HashMap<String, Value> = serde_json::from_str(&data).unwrap();
        let mut keys: Vec<String> = Vec::new();

        for (k, _v) in lookup.clone() {
            keys.push(k.to_string());
        }

        let mut map = HashMap::new();
        for key in &keys {
            let (k, v) = lookup.remove_entry(key).unwrap();
            let word = v.as_str().expect("output is not a string");

            map.insert(k, word.to_string());
        }

        self.dict = map.into();
        self.dict_changed();
    }

    fn refresh_dict(&mut self) {
        self.dict_changed();
    }
}

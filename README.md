# QTranslater
Ce "crate" est un petit système de traduction pour les QMetaOjbect qui est basé sur l'utilisation de Json.


## Objectif : 
Permettre de traduire simplement des variables pour les interfaces qui utilisent le crate "https://github.com/woboq/qmetaobject-rs"

## Mise en place 

```rust
    //For example, it's in my main.rs
    //We start with import*
use qmetaobject::prelude::*; 
use qtranslation::QTranslater;

    fn main(){
        /*
        Some code...
        */
        qml_ressources(); 
        qml_register_type::<QTranslater>(cstr!("QTranslater"), 1, 0, cstr!("QTranslater")); //Register the QTranslater type
        let mut engine = QmlEngine::new();
        engine.load_file("qrc:/main/qml/main_window.qml".into()); //"main_window" is just my qml first window. It can be any other page
        engine.exec();

    }
    
```

```qml
    property var global_opacity : 0.90

    property var global_bg: "#f0f0f0"
   
    property var global_bg1: "#ACD9E6"
    property var global_bg2: "#D8F1E9"

    property var title_color: "#000000"


```



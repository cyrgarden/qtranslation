# QTranslater
This crate is a very small translation system based on Json for QMetaOjbect.
It was originally created for the translation of the [Grace software](https://gitlab.cairn-devices.eu/cairntech/grace)(by Cairn Devices).


## Goal
Allow simple translation of variables for interfaces that use the [qmetaobject](https://github.com/woboq/qmetaobject-rs) crate.

## Setup


### In your folder project 
Create a "lang" folder in your "src" folder. In this "lang" folder create a json file for every languages you want to switch to. <br><br>
![Screenshot](./screenshot.png)

### In a Json file
For example in "fr_FR.json"
```json
{
    "water": "eau",
    "battery": "Batterie",
    "trains":"age",
    "global_infos": "Informations générales", 
}
```

### In your rust file
By default, the library will determine the language of your computer using ["sys-local::get_local"](https://crates.io/crates/sys-locale).
If the language is not present in the "lang" folder, it will automatically load an "en_GB.json".
So please provide an "en_GB.json" file to avoid any crash of your application.

```rust
//For example: in my main.rs
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
        engine.load_file("qrc:/main/qml/main_window.qml".into()); //"main_window" is just my qml first window.But it can be any other page.
        engine.exec();

    }
    
```
### In your QML file
QTranslater init() function takes one argument "folder_path" a String. This argument determines the file in which the Qtranslater will look for json files.
There are three options : 
<ol>
    <li>folder_path = "dev_path" --> in this case the path will be ./src/lang/ </li>
    <li>folder_path = "installed_path" --> in this case the path will be /home/{username}/.local/share/cairn-grace/lang/ </li>
    <li>Else, if folder_path is an other path that you choose. For example : "/home/username/Desktop/random_folder/" --> the </li>
</ol>

```qml
//Import the QTranslater object
import QTranslater 1.0

ApplicationWindow{
    id:my_window
    visible:true
    width : 800
    height:800

    QTranslater{
        id: lang //Assign an ID you want
        Component.onCompleted: {
            init(installed_path); // it can be "dev_path" or a custom path like the one in the example below
            //init(dev_path);
            //init("/home/username/Desktop/random_folder/")
        }
    }
    //Some code...
    ColumnLayout{
        Text{
            text: lang.dict["global_informations"]
        }
        Text{
            text: lang.dict["water"]
        }
        Text{
            text: lang.dict["battery"] + ": 23
            font.bold: true
        }

        Text{
            textFormat: Text.RichText
            text: "<html><span style='font-weight: bold '>I love</span><span>" +lang.dict.["trains"] +"</span><\html>"
            color: "red"
            font.family: ubuntu_regular.name
        }
        
        Button{
            text: "Change to French"
            onClicked: lang.change_lang("fr_FR") // To change language
        }
    } 
}


```





#QTranslater
Ce "crate" est un petit système de traduction pour les QMetaOjbect qui est basé sur l'utilisation de Json.


##Objectif : 
Permettre de traduire simplement des variables pour les interfaces qui utilisent le crate "https://github.com/woboq/qmetaobject-rs"

```rust
    init: qt_method!(fn(&mut self)),         // Constructor method
    refresh_dict: qt_method!(fn(&mut self)), //to refresh datas
    change_lang: qt_method!(fn(&mut self, new_lang: QString)), // Methode to switch to change language


```

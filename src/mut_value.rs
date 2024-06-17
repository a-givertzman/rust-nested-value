use std::fmt::Debug;
use chrono::Utc;
use crate::nested_value::NestedValue;
///
/// Contains the mutable value, returns on call get() method
/// - if editors are specified, store() method will be accepted only from defined editor,
///   otherwise store() will panic.
#[derive(Clone)]
pub struct MutValue<T> {
    id: String,
    inited: bool,
    value: T,
    edited: Vec<String>,
    editors: Vec<String>,
}
//
//
impl<T: Clone + Debug> MutValue<T> {
    ///
    /// Returns new instance of the [MutValue]
    pub fn new(value: T) -> Self {
        let mut me = Self {
            id: "MutValue".to_owned(),
            inited: false,
            value: value.clone(),
            edited: vec![],
            editors: vec![],
        };
        Self::register_edit(&mut me, "self", &value);
        me
    }
    ///
    /// Returns new instance of the [MutValue] with allowed editors
    pub fn with_editors(value: T, editors: Vec<String>) -> Self {
        let mut me = Self {
            id: "MutValue".to_owned(),
            inited: false,
            value: value.clone(),
            edited: vec![],
            editors
        };
        Self::register_edit(&mut me, "self", &value);
        me
    }
    ///
    /// Registers edited event
    fn register_edit(&mut self, editor: &str, value: &T) {
        self.edited.push(format!("{}. {} - {} ({:?})", self.edited.len() + 1, Utc::now(), editor, value));
    }
    ///
    /// Returns contained value
    pub fn get(&self) -> T {
        self.value.clone()
    }
    ///
    /// Stores a new value
    pub fn store(&mut self, editor: &str, value: T) -> Result<(), String> {
        if !self.editors.is_empty() {
            if !self.editors.contains(&editor.to_owned()) {
                panic!("{}.store | Write access denied for editor '{}' ", self.id, editor);
            }
        }
        self.register_edit(editor, &value);
        self.value = value;
        // self.edited.push(format!("{}. {} - {} ({:?})", self.edited.len() + 1, Utc::now(), editor, value));
        // println!("{}.store | edited: {:#?}", self.id, self.edited);
        Ok(())
    }
}
//
//
impl<T: Clone + Debug> NestedValue<T> for MutValue<T> {
    //
    //
    fn id(&self) -> String {
        self.id.clone()
    }
    //
    //
    fn init_(&mut self, key: &str) {
        self.id = key.to_owned();
        self.inited = true;
    }
    //
    //
    fn get_(&self, _: &str) -> Result<T, String> {
        Ok(self.get())
    }
    //
    //
    fn store(&mut self, editor: &str, _: &str, value: T) -> Result<(), String> {
        self.store(editor, value)
        // self.register_edit(editor, &value);
        // self.value = value;
        // // self.edited.push(format!("{}. {} - {} ({:?})", self.edited.len() + 1, Utc::now(), editor, value));
        // // println!("{}.store | edited: {:#?}", self.id, self.edited);
        // Ok(())
    }
    //
    //
    fn edited(&self, _: &str) -> Result<Vec<String>, String> {
        Ok(self.edited.clone())
    }
}
//
//
impl<T: Debug> std::fmt::Debug for MutValue<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MutValue").field("id", &self.id).field("value", &self.value).finish()
    }
}
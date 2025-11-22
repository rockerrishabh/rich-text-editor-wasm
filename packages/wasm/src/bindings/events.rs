// Event callback system for JavaScript callbacks

use js_sys::Function;
use wasm_bindgen::prelude::*;

/// Manages event callbacks for document changes and selection changes
pub struct EventCallbacks {
    /// Callbacks triggered when document content changes
    change_callbacks: Vec<Function>,
    /// Callbacks triggered when selection changes
    selection_callbacks: Vec<Function>,
}

impl EventCallbacks {
    /// Creates a new EventCallbacks instance with empty callback vectors
    pub fn new() -> Self {
        Self {
            change_callbacks: Vec::new(),
            selection_callbacks: Vec::new(),
        }
    }

    /// Registers a callback for document change events
    ///
    /// # Arguments
    /// * `callback` - JavaScript function to call when document changes
    pub fn add_change_callback(&mut self, callback: Function) {
        self.change_callbacks.push(callback);
    }

    /// Registers a callback for selection change events
    ///
    /// # Arguments
    /// * `callback` - JavaScript function to call when selection changes
    pub fn add_selection_callback(&mut self, callback: Function) {
        self.selection_callbacks.push(callback);
    }

    /// Removes a specific change callback
    ///
    /// # Arguments
    /// * `callback` - JavaScript function to remove from change callbacks
    ///
    /// # Returns
    /// true if the callback was found and removed, false otherwise
    pub fn remove_change_callback(&mut self, callback: &Function) -> bool {
        // Compare functions by their JsValue representation
        let callback_val: &JsValue = callback.as_ref();
        if let Some(pos) = self.change_callbacks.iter().position(|cb| {
            let cb_val: &JsValue = cb.as_ref();
            cb_val == callback_val
        }) {
            self.change_callbacks.remove(pos);
            true
        } else {
            false
        }
    }

    /// Removes a specific selection callback
    ///
    /// # Arguments
    /// * `callback` - JavaScript function to remove from selection callbacks
    ///
    /// # Returns
    /// true if the callback was found and removed, false otherwise
    pub fn remove_selection_callback(&mut self, callback: &Function) -> bool {
        // Compare functions by their JsValue representation
        let callback_val: &JsValue = callback.as_ref();
        if let Some(pos) = self.selection_callbacks.iter().position(|cb| {
            let cb_val: &JsValue = cb.as_ref();
            cb_val == callback_val
        }) {
            self.selection_callbacks.remove(pos);
            true
        } else {
            false
        }
    }

    /// Triggers all registered change callbacks
    ///
    /// Errors from individual callbacks are caught and silently ignored
    pub fn trigger_change_callbacks(&self) {
        for callback in &self.change_callbacks {
            // Call the callback and ignore any errors
            let _ = callback.call0(&JsValue::NULL);
        }
    }

    /// Triggers all registered selection callbacks
    ///
    /// Errors from individual callbacks are caught and silently ignored
    pub fn trigger_selection_callbacks(&self) {
        for callback in &self.selection_callbacks {
            // Call the callback and ignore any errors
            let _ = callback.call0(&JsValue::NULL);
        }
    }

    /// Clears all registered callbacks
    ///
    /// This method is called during cleanup to release JavaScript function
    /// references and prevent memory leaks. After calling this method,
    /// no callbacks will be triggered.
    ///
    /// # Memory Management
    ///
    /// This is important for proper memory management because JavaScript
    /// function references held by Rust can prevent garbage collection
    /// of JavaScript objects. Clearing callbacks ensures that:
    /// - JavaScript functions can be garbage collected
    /// - Circular references are broken
    /// - Memory leaks are prevented
    pub fn clear_all(&mut self) {
        self.change_callbacks.clear();
        self.selection_callbacks.clear();
    }
}

impl Default for EventCallbacks {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;



    #[wasm_bindgen_test]
    fn test_event_callbacks_creation() {
        let callbacks = EventCallbacks::new();
        assert_eq!(callbacks.change_callbacks.len(), 0);
        assert_eq!(callbacks.selection_callbacks.len(), 0);
    }

    #[wasm_bindgen_test]
    fn test_add_change_callback() {
        let mut callbacks = EventCallbacks::new();
        let func = Function::new_no_args("return 42;");
        callbacks.add_change_callback(func);
        assert_eq!(callbacks.change_callbacks.len(), 1);
    }

    #[wasm_bindgen_test]
    fn test_add_selection_callback() {
        let mut callbacks = EventCallbacks::new();
        let func = Function::new_no_args("return 42;");
        callbacks.add_selection_callback(func);
        assert_eq!(callbacks.selection_callbacks.len(), 1);
    }

    #[wasm_bindgen_test]
    fn test_trigger_change_callbacks() {
        let mut callbacks = EventCallbacks::new();
        let func = Function::new_no_args("return 42;");
        callbacks.add_change_callback(func);
        // Should not panic
        callbacks.trigger_change_callbacks();
    }

    #[wasm_bindgen_test]
    fn test_trigger_selection_callbacks() {
        let mut callbacks = EventCallbacks::new();
        let func = Function::new_no_args("return 42;");
        callbacks.add_selection_callback(func);
        // Should not panic
        callbacks.trigger_selection_callbacks();
    }

    #[wasm_bindgen_test]
    fn test_clear_all_callbacks() {
        let mut callbacks = EventCallbacks::new();
        let func1 = Function::new_no_args("return 1;");
        let func2 = Function::new_no_args("return 2;");
        callbacks.add_change_callback(func1);
        callbacks.add_selection_callback(func2);

        callbacks.clear_all();

        assert_eq!(callbacks.change_callbacks.len(), 0);
        assert_eq!(callbacks.selection_callbacks.len(), 0);
    }
}

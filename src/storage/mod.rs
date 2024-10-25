pub mod memory_storage_provider;

pub trait StoreProvider<T>: Sync + Send {
    fn get_history_mut(&mut self) -> &mut Vec<Option<T>>;
    fn get_history(&self) -> &Vec<Option<T>>;
    fn set_dyn(&mut self, value: Option<T>);

    fn get(&self) -> Option<&T>;

    fn get_mut(&mut self) -> Option<&mut T>;

    fn get_dyn_mut_or_default(&mut self, default: T) -> &mut T {
        if !self.has_value() {
            self.set_dyn(Some(default));
        }
        self.get_mut().unwrap()
    }

    fn has_value(&self) -> bool {
        self.get().is_some()
    }

    fn get_last_value(&self) -> Option<&T> {
        if let Some(v) = self.get_history().last() {
            v.as_ref()
        } else {
            None
        }
    }

    fn tick(&mut self);
}

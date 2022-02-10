use std::{
    cell::{Ref, RefCell, RefMut},
    fmt::{Debug, Formatter},
    rc::Rc,
    sync::Arc,
};

use dioxus::prelude::*;
use dioxus_elements::{div, GlobalAttributes};

use pi_search::{computed, PiBase10, PiComputed};

/// A hook which keeping the context of KaTeX formula.
pub struct UseSearcher {
    computer: Rc<RefCell<PiComputed<PiBase10>>>,
    updater: Arc<dyn Fn() + 'static>,
}

pub fn use_searcher(cx: &ScopeState) -> &mut UseSearcher {
    cx.use_hook(|_| UseSearcher { computer: Rc::new(RefCell::new(computed())), updater: cx.schedule_update() })
}

impl UseSearcher {
    /// Get all config of KaTeX formula.
    pub fn get_state(&self) -> Ref<'_, PiComputed<PiBase10>> {
        self.computer.borrow()
    }
    /// Get the mutable reference of all config.
    pub fn get_state_mut(&self) -> RefMut<'_, PiComputed<PiBase10>> {
        self.computer.borrow_mut()
    }
    /// Notify the scheduler to re-render the component.
    pub fn needs_update(&self) {
        (self.updater)();
    }
}

impl UseSearcher {
    /// Compile the formula to HTML.
    ///
    /// Never fails even if the formula is invalid.
    pub fn search(&self, input: &str) -> LazyNodes {
        let mut config = self.computer.borrow_mut();
        let out = config.search(input.to_string());
        match out {
            Ok(0) => {
                rsx! {
                    div {
                        class: "text-green-500",
                        "Your lucky pi number is in the 1st decimal place"
                    }
                }
            },
            Ok(1) => {
                rsx! {
                    div {
                        class: "text-green-500",
                        "Your lucky pi number is in the 2nd decimal place"
                    }
                }
            },
            Ok(2) => {
                rsx! {
                    div {
                        class: "text-green-500",
                        "Your lucky pi number is in the 3rd decimal place"
                    }
                }
            },
            Ok(o) => {
                let o = o + 1;
                rsx! {
                    div {
                        class: "text-green-500",
                        "Your lucky pi number is in the {o}th decimal place"
                    }
                }
            },
            Err(e) => rsx! {
                div {
                    class: "text-red-500",
                    "{e}"
                }
            },
        }
    }
}

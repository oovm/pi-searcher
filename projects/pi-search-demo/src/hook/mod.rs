use std::{
    cell::{Ref, RefCell, RefMut},
    fmt::{Debug, Formatter},
    rc::Rc,
};
use std::sync::Arc;

use dioxus::prelude::*;
use dioxus_elements::{div, GlobalAttributes};

use pi_search::{computed, PiBase10, PiComputed};

/// A hook which keeping the context of KaTeX formula.
pub struct UseSearcher {
    computer: Rc<RefCell<PiComputed<PiBase10>>>,
    updater: Arc<dyn Fn() + 'static>,
}

pub fn use_searcher(cx: &ScopeState) -> &mut UseSearcher {
    cx.use_hook(|| {
        Self {
            computer: Rc::new(RefCell::new(computed())),
            updater: cx.schedule_update(),
        }
    })
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
            Ok(o) => {
                rsx! {
                    div {
                        "{o}"
                    }
                }
            }
            Err(e) => {
                rsx! {
                    div {
                        "{o}"
                    }
                }
            }
        }
        LazyNodes::new(move |cx: NodeFactory| -> VNode {
            cx.element(
                div,
                &[],
                cx.bump().alloc([div.dangerous_inner_html(cx, format_args!("{out}", out = out))]),
                &[],
                None,
            )
        })
    }
}

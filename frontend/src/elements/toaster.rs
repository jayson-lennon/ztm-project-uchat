#![allow(non_snake_case)]

use std::collections::HashMap;

use chrono::{DateTime, Duration, Utc};
use dioxus::prelude::*;
use fermi::{use_atom_ref, UseAtomRef};

pub fn use_toaster(cx: &ScopeState) -> &UseAtomRef<Toaster> {
    use_atom_ref(cx, crate::app::TOASTER)
}

pub enum ToastKind {
    Error,
    Info,
    Success,
}

pub struct Toast {
    pub message: String,
    pub expires: DateTime<Utc>,
    pub kind: ToastKind,
}

#[derive(Default)]
pub struct Toaster {
    toasts: HashMap<usize, Toast>,
    next_id: usize,
}

impl Toaster {
    fn increment_id(&mut self) {
        self.next_id += 1;
    }

    pub fn push(&mut self, toast: Toast) {
        self.toasts.insert(self.next_id, toast);
        self.increment_id();
    }

    pub fn remove(&mut self, id: usize) {
        self.toasts.remove(&id);
    }

    pub fn success<T: Into<String>>(&mut self, message: T, duration: Duration) {
        let toast = Toast {
            message: message.into(),
            expires: Utc::now() + duration,
            kind: ToastKind::Success,
        };
        self.push(toast);
    }

    pub fn info<T: Into<String>>(&mut self, message: T, duration: Duration) {
        let toast = Toast {
            message: message.into(),
            expires: Utc::now() + duration,
            kind: ToastKind::Info,
        };
        self.push(toast);
    }

    pub fn error<T: Into<String>>(&mut self, message: T, duration: Duration) {
        let toast = Toast {
            message: message.into(),
            expires: Utc::now() + duration,
            kind: ToastKind::Error,
        };
        self.push(toast);
    }

    pub fn iter(&self) -> std::collections::hash_map::Iter<'_, usize, Toast> {
        self.toasts.iter()
    }
}

#[derive(Props)]
pub struct ToastRootProps<'a> {
    toaster: &'a UseAtomRef<Toaster>,
}

pub fn ToastRoot<'a>(cx: Scope<'a, ToastRootProps<'a>>) -> Element {
    todo!()
}

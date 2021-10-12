use crate::replace_ident::replace_ident_from_self_to_state;

pub struct Inputs {
    /// The `self`, or `&mut self`, or `&self` part.
    pub receiver: Option<syn::Receiver>,

    pub args: Vec<syn::PatType>,
}

impl Inputs {
    pub fn replace_from_self_to_state<'a>(inputs: impl Iterator<Item = &'a syn::FnArg>) -> Self {
        let mut receiver = None;
        let mut args = Vec::new();
        for arg in inputs {
            match arg {
                syn::FnArg::Receiver(r) => {
                    assert!(receiver.is_none());
                    receiver = Some(r.clone())
                }
                syn::FnArg::Typed(pty) => {
                    let mut pty = pty.clone();
                    replace_ident_from_self_to_state(&mut pty);
                    args.push(pty);
                }
            }
        }

        Self { receiver, args }
    }
}

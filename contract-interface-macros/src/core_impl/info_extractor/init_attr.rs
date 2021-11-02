use darling::FromMeta;

#[derive(Debug, FromMeta)]
pub struct InitAttr {
    #[darling(default)]
    pub ignore_state: Option<bool>,
}

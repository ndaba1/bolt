#[allow(dead_code)]
pub struct PromptConfig {
    ptype: PromptType,
    msg: String,
    when: bool,
}

pub enum PromptType {
    Confirm,
    List,
    Checkbox,
    Input,
}

pub fn prompt() {}

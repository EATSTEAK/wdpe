use crate::{
    command::WebDynproCommand,
    element::{action::ButtonDef, parser::ElementParser},
    error::WebDynproError,
    event::Event,
};

/// 주어진 [`Button`](crate::element::action::Button)을 누르는 이벤트를 반환
pub struct ButtonPressEventCommand {
    element_def: ButtonDef,
}

impl ButtonPressEventCommand {
    /// 새로운 명령 객체를 생성합니다.
    pub fn new(element_def: ButtonDef) -> Self {
        Self { element_def }
    }
}

impl WebDynproCommand for ButtonPressEventCommand {
    type Result = Event;

    fn dispatch(&self, parser: &ElementParser) -> Result<Self::Result, WebDynproError> {
        parser.element_from_def(&self.element_def)?.press()
    }
}

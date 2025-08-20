use crate::{
    error::ClientError,
    event::{EventBuilder, EventBuilderError, ucf_parameters::UcfParameters},
};

use super::{EVENT_SPECTATOR, Event};
use std::collections::{HashMap, LinkedList};

/// 이벤트가 추가된 후, [`EventQueue`]가 처리되어야 할 지를 나타내는 enum입니다.
pub enum EnqueueEventResult {
    /// [`EventQueue`]가 처리되어야 함을 나타냅니다. [`EventQueue::serialize_and_clear()`] 함수로 큐를 처리할 수 있습니다.
    ShouldProcess,
    /// [`EventQueue`]에 이벤트가 추가되었고, 별도의 작업이 필요하지 않음을 나타냅니다.
    Enqueued,
}

#[derive(Debug)]
pub struct EventQueue {
    queue: LinkedList<Event>,
    should_process: bool,
}

impl EventQueue {
    pub fn new() -> EventQueue {
        EventQueue {
            queue: LinkedList::new(),
            should_process: false,
        }
    }

    pub fn serialize_and_clear_with_form_event(&mut self) -> Result<String, ClientError> {
        let form_req = create_form_request_event(false, "", "", false, false).or(Err(
            ClientError::NoSuchForm("sap.client.SsrClient.form".to_string()),
        ))?;
        self.add(form_req.to_owned());
        Ok(self.serialize_and_clear())
    }

    pub fn serialize_and_clear(&mut self) -> String {
        let mut owned = "".to_owned();
        let events = &self.queue;
        for (idx, event) in events.iter().enumerate() {
            owned.push_str(&event.serialize());
            if idx < events.len() - 1 {
                owned.push_str(EVENT_SPECTATOR);
            }
        }
        let _ = &self.queue.clear();
        owned
    }

    pub fn add(&mut self, evt: Event) -> EnqueueEventResult {
        if !evt.is_enqueable() && evt.is_submitable() {
            self.should_process = true;
        }
        self.queue.push_back(evt);
        if self.should_process {
            EnqueueEventResult::ShouldProcess
        } else {
            EnqueueEventResult::Enqueued
        }
    }

    #[allow(unused)]
    pub fn remove(&mut self) -> Option<Event> {
        self.queue.pop_front()
    }
}

fn create_form_request_event(
    is_async: bool,
    focus_info: &str,
    hash: &str,
    dom_changed: bool,
    is_dirty: bool,
) -> Result<Event, EventBuilderError> {
    let mut form_parameters: HashMap<String, String> = HashMap::new();
    form_parameters.insert("Id".to_string(), "sap.client.SsrClient.form".to_string());
    form_parameters.insert("Async".to_string(), is_async.to_string());
    form_parameters.insert("FocusInfo".to_string(), focus_info.to_string());
    form_parameters.insert("Hash".to_string(), hash.to_string());
    form_parameters.insert("DomChanged".to_string(), dom_changed.to_string());
    form_parameters.insert("IsDirty".to_string(), is_dirty.to_string());
    EventBuilder::default()
        .control("Form".to_string())
        .event("Request".to_string())
        .parameters(form_parameters)
        .ucf_parameters(UcfParameters::default())
        .custom_parameters(HashMap::new())
        .build()
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use crate::event::{
        EventBuilder,
        event_queue::EventQueue,
        ucf_parameters::{UcfAction, UcfParametersBuilder, UcfResponseData},
    };

    #[test]
    fn event_queue_serialize() {
        let mut parameters = HashMap::new();
        parameters.insert("Id".to_string(), "WD0213".to_string());
        let ucf_params = UcfParametersBuilder::default()
            .response(Some(UcfResponseData::Delta))
            .action(Some(UcfAction::Submit))
            .build()
            .unwrap();
        let event = EventBuilder::default()
            .control("Button".to_owned())
            .event("Press".to_owned())
            .parameters(parameters)
            .ucf_parameters(ucf_params)
            .build()
            .unwrap();
        let mut parameters_two = HashMap::new();
        parameters_two.insert("Id".to_string(), "sap.client.SsrClient.form".to_string());
        parameters_two.insert("Async".to_string(), "false".to_string());
        parameters_two.insert(
            "FocusInfo".to_string(),
            "@{\"sFocussedId\":\"WD0213\"}".to_string(),
        );
        parameters_two.insert("Hash".to_string(), "".to_string());
        parameters_two.insert("DomChanged".to_string(), "false".to_string());
        parameters_two.insert("IsDirty".to_string(), "false".to_string());
        let ucf_params_two = UcfParametersBuilder::default()
            .response(Some(UcfResponseData::Delta))
            .build()
            .unwrap();
        let event_two = EventBuilder::default()
            .control("Form".to_owned())
            .event("Request".to_owned())
            .parameters(parameters_two)
            .ucf_parameters(ucf_params_two)
            .build()
            .unwrap();
        let mut queue = EventQueue::new();
        queue.add(event);
        queue.add(event_two);
        assert_eq!(queue.serialize_and_clear().len(), "Button_Press~E002Id~E004WD0213~E003~E002ClientAction~E004submit~E005ResponseData~E004delta~E003~E002~E003~E001Form_Request~E002FocusInfo~E004~0040~007B~0022sFocussedId~0022~003A~0022WD0213~0022~007D~E005Id~E004sap.client.SsrClient.form~E005Async~E004false~E005Hash~E004~E005IsDirty~E004false~E005DomChanged~E004false~E003~E002ResponseData~E004delta~E003~E002~E003".len());
    }
}

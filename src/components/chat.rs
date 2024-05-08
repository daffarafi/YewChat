use serde::{Deserialize, Serialize};
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_agent::{Bridge, Bridged};

use crate::services::event_bus::EventBus;
use crate::{services::websocket::WebsocketService, User};

pub enum Msg {
    HandleMsg(String),
    SubmitMessage,
}

#[derive(Deserialize)]
struct MessageData {
    from: String,
    message: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum MsgTypes {
    Users,
    Register,
    Message,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct WebSocketMessage {
    message_type: MsgTypes,
    data_array: Option<Vec<String>>,
    data: Option<String>,
}

#[derive(Clone)]
struct UserProfile {
    name: String,
    avatar: String,
}

pub struct Chat {
    users: Vec<UserProfile>,
    chat_input: NodeRef,
    _producer: Box<dyn Bridge<EventBus>>,
    wss: WebsocketService,
    messages: Vec<MessageData>,
}
impl Component for Chat {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let (user, _) = ctx
            .link()
            .context::<User>(Callback::noop())
            .expect("context to be set");
        let wss = WebsocketService::new();
        let username = user.username.borrow().clone();

        let message = WebSocketMessage {
            message_type: MsgTypes::Register,
            data: Some(username.to_string()),
            data_array: None,
        };

        if let Ok(_) = wss
            .tx
            .clone()
            .try_send(serde_json::to_string(&message).unwrap())
        {
            log::debug!("message sent successfully");
        }

        Self {
            users: vec![],
            messages: vec![],
            chat_input: NodeRef::default(),
            wss,
            _producer: EventBus::bridge(ctx.link().callback(Msg::HandleMsg)),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::HandleMsg(s) => {
                let msg: WebSocketMessage = serde_json::from_str(&s).unwrap();
                match msg.message_type {
                    MsgTypes::Users => {
                        let users_from_message = msg.data_array.unwrap_or_default();
                        self.users = users_from_message
                            .iter()
                            .map(|u| UserProfile {
                                name: u.into(),
                                avatar: format!(
                                    "https://avatars.dicebear.com/api/adventurer-neutral/{}.svg",
                                    u
                                )
                                .into(),
                            })
                            .collect();
                        return true;
                    }
                    MsgTypes::Message => {
                        let message_data: MessageData =
                            serde_json::from_str(&msg.data.unwrap()).unwrap();
                        self.messages.push(message_data);
                        return true;
                    }
                    _ => {
                        return false;
                    }
                }
            }
            Msg::SubmitMessage => {
                let input = self.chat_input.cast::<HtmlInputElement>();
                if let Some(input) = input {
                    let message = WebSocketMessage {
                        message_type: MsgTypes::Message,
                        data: Some(input.value()),
                        data_array: None,
                    };
                    if let Err(e) = self
                        .wss
                        .tx
                        .clone()
                        .try_send(serde_json::to_string(&message).unwrap())
                    {
                        log::debug!("error sending to channel: {:?}", e);
                    }
                    input.set_value("");
                };
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let submit = ctx.link().callback(|_| Msg::SubmitMessage);

        html! {
          <div class="flex w-full min-h-screen bg-gray-200">
              <div class="w-60 h-full bg-gray-800 text-white flex flex-col">
                  <div class="text-2xl font-bold p-4 border-b border-gray-700">{"Users"}</div>
                  <div class="flex flex-col overflow-auto px-4 py-6">
                      {
                          self.users.iter().map(|user| {
                              html! {
                                  <div class="flex items-center p-4 hover:bg-gray-700 rounded-lg transition-colors duration-200 ease-in-out">
                                      <img src={user.avatar.clone()} class="w-10 h-10 rounded-full" alt="User Avatar"/>
                                      <div class="ml-4">
                                          <p class="text-lg font-semibold">{user.name.clone()}</p>
                                          <p class="text-sm text-gray-400">{"Online"}</p>
                                      </div>
                                  </div>
                              }
                          }).collect::<Html>()
                      }
                  </div>
              </div>
              
              <div class="flex flex-col flex-grow">
                  <div class="h-16 flex items-center bg-white border-b border-gray-300 px-6">
                      <h1 class="text-2xl font-semibold">{"💬 Chat Room"}</h1>
                  </div>
          
                  <div class="flex-grow overflow-auto p-6 bg-white">
                      {
                          self.messages.iter().map(|message| {
                              let user = self.users.iter().find(|u| u.name == message.from).unwrap();
                              html! {
                                  <div class="flex items-start mb-6">
                                      <img src={user.avatar.clone()} class="w-10 h-10 rounded-full" alt="User Avatar"/>
                                      <div class="ml-4 p-4 bg-gray-100 rounded-lg">
                                          <p class="text-sm font-semibold">{message.from.clone()}</p>
                                          <p class="text-base text-gray-700">
                                              if message.message.ends_with(".gif") {
                                                  <img src={message.message.clone()} class="w-20 h-20 rounded-lg"/>
                                              } else {
                                                  {message.message.clone()}
                                              }
                                          </p>
                                      </div>
                                  </div>
                              }
                          }).collect::<Html>()
                      }
                  </div>
                  
                  <div class="h-16 flex items-center bg-white border-t border-gray-300 px-6">
                      <input 
                          ref={self.chat_input.clone()} 
                          type="text" 
                          placeholder="Type your message..." 
                          class="w-full bg-gray-100 border rounded-full px-4 py-3 focus:outline-none focus:ring focus:border-blue-500 transition duration-200 ease-in-out"
                      />
                      <button 
                          onclick={submit} 
                          class="ml-4 bg-blue-600 text-white w-12 h-12 rounded-full flex items-center justify-center hover:bg-blue-700 transition-colors duration-200"
                      >
                          <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" class="w-6 h-6">
                              <path d="M0 0h24v24H0z" fill="none"/><path d="M2.01 21L23 12 2.01 3 2 10l15 2-15 2z"/>
                          </svg>
                      </button>
                  </div>
              </div>
          </div>
      
        }
    }
}
use web_sys::HtmlInputElement;
use yew::functional::*;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::Route;
use crate::User;

#[function_component(Login)]
pub fn login() -> Html {
    let username = use_state(|| String::new());
    let user = use_context::<User>().expect("No context found.");

    let oninput = {
        let current_username = username.clone();

        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            current_username.set(input.value());
        })
    };

    let onclick = {
        let username = username.clone();
        let user = user.clone();
        Callback::from(move |_| *user.username.borrow_mut() = (*username).clone())
    };

    html! {
      <div class="bg-gray-900 flex w-full min-h-screen items-center justify-center">
          <div class="container mx-auto flex flex-col items-center justify-center">
              <h1 class="text-3xl font-semibold text-white mb-6">{"Welcome to Chat App"}</h1>
              
              <form class="w-full max-w-sm flex items-center justify-center">
                  <input 
                      {oninput} 
                      class="flex-1 rounded-l-full px-5 py-3 border-t border-b border-l text-gray-800 bg-white focus:outline-none focus:ring focus:border-blue-500 transition ease-in-out duration-200" 
                      placeholder="Enter your username" 
                  />
                  <Link<Route> to={Route::Chat}>
                      <button 
                          {onclick} 
                          disabled={username.len() < 1} 
                          class="px-6 py-1 rounded-r-full bg-violet-700 text-white font-bold uppercase border-t border-b border-r border-violet-700 transition-colors duration-200 hover:bg-violet-600 disabled:opacity-50"
                      >
                          {"Go Chatting!"}
                      </button>
                  </Link<Route>>
              </form>
          </div>
      </div>
    }
}
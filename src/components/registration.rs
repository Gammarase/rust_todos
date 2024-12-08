use dioxus::prelude::*;
use dioxus_router::hooks::use_navigator;

use crate::{components::auth_state::AuthState, datasources::datarepository::Datasource};

#[component]
pub fn Registration(cx: Scope) -> Element {
    let nav = use_navigator(cx);
    let auth_state = use_shared_state::<AuthState>(cx)?;
    
    // State for input fields
    let username = use_state(cx, || String::new());
    let password = use_state(cx, || String::new());
    let repeat_password = use_state(cx, || String::new());
    
    // Error message state
    let error_message = use_state(cx, || String::new());

    cx.render(rsx! {
        div {
            h1 { "Registration" }
            
            input {
                placeholder: "Username",
                oninput: move |evt| username.set(evt.value.clone()),
            }
            
            input {
                r#type: "password",
                placeholder: "Password",
                oninput: move |evt| password.set(evt.value.clone()),
            }
            
            input {
                r#type: "password",
                placeholder: "Repeat Password",
                oninput: move |evt| repeat_password.set(evt.value.clone()),
            }
            
            if !error_message.is_empty() {
                rsx!{
                div {
                    style: "color: red;",
                    "{error_message}"
                }
            }
            }
            
            button {
                onclick: move |_| {
                    if username.get().is_empty() {
                        error_message.set("Username cannot be empty".to_string());
                        return;
                    }
                    
                    if password.get().is_empty() {
                        error_message.set("Password cannot be empty".to_string());
                        return;
                    }
                    
                    if password.get() != repeat_password.get() {
                        error_message.set("Passwords do not match".to_string());
                        return;
                    }
                    
                    match Datasource::new() {
                        Ok(datasource) => {
                            
                            match datasource.register_user(username.get(), password.get()) {
                                Ok(user_id) => {
                                    auth_state.write().user_id = Some(user_id);
                                    auth_state.write().username = Some(username.get().clone());
                                    nav.push("/");
                                },
                                Err(e) => {
                                    error_message.set(format!("Registration failed: {}", e));
                                }
                            }
                        },
                        Err(_) => {
                            error_message.set("Could not connect to database".to_string());
                        }
                    }
                },
                "Register"
            }
    button{
        onclick: move |_| {
            nav.push("/");},
        "Go Back"
    }
        }
    })
}


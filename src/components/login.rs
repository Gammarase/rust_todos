use dioxus::prelude::*;
use dioxus_router::hooks::use_navigator;
use crate::components::auth_state::AuthState;
use crate::datasources::datarepository::Datasource;

#[component]
pub fn Login(cx: Scope) -> Element {
    let nav = use_navigator(cx);
    let auth_state = use_shared_state::<AuthState>(cx)?;

    // State for input fields
    let username = use_state(cx, || String::new());
    let password = use_state(cx, || String::new());

    // Error message state
    let error_message = use_state(cx, || String::new());

    cx.render(rsx! {
        div {
            h1 { "Login" }

            input {
                placeholder: "Username",
                // value: username.get(),
                oninput: move |evt| username.set(evt.value.clone()),
            }

            input {
                r#type: "password",
                placeholder: "Password",
                // value: password.get(),
                oninput: move |evt| password.set(evt.value.clone()),
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
                    match Datasource::new() {
                        Ok(datasource) => {
                            match datasource.login(username.get(), password.get()) {
                                Ok(Some(user)) => {
                                    auth_state.write().user_id = user.id;
                                    auth_state.write().username = Some(user.name.clone());
                                    nav.push("/todos");
                                },
                                Ok(_none) => {
                                    error_message.set("Invalid username or password".to_string());
                                },
                                Err(_) => {
                                    error_message.set("An error occurred during login".to_string());
                                }
                            }
                        },
                        Err(_) => {
                            error_message.set("Could not connect to database".to_string());
                        }
                    }
                },
                "Login"
            }

            button {
                onclick: move |_| {
                    nav.push("/registration");
                },
                "Register"
            }
        }
    })
}

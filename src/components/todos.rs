use dioxus::prelude::*;
use dioxus_router::hooks::use_navigator;
use crate::{components::auth_state::AuthState,datasources::datarepository::Todo,  datasources::datarepository::Datasource};

#[component]
pub fn Todos(cx: Scope) -> Element {
    let auth_state = use_shared_state::<AuthState>(cx)?;
    
    
    if auth_state.read().user_id.is_none() {
        let nav = use_navigator(cx);
        nav.push("/");
        return None;
    }

    
    let todos = use_state(cx, || Vec::<Todo>::new());
    let new_todo_name = use_state(cx, || String::new());
    let new_todo_description = use_state(cx, || String::new());
    let editing_todo = use_state(cx, || Option::<Todo>::None);
    
    
    use_effect(cx, (&auth_state.read().user_id,), |(user_id,)| {
        to_owned![todos, user_id];
        async move {
            if let Some(id) = user_id {
                match Datasource::new() {
                    Ok(datasource) => {
                        match datasource.get_user_todos(id) {
                            Ok(user_todos) => {
                                todos.set(user_todos);
                            },
                            Err(e) => {
                                println!("Error fetching todos: {:?}", e);
                            }
                        }
                    },
                    Err(e) => {
                        println!("Error creating datasource: {:?}", e);
                    }
                }
            }
        }
    });

    
    let create_todo = move |_| {
        if let Some(user_id) = auth_state.read().user_id {
            if !new_todo_name.get().is_empty() {
                match Datasource::new() {
                    Ok(datasource) => {
                        let new_todo = Todo {
                            id: None,
                            user_id,
                            name: new_todo_name.get().clone(),
                            description: Some(new_todo_description.get().clone()),
                            status: "PENDING".to_string(),
                            deadline: None,
                        };

                        match datasource.create_todo(&new_todo) {
                            Ok(_) => {
                                
                                match datasource.get_user_todos(user_id) {
                                    Ok(updated_todos) => {
                                        todos.set(updated_todos);
                                        new_todo_name.set(String::new());
                                        new_todo_description.set(String::new());
                                    },
                                    Err(e) => println!("Error refreshing todos: {:?}", e),
                                }
                            },
                            Err(e) => println!("Error creating todo: {:?}", e),
                        }
                    },
                    Err(e) => println!("Error creating datasource: {:?}", e),
                }
            }
        }
    };

    
    let delete_todo = move |todo_id: i64| {
        match Datasource::new() {
            Ok(datasource) => {
                match datasource.delete_todo(todo_id) {
                    Ok(_) => {
                        
                        if let Some(user_id) = auth_state.read().user_id {
                            match datasource.get_user_todos(user_id) {
                                Ok(updated_todos) => {
                                    todos.set(updated_todos);
                                },
                                Err(e) => println!("Error refreshing todos: {:?}", e),
                            }
                        }
                    },
                    Err(e) => println!("Error deleting todo: {:?}", e),
                }
            },
            Err(e) => println!("Error creating datasource: {:?}", e),
        }
    };

    
    let update_todo = move |updated_todo: Todo| {
        match Datasource::new() {
            Ok(datasource) => {
                match datasource.update_todo(&updated_todo) {
                    Ok(_) => {
                        
                        if let Some(user_id) = auth_state.read().user_id {
                            match datasource.get_user_todos(user_id) {
                                Ok(updated_todos) => {
                                    todos.set(updated_todos);
                                    editing_todo.set(None);
                                },
                                Err(e) => println!("Error refreshing todos: {:?}", e),
                            }
                        }
                    },
                    Err(e) => println!("Error updating todo: {:?}", e),
                }
            },
            Err(e) => println!("Error creating datasource: {:?}", e),
        }
    };

    cx.render(rsx! {
        div {
            h1 { "Todos" }
            
            
            div {
                input {
                    placeholder: "Todo Name",
                    oninput: move |evt| new_todo_name.set(evt.value.clone()),
                }
                input {
                    placeholder: "Description (Optional)",
                    oninput: move |evt| new_todo_description.set(evt.value.clone()),
                }
                button {
                    onclick: create_todo,
                    "Add Todo"
                }
            }
            
            
            ul {
                {todos.iter().map(|todo| {
                    
                    let is_editing = editing_todo.as_ref().map_or(false, |et| et.id == todo.id);
                    
                    rsx! {
                        li {
                            key: "{todo.id.unwrap_or(0)}",
                            if is_editing {
                                rsx!{
                                    input {
                                    value: "{editing_todo.as_ref().unwrap().name}",
                                    oninput: move |evt| {
                                        if let Some(mut edit_todo) = editing_todo.as_ref().cloned() {
                                            edit_todo.name = evt.value.clone();
                                            editing_todo.set(Some(edit_todo));
                                        }
                                    }
                                }
                                    input {
                                    value: "{editing_todo.as_ref().and_then(|et| et.description.clone()).unwrap_or(String::new())}",
                                    oninput: move |evt| {
                                        if let Some(mut edit_todo) = editing_todo.as_ref().cloned() {
                                            edit_todo.description = Some(evt.value.clone());
                                            editing_todo.set(Some(edit_todo));
                                        }
                                    }
                                }
                                button {
                                    onclick: move |_| {
                                        if let Some(edit_todo) = editing_todo.as_ref().cloned() {
                                            update_todo(edit_todo);
                                        }
                                    },
                                    "Save"
                                }
                                button {
                                    onclick: move |_| editing_todo.set(None),
                                    "Cancel"
                                }
                                }
                            } else {
                                rsx!{
                                    input {
                                    r#type: "checkbox",
                                    checked: todo.status == "COMPLETED",
                                    onchange: move |_| {
                                        let mut updated_todo = todo.clone();
                                        updated_todo.status = if todo.status == "COMPLETED" {
                                            "PENDING".to_string()
                                        } else {
                                            "COMPLETED".to_string()
                                        };
                                        update_todo(updated_todo);
                                    }
                                }
                                span { "{todo.name} -- " }
                                match &todo.description {
                                    Some(description) => {
                                        rsx!{span { "{description}" }}
                                    }
                                    None => {rsx!{span { "" }}}
                                }
                                button {
                                    onclick: move |_| {
                                        editing_todo.set(Some(todo.clone()));
                                    },
                                    "Edit"
                                }
                                button {
                                    onclick: move |_| {
                                        if let Some(id) = todo.id {
                                            delete_todo(id);
                                        }
                                    },
                                    "Delete"
                                }
                                }
                            }
                        }
                    }
                })}
            }
        }
    })
}

// #![allow(non_snake_case)]

mod pages;
mod components;
mod store;
mod middleware;
mod app;

use components::sidebar_item::SidebarItem;
use dioxus::prelude::*;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(Navbar)]
    #[route("/")]
    Home {},
    #[route("/blog/:id")]
    Blog { id: i32 },
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const HEADER_SVG: Asset = asset!("/assets/header.svg");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS } document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        Router::<Route> {}
    }
}

#[component]
pub fn Hero() -> Element {
    rsx! {
        div {
            id: "hero",
            div { id: "links",
                a { href: "https://dioxuslabs.com/learn/0.6/", "📚 Learn Dioxus" }
                a { href: "https://dioxuslabs.com/awesome", "🚀 Awesome Dioxus" }
                a { href: "https://github.com/dioxus-community/", "📡 Community Libraries" }
                a { href: "https://github.com/DioxusLabs/sdk", "⚙️ Dioxus Development Kit" }
                a { href: "https://marketplace.visualstudio.com/items?itemName=DioxusLabs.dioxus", "💫 VSCode Extension" }
                a { href: "https://discord.gg/XgGxMSkvUM", "👋 Community Discord" }
            }
        }
    }
}

/// Home page
#[component]
fn Home() -> Element {
    rsx! {
        Hero {}

    }
}

/// Blog page
#[component]
pub fn Blog(id: i32) -> Element {
    rsx! {
        div {
            id: "blog",

            // Content
            h1 { "This is blog #{id}!" }
            p { "In blog #{id}, we show how the Dioxus router works and how URL parameters can be passed as props to our route components." }

            // Navigation links
            Link {
                to: Route::Blog { id: id - 1 },
                "Previous"
            }
            span { " <---> " }
            Link {
                to: Route::Blog { id: id + 1 },
                "Next"
            }
        }
    }
}


#[component]
fn Navbar() -> Element {
    let mut is_collapsed= use_signal(|| false);
    rsx! {
        div {
            class: "flex h-screen",
           
           // Sidebar
            aside {
                class: format_args!("bg-gray-100 h-full p-4 flex flex-col z-50 transiton-all duration-300 {}",
                    if is_collapsed(){ "w-16"} else {"w-64"}
                ),
                //Side header
                div {  
                    class: "flex items-center justify-between mb-4",
                    //logo
                    div {
                        class: format_args!(
                            "transition-all duration-300 overflow-hidden whitespace-nowrap text-xl font-bold text-gray-800 {}",
                            if is_collapsed(){
                                "max-w-0 opacity-0"
                            }else{
                                "max-w-xs opacity-100"
                            }
                        ),
                        " My App"
                    }

                    button {  
                        class: "mb-4 text-gray-600 hover:text-black ",
                        onclick: move |_| is_collapsed.set(!is_collapsed()),
                        if is_collapsed(){"1"} else {"2"} 
                    }
                }

                //Menu items
                nav { 
                    class: "space-y-4 flex-1",
                    SidebarItem {
                        icon: "🏠",
                        label: "Home",
                        to: Route::Home {},
                        collapsed: is_collapsed(),
                    }

                    SidebarItem {
                        icon: "📝", 
                        label: "Blog",
                        to: Route::Blog { id: 1},
                        collapsed: is_collapsed(),
                    }
                }
                
            }
            
            // Content
            div {  
                class: "flex-1 flex flex-col",

                // Header
                header {
                    class: "bg-gray-800 text-white px-4 py-3 flex justify-between items-center",
                    h1 { 
                        class: "text-xl font-bold",
                        "🌐 My Dioxus App" 
                    }

                    button {   
                        class: "bg-gray-600 hover:bg-gray-500 text-white px-3 py-1 rounded",
                        "Logout"
                    }
                }
                // Main content
                main {
                    class: "flex-1 bg-gray-50 p-6 overflow-y-auto",
                    // Đây là nơi các trang con (Home, Blog) sẽ hiển thị
                    Outlet::<Route> {}
                }
    
                footer {
                    class: "bg-gray-800 text-white p-4 text-center",
                    p { "© 2025 This Dioxus App is created  by HSPM. All rights reserved." }
                }
            }
        }
    }
}


use dioxus::prelude::{component, rsx, Props};
use dioxus_core::Element;
use dioxus::prelude::*;

use crate::Route;


#[derive(Props, PartialEq, Clone)]
pub struct SidebarItemProps {
    icon: &'static str,
    label: &'static str,
    to: Route,
    collapsed: bool,
}


#[component]
pub fn SidebarItem(props: SidebarItemProps)-> Element{
    rsx! {
        Link { 
            to: props.to.clone(),
            class: "flex items-center space-x-2 text-gray-700 hover:text-black transition-all",
            span { "Icon 1"},
            span {
                class: format_args!(
                    "transition-all duration-300 overflow-hidden whitespace-nowrap
                     {}",
                    if props.collapsed {
                        "max-w-0 opacity-0"
                    } else {
                        "max-w-xs opacity-100"
                    }
                ),
                {props.label}
            }
        }
    }
}
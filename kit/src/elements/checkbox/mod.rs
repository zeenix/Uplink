use common::icons::outline::Shape as Icon;
use common::icons::Icon as IconElement;
use dioxus::prelude::*;

#[derive(Props)]
pub struct Props<'a> {
    disabled: bool,
    width: Option<String>,
    height: Option<String>,
    // if the checkbox is in a row and it is desired that clicking the row
    // triggers the click event, this hook lets that happen.
    // please don't create the hook on the fly. Creating Elements, which define a single hook, on the fly is OK.
    is_checked: bool,
    // returns true if the box is selected, false otherwise
    on_click: EventHandler<'a, ()>,
}

#[allow(non_snake_case)]
pub fn Checkbox<'a>(cx: Scope<'a, Props<'a>>) -> Element<'a> {
    let disabled_class = if cx.props.disabled { "disabled" } else { "" };
    let checked_class = if cx.props.is_checked { "checked" } else { "" };

    let height = cx
        .props
        .height
        .clone()
        .unwrap_or_else(|| "fit-content".into());
    let width = cx
        .props
        .width
        .clone()
        .unwrap_or_else(|| "fit-content".into());

    cx.render(rsx!(
            div {
            class: "input-checkbox {checked_class} {disabled_class} ",
            height: "{height}",
            width: "{width}",
            onclick: move |_| {
                cx.props.on_click.call(());
            },
            cx.props.is_checked.then(|| {
                rsx!(
                    IconElement {
                        icon: Icon::Check
                    }
                )
            }),
        }
    ))
}

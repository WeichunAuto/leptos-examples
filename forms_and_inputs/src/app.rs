use leptos::ev::SubmitEvent;
use leptos::html;
use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Stylesheet};

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    let (name, set_name) = signal("Controlled".to_string());
    let email = RwSignal::new("".to_string());
    let spam_me = RwSignal::new(true);
    let favorite_color = RwSignal::new("red".to_string());

    let (uname, set_uname) = signal("Uncontrolled".to_string());
    let input_element: NodeRef<html::Input> = NodeRef::new();

    let (textarea_value, set_textarea_value) = signal("textare init value".to_string());
    let (select_value, set_select_value) = signal("1".to_string());

    let on_submit = move |ev: SubmitEvent| {
        // stop the page from reloading!
        ev.prevent_default();

        // here, we'll extract the value from the input
        let value = input_element
            .get()
            // event handlers can only fire after the view
            // is mounted to the DOM, so the `NodeRef` will be `Some`
            .expect("<input> should be mounted")
            // `leptos::HtmlElement<html::Input>` implements `Deref`
            // to a `web_sys::HtmlInputElement`.
            // this means we can call`HtmlInputElement::value()`
            // to get the current value of the input
            .value();
        set_uname.set(value);
    };

    view! {

        <Stylesheet id="leptos" href="/pkg/forms_and_inputs.css"/>

        <h1>"受控组件"</h1>
        <label>
            "Your Name: "
            <input type="text" bind:value=(name, set_name)/>
        </label>

        <label>
            " Email: "
            <input type="email" bind:value=email/>
        </label>

        <label>
            " Please send me lots of spam email. "
            <input type="checkbox" bind:checked=spam_me/>
        </label>

        <br />
        <fieldset style="border: 12px solid green; padding: 10px;">
            <legend>"Favorite color"</legend>
            <label>
                "Red"
                <input
                    type="radio"
                    name="color"
                    value="red"
                    bind:group=favorite_color
                />
            </label>
            <label>
                "Green"
                <input
                    type="radio"
                    name="color"
                    value="green"
                    bind:group=favorite_color
                />
            </label>
            <label>
                "Blue"
                <input
                    type="radio"
                    name="color"
                    value="blue"
                    bind:group=favorite_color
                />
            </label>
        </fieldset>

        <p>"Your Favorite color is "<span style="color: red;">{favorite_color}</span></p>
        <p>"Your name is " <span style="color:red;">{move || name.get()}</span></p> // 也可以直接 name, 不写闭包
        <p>"Your email is " <span style="color:red;">{email}</span></p>

        // The <Show/> component in Leptos is a control-flow component used for conditionally rendering content based on a boolean condition. It helps avoid unnecessary re-renders of expensive components by managing the reactive state internally.
        <Show
            when=move || spam_me.get()
            // `fallback` content to show if `when` is false
            fallback=|| view! { <p>"spame_me value is false."</p> }
        >
            <p style="color:blue;">"You’ll receive cool bonus content!"</p>
        </Show>


        // Our on_submit handler will access the input’s value and use it to call set_name.set().
        // To access the DOM node stored in the NodeRef, we can simply call it as a function (or using .get()).
        // This will return Option<leptos::HtmlElement<html::Input>>,
        //but we know that the element has already been mounted (how else did you fire this event!),
        // so it's safe to unwrap here.
        // We can then call .value() to get the value out of the input, because NodeRef gives us access to a correctly-typed HTML element.
        <h1>"非受控组件"</h1>
        <form on:submit=on_submit>
            <input type="text"
                value=uname
                node_ref=input_element
            />
            <input type="submit" value="Submit Go"/>
        </form>

        <p>"Uncontrolled Name is: " {uname}</p>


        // Unlike <input>, the <textarea> element does not support a value attribute in HTML.
        // Instead, it receives its initial value as a plain text node in its HTML children.
        // So if you’d like to server render an initial value, and have the value also react in the browser,
        // you can both pass it an initial text node as a child and use prop:value to set its current value.
        <h1>"Textarea 组件"</h1>
        <textarea
            prop:value = textarea_value
            on:input = move |ev| {
                set_textarea_value.set(event_target_value(&ev));
            }
        >
            {textarea_value}
        </textarea>
        <p>"text area 中的值为: " <span style="color: pink;">{textarea_value}</span></p>


        <h1>"Select 组件"</h1>
        <label>
            "选择个数："
            <select
                on:change:target = move|ev| set_select_value.set(ev.target().value().parse().unwrap())
                prop:value= select_value
            >
                <option value="0">"0"</option>
                <option value="1">"1"</option>
                <option value="2">"2"</option>
            </select>
        </label>
        <button on:click=move |_| set_select_value.update(|n| {
            if *n == "2" {
                *n = "0".to_string();
            } else {
                *n = (n.parse::<i32>().unwrap() + 1).to_string();
            }
        })
            style="margin-left:5px;"
        >
            "Next Option"
        </button>

        <p>"select 中的选择是: " <span style="color: purple;">{select_value}</span></p>

    }
}

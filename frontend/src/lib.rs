use seed::browser::fetch::header::Header;
use seed::virtual_dom::el_ref::el_ref;
use seed::{prelude::*, *};
use serde::{ Deserialize, Serialize };
use serde_json::Value;
// use std::future::Future;
use shared::responses::{ApiResponse, TokenResponse, UserResponse};
use shared::payloads::CreateUserPayload;
use web_sys::HtmlInputElement;
use std::fmt;

mod api;
mod view;

// ------ ------
//     Model
// ------ ------

// `Model` describes our app state.
#[derive(Debug)]
pub struct Model {
    login_form: LoginForm,
    sign_up_form: SignUpForm,
    auth_token: Option<String>,
    current_user: Option<UserResponse>,
    base_url: Url,
    page: Page,
}

// impl<'a> Default for Model<'a> {
//     fn default() -> Self {
//         Model::Redirect(Session::default())
//     }
// }

#[derive(Debug, Default)]
pub struct LoginForm {
    username_input: ElRef<HtmlInputElement>,
    password_input: ElRef<HtmlInputElement>,
}
    #[derive(Debug, Default)]
pub struct SignUpForm {
    username_input: ElRef<HtmlInputElement>,
    password_input: ElRef<HtmlInputElement>,
}

#[derive(Debug)]
enum Page {
    Root, 
    Login,
    SignUp,
    UserProfile(String),
}

impl fmt::Display for Page {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Page::Root => write!(f, "/"),
            Page::Login => write!(f, "/login"), 
            Page::SignUp => write!(f, "/sign_up"),
            Page::UserProfile(username) => write!(f, "/users/{}", username.clone()),
        }
    }
}

// ------ ------
// Before Mount
// ------ ------

// ------ ------
//  After Mount
// ------ ------

// I think this is the initial build of the app, before update messages
fn after_mount(url: Url, _: &mut impl Orders<Msg>) -> AfterMount<Model> {

    AfterMount::new(Model {
        auth_token: None,
        current_user: None,
        base_url: url.to_base_url(),
        page: Page::Root,
        login_form: Default::default(),
        sign_up_form: Default::default(),
    })
}

// ------ ------
//    Update
// ------ ------

// `update` describes how to handle each `Msg`, and each 
// 'Msg" describes events you modify state (of the model) with
#[derive(Clone)]
pub enum Msg {
    LoginFormSubmitted,
    SignUpFormSubmitted,
    CreateUserEndpointResponded(String),
    MeLoaded(UserResponse),
    UrlChanged(Url),
//    UrlChanged(subs::UrlChanged),
    #[allow(dead_code)]
    Noop,
}

fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::Noop => {}
        Msg::UrlChanged(url) => {
            let path = url.path().iter().map(|s| s.as_str()).collect::<Vec<_>>();
            log!(path);

            let page = match path.as_slice() {
                ["sign_up"] => Page::SignUp,
                ["login"] => Page::Login,
                ["users", username] => Page::UserProfile(username.to_string()),
                [] => Page::Root,
                _ => todo!(),
            };

            model.page = page;

            seed::push_route(url);
        }
        Msg::MeLoaded(user) => {
            model.current_user = Some(user);
            log!("me loaded", model);
        }
        Msg::LoginFormSubmitted => {
            // let form = &model.login_form;
            // let username = form.username_input.get().unwrap().value();
            // let password = form.password_input.get().unwrap().value();
            // orders.perform_cmd(api::login(username, password));
        }
        Msg::SignUpFormSubmitted => {
            let form = &model.sign_up_form;
            let username = form.username_input.get().unwrap().value();
            let password = form.password_input.get().unwrap().value();
            orders.perform_cmd(api::create_user(username, password));
        }
        Msg::CreateUserEndpointResponded(token) => {
            model.auth_token = Some(token.clone());
            orders.perform_cmd(api::reload_current_user(token.to_string()));
        }
    }
}

#[derive(Debug, Deserialize)]
struct Data<T> {
    data: T,
}

// ------ ------
//     View
// ------ ------

// `view` describes what to display, based on the state of the model
// -- moved to module -- 

fn routes(url: Url) -> Option<Msg> {
    log!("url in routes fn", url);
    Some(Msg::UrlChanged(url))
}

// ------ ------
//     Start
// ------ ------

// `init` describes what should happen when your app started.
// fn init(url: Url, _: &mut impl Orders<Msg>) -> Model {
//     Model::default()
// }
//    start("app", init, update, view);


// (This function is invoked by `init` function in `index.html`.)
#[wasm_bindgen(start)]
pub fn start() {
    App::builder(update, view::view)
        .after_mount(after_mount)
        .routes(routes)
        .build_and_start();
}
use seed::{prelude::*, *};
use shared::responses::{
    ApiResponse, TokenResponse, UserResponse, TweetResponse,
};
use shared::payloads::CreateUserPayload;
use shared::payloads::CreateTweetPayload;
use web_sys::HtmlInputElement;
use std::fmt;
use shared::{GetUser, GetUserUrl};
use shared::{NoPayLoad, PostTweet, PostTweetUrl};

mod api;
mod view;

// ------ ------
//     Model - state of the application
// ------ ------

// `Model` describes our app state.
#[derive(Debug)]
pub struct Model {
    login_form: LoginForm,
    sign_up_form: SignUpForm,
    auth_token: Option<String>,
    current_user: Option<UserResponse>,
//    base_url: Url,
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
// Before Mount -- Still useful?
// ------ ------

// ------ ------
//  After Mount -- Deprecated
// ------ ------

// ------ ------
//    Update - change state with messages
// ------ ------

// `update` describes how to handle each `Msg`, and each 
// 'Msg" describes events you modify state (of the model) with
#[derive(Debug)]
pub enum Msg {
    LoginFormSubmitted,
    SignUpFormSubmitted,
    CreateUserEndpointResponded(String),
    MeLoaded(UserResponse),
    UrlChanged(subs::UrlChanged),
    LoadUserProfile(String),
    GetUserLoaded(UserResponse),
    TweetPosted(TweetResponse),
    RequestFailed(FetchError),
    #[allow(dead_code)]
    Noop,
}

fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::Noop => {}
        Msg::UrlChanged(subs::UrlChanged(url)) => {
            let page = url_to_page(&url);
            model.page = page;
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
        Msg::LoadUserProfile(username) => {
            orders.perform_cmd(api::fetch::<GetUser>(
                model.auth_token.clone(), 
                GetUserUrl { username },
                NoPayLoad,
                Msg::GetUserLoaded
            ));

            orders.perform_cmd(api::fetch::<PostTweet>(
//                Some("acXVaKX4mRrtilqMEcWjXGNjP1sXZla0".to_string()),
                model.auth_token.clone(), 
                PostTweetUrl,
                CreateTweetPayload {
                    text: "Tweet text".to_string(),
                },
                Msg::TweetPosted,
            ));
        }
        Msg::GetUserLoaded(user) => log!("user loaded", user),
        Msg::TweetPosted(tweet) => log!(tweet),
        Msg::RequestFailed(err) => log!("request failed", err),
    }
}

fn url_to_page(url: &Url) -> Page {
    let path = url.path().iter().map(|s| s.as_str()).collect::<Vec<_>>();

    match path.as_slice() {
        ["sign_up"] => Page::SignUp,
        ["login"] => Page::Login,
        ["users", username] => Page::UserProfile(username.to_string()),
        [] => Page::Root,
        _ => todo!(),
    }
}

// #[derive(Debug, Deserialize)]
// struct Data<T> {
//     data: T,
// }

// ------ ------
//     View - change your state into HTML
// ------ ------
// ** moved to module **


// ------ ------
//     Start
// ------ ------

// `init` describes what should happen when your app started.
fn init(url: Url, orders: &mut impl Orders<Msg>) -> Model {
    orders.subscribe(Msg::UrlChanged);
    orders.send_msg(Msg::UrlChanged(subs::UrlChanged(url.clone())));
    
    let page = url_to_page(&url);

    match &page {
        Page::UserProfile(username) => {
            orders.send_msg(Msg::LoadUserProfile(username.to_string()));
        }
        _ => {}
    }

    Model {
        auth_token: None,
        current_user: None,
        page,
        login_form: Default::default(),
        sign_up_form: Default::default(),
    }

}

// This function starts the app and initiates the js bindings
// It also monitors changes to the app, when messages are sent

#[wasm_bindgen(start)]
pub fn start() {
    App::start("app", init, update, view::view);
}

// Don't need this?
// fn routes(url: Url) -> Option<Msg> {
//     log!("url in routes fn", url);
//     Some(Msg::UrlChanged(url))
// }


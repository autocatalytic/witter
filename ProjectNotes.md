# Project Notes

Personal goals: learn more about Rust, study modern web frameworks, deploy some WASM


Secondary objectives: deploy something to personal server, be helpful to the open-source community (doesn't matter how), learn a modern IDE (looks like it'll be vscode) and cool project tools like https://github.com/davidpdrsn/assert-json-diff.git.


Side note: I began re-tooling my web understanding in the NodeJS, React, npm world and soon realized I was importing and integrating too many libraries that I had very little capacity to understand. Rather than continue a path requiring a detailed study of javascript I've decided to learn Rust because it's a more foundational language, which can also be quite webby since the introduction of WASM. 


Of course WASM has to talk with javascript (hello wasm_bindgen) so I'll likely have to head down that road eventually anyway, but hopefully my foundation will be more solid.

## Seed Model Description

Start function takes 
 - root element (app) 
 - the init function (AfterMount previously)
 - update
 - and view

"init" 
 - takes Url that the user is currently loading
 - loads it into the browser URI
 - implements the seed OrdersContainer
 - sets up the initial message on app start
 - builds and returns the model

 OrderContainer ("order") then allows you to 
 - set up subscriptions to different events in seed
 - for example, client notification that URL had changed
 - Subscribe on the type
 - Seed checks subscribers and notifies them when types change

"view"
 - Hrefs take anything that implements fmt::Display
 - generates HTML reflecting the model state

"update"
 - pattern matches on message events 
 - calls functions that implement changes in logic
 - stores information required to maintain state
 - rewrites (updates) the state of the model 
 - model updates trigger different views


## Backend Setup

This is intended to be a "bleeding edge" project to explore fully async Rust features, combined with WASM front end deployment. If we were building a project with ambitions of going to production in short-order, going with Node-JS or LAMP or another traditional stack would be the obvious choice. Finding people with the skills here is hard outside of crypto, and nearly all the layers are subject to breaking changes regularly. Indeed, in terms of performance Javascript is likely just as fast or faster than this build, because browsers are so well tuned for running compiled JS these days.

 - Running on mac OSX using default terminal and vscode
 - Postgres for the database, installed using Brew
   - Do NOT forget to create the postgres user as root!
   - Strange issue: have to `unset USER` when running `cargo test` or DB error
 - Using tide framework instead of Rocket because it's fully async
 - Running CORS middleware for decorating some messages

>> Cross Origin Resource Sharing (CORS) is a W3C standard that allows a server to relax the same-origin policy. Using CORS, a server can explicitly allow some cross-origin requests while rejecting others. CORS is safer and more flexible than earlier techniques such as JSONP.  https://fetch.spec.whatwg.org/#http-cors-protocol

 - Maximizing re-use of code between layers with ROOT/shared/src
 - 

todo!() - Messages diagram


todo!() - API description with abstractions


todo!() - Datamodel description


todo!() - Reasons for choosing tide write-up


## Need to figure out presentation layer details
CSS, look and feel, etc.

========================

## Witter status as of beginning of stream 7:

 - Have a tide server running on backend, connected to Postgres
 - Create routes for the front end with server.at
 - Create methods for handling endpoints like endpoints::users::create
 - Methods return tide Responses
 - Front end (seed/WASM) makes HTTP calls to backend,
 - using orders.perform_cmd to await messages and resend them

seed is async compatible, so HTTP calls, for example, are futures 

Goals for 8/11/2020: looking to improve the front/back code sharing
starting with ApiEndpoints, by creating traits common between them


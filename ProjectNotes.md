# Project Notes

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

 - Running on mac OSX using default terminal and vscode
 - Postgres for the database, installed using Brew
   - Do NOT forget to create the postgres user as root!
   - Strange issue: have to `unset USER` when starting cargo
 - Using tide framework instead of Rocket because it's fully async
 - Running CORS middleware for decorating some messages

> Cross Origin Resource Sharing (CORS) is a W3C standard that allows a server to relax the same-origin policy. Using CORS, a server can explicitly allow some cross-origin requests while rejecting others. CORS is safer and more flexible than earlier techniques such as JSONP.

 - Trying to maximize re-use of code between front/back end

todo!() - Messages diagram
todo!() - API description with abstractions
todo!() - Datamodel description
todo!() - Reasons for choosing tide write-up


## Need to figure out presentation layer details
CSS, look and feel, etc.

========================

# Witter status as of beginning of stream 7:

 - Have a tide server running on backend, connected to Postgres
 - Create routes for the front end with server.at
 - Create methods for handling endpoints like endpoints::users::create
 - Methods return tide Responses
 - Front end (seed/WASM) makes HTTP calls to backend,
 - using orders.perform_cmd to await messages and resend them

seed is async compatible, so HTTP calls, for example, are futures

Today, 8/11/2020, looking to improve the front/back code sharing
starting with ApiEndpoints, by creating traits common between them


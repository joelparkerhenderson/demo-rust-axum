h I add a nginx proxy in front of it to serve static files, rather than implementing it inside of the rust app. 

I'm using ServeDir from tower_http, it was easy enough to setup and I didn't want any external software to configure... well other than postgresql I guess. I also am using tower_helmet so I can set content security policy. I have to reevaluate how useful that is though, since I had to set script-src to unsafe-eval for alpine. You can get around it https://alpinejs.dev/advanced/csp but it makes alpine a lot more verbose. 


https://prest.blog/


    axum: fast and scalable, lots of middleware from tower

    axum-htmx: helpers when dealing with htmx headers

    axum-login: user auth, has oauth2 and user permissions

    tower-sessions: save user sessions (Redis, sqlite, memory, etc.)

    fred: Redis client for user sessions

    tracing: trace and instrument async logs

    reqwest: for API calls, oath2

    anyhow: turn any error into an AppError returning: “Internal Server Error”

    maud: templating html, can split fragments into functions in a single file (LoB)

    sqlx: dealing with a database, migrations, reverts



https://docs.rs/axum_session_auth/latest/axum_session_auth/

rust-embed + a custom tower service, to bundle truly static files (e.g. htmx.min.js) directly into the binary and serve them from memory at run time.

    I've been experimenting with using a build script to automatically download those dependencies as well, comparing them to expected hashes to prevent tampering.

grass to compile scss files. For debug builds this is done at request time to allow for quick changes to the css without having to restart the server, but for release builds it gets done at build time and bundled like the static assets.

moka for an in-process cache

tap. This isn't webdev specific, it's just really nice to be able to call call functions in a postfix manner like it allows with it's Pipe trait.


 (Together, these tips dramatically simplify deployment iME. I don't really need to worry about docker containers, setting up database and caching servers, making sure the right static resources are present in the expected paths, etc).

Non-library tips:

    My failable handlers return a custom error enum, with a few variants: one which wraps eyre::Report and a status code for generic errors (mostly 5** status codes), one which wraps maud::Markup and another status code for errors which I want to update the UI (mostly 4** status codes), and a third to handle redirects for handlers which might redirect, but might also return a normal response.

        I also have a few extension traits to handle things like changing the status code of the error response (if it's an error), converting None into 404 responses, etc.


 https://github.com/kellpossible/avalanche-report/

It uses:

    axum

    minijinja

    tailwindcss

    fluent localization

    sqlite

    rust-embed (all assets are embedded)



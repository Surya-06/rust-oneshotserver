# ONE-SHOT-SERVER

This is a crate for usecases where simple incoming http requests are to be handled for further data extraction. Usecases include OAuth token fetches where the token is usually sent as a response to a local webpage.

## Usage
This crate only has one function `start_listening_for_request` which takes a port number where the server will be hosted and a closure which runs after the incoming request is received. The request is handed over to the closure for further usage.

The function runs synchronously while blocking the thread where it is invoked on.


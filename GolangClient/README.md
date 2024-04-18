#  Vugu

This is a Vugu Client application that requests a specified number of data entries from the BackEnd.
It is ended for performance testing Golang E2E chain.
npx http-server -p 8087

## Building the app

Execute  `npm run build-prod-force` (or `go run dist.go`) from <projectRootDir>/GolangClient/


## Serving the app

Serve the app by, for instance, executing `npx http-server -p 8087` from <projectRootDir>/GolangClient/bundled-dist/

**Note** Data generation happens on the backend, so for this client-side app to function, the server needs to be up & running
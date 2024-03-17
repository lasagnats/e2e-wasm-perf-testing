# .NET REST API
This is a .NET REST API for E2E WASM performance testing.


## Commands
Serve the app via `dotnet run`.

## API
This service contains a /data/{count} endpoint that generates an array of entries in the format:
```
[
  {
    "id": 1,
    "label": "elegant blue car"
  },
  {
    "id": 2,
    "label": "short brown mouse"
  }
]
```

You can test the endpoint by executing `dotnet run` & navigating to ` http://localhost:50XX/swagger` in the browser.
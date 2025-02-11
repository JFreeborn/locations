# Location 

## Welcome to locations
This repository is going to be focused on using Lat. Long. points and determining where other lat/long points are located based on other inputs.

I will be learning Rust some more as this project progresses. So by no means should this be seen as production code or even good code for that matter. This will also be the ground for starting a UI project to learn on.

If you have found this, don't contribute to it. Reach out, if you can.

## Functionality
- Flow One:
  - Take in a single lat long point.
  - Take in a distance
  - Determine the lat long of the new point based on the provided distance.
- Flow Two: TODO
  - Take in a single lat lon pont.
  - take in a distance
  - create a bounding box around this point

## Notes
- Use this website to see about the GeoJson points on [This Site](https://geojson.io/#map=2/0/20)
- When the distance being calculated is 
  - less than 3 miles the [Euclidean](https://www.geeksforgeeks.org/euclidean-distance/) distance formula will be used
  - greater than 3 miles the [Haversine Formula](https://en.wikipedia.org/wiki/Haversine_formula) is used (Haversine formula combined with spherical trigonometry)
- for great distance Vincenty’s formulae or geodesic calculations will be explored but are not used for now. 
  - We are going to be operating on a perfect sphere of an earth for now

## TODOs 
- Build GeoJson for two points
- print the GeoJson for the two points
- check distance and use correct calculation
- Add validation 
- Add error handling 
- Add Tests
- Fix project structure
```aiignore
/my_rust_api
│── /src
│   ├── /api             # Presentation layer (Actix Web handlers, routes)
│   │   ├── mod.rs
│   │   ├── handlers.rs  # HTTP handlers (Controllers in C#)
│   │   ├── request.rs   # Request DTOs
│   │   ├── response.rs  # Response DTOs
│   │   ├── routes.rs    # Route definitions
│   │
│   ├── /application     # Application layer (Use Cases)
│   │   ├── mod.rs
│   │   ├── services.rs  # Business logic
│   │
│   ├── /domain          # Domain layer (Core Business Models)
│   │   ├── mod.rs
│   │   ├── models.rs    # Entities (e.g., User, Order)
│   │   ├── errors.rs    # Domain-specific errors
│   │
│   ├── /infrastructure  # Infra (Persistence, External APIs, Logging, etc.)
│   │   ├── mod.rs
│   │   ├── db.rs        # Database connections & repositories
│   │   ├── logging.rs   # Logging setup
│   │
│   ├── main.rs          # Entry point (bootstraps API & dependencies)
│   ├── config.rs        # Configuration setup
│   ├── lib.rs           # Re-exports modules for easy access
│
│── Cargo.toml           # Rust package config

```
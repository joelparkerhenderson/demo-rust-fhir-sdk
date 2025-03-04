# Demo Rust Fast Healthcare Interoperability Resources (FHIR) Software Development Kit (SDK)

Demonstration of:

* Rust programming language

* Fast Healthcare Interoperability Resources (FHIR) Software Development Kit (SDK)


## Examples

Run:

```sh
cargo run --example resource-identifier-access
cargo run --example read-resource
cargo run --example search-patients
cargo run --example re-authenticate-via-callback
```

## Docker

Run:

```sh
cargo make docker-ci-up
```

```stdout
[cargo-make] INFO - cargo make 0.37.24
[cargo-make] INFO - 
[cargo-make] INFO - Build File: Makefile.toml
[cargo-make] INFO - Task: docker-ci-up
[cargo-make] INFO - Profile: development
[cargo-make] INFO - Execute Command: "docker" "compose" "--profile" "ci" "up" "-d"
[+] Running 63/63
 ✔ hapi-r5 Pulled
 ✔ postgres Pulled
 ✔ medplum-r4 Pulled
 ✔ hapi-stu3 Pulled
 ✔ hapi-r4b Pulled
 ✔ Network docker_default         Created 
 ✔ Volume "docker_postgres-db"    Created 
 ✔ Container docker-redis-1       Started 
 ✘ Container docker-postgres-1    Error   
 ✔ Container docker-hapi-stu3-1   Created 
 ✔ Container docker-hapi-r5-1     Created 
 ✔ Container docker-medplum-r4-1  Created 
 ✔ Container docker-hapi-r4b-1    Created 
 ✔ Container docker-redis-1       Running 
 ✔ Container docker-postgres-1    Healthy 
 ✔ Container docker-hapi-r5-1     Started 
 ✔ Container docker-medplum-r4-1  Started 
 ✔ Container docker-hapi-stu3-1   Started 
 ✔ Container docker-hapi-r4b-1    Started 
[cargo-make] INFO - ==================Time Summary==================
[cargo-make] INFO - docker-ci-up:               93.57%     6.14 seconds
[cargo-make] INFO - [Setup Env]:                3.21%      0.21 seconds
[cargo-make] INFO - [Setup Env - Crate Info]:   2.33%      0.15 seconds
[cargo-make] INFO - [Setup Env - Git]:          0.56%      0.04 seconds
[cargo-make] INFO - [Setup Env - Rust]:         0.32%      0.02 seconds
[cargo-make] INFO - [Load Makefiles]:           0.00%      0.00 seconds
[cargo-make] INFO - [Setup Env - Duckscript]:   0.00%      0.00 seconds
[cargo-make] INFO - [Setup Env - CI]:           0.00%      0.00 seconds
[cargo-make] INFO - [Setup Env - Project]:      0.00%      0.00 seconds
[cargo-make] INFO - [Setup Env - Vars]:         0.00%      0.00 seconds
[cargo-make] INFO - ================================================
[cargo-make] INFO - Build Done in 6.36 seconds.
```

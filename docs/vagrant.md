
# cargo packages installation

When installing cargo packages this may not work or can take too much time.

```
vagrant plugin install vagrant-faster
```

## Runing actix-web inside vagrant
```
async fn main() -> Result<(), std::io::Error> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(greet))
            .route("/{name}", web::get().to(greet))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
```
Initially ```.bind("127.0.0.1:8080")?``` did not  work, so this was changed to ```0.0.0.0```` and then works.

http://localhost:8080/



# openssl-sys error when running cargo test
```
cargo test
warning: unused manifest key: target.x86_64-unknown-linux-gnu.rustflags
   Compiling openssl-sys v0.9.102
error: failed to run custom build command for `openssl-sys v0.9.102`
```
```
sudo apt install pkg-config
```
```
warning: unused manifest key: target.x86_64-unknown-linux-gnu.rustflags
   Compiling openssl-sys v0.9.102
   Compiling base64 v0.21.7
   Compiling openssl v0.10.64
```

# curl s
```
curl -v http://127.0.0.1:8080
```

```
curl -v http://127.0.0.1:8080//health_check
```

```
curl -v -X POST -d "name=le%20guin&email=ursula_le_guin%40gmail.com" http://127.0.0.1:8080/subscriptions -H "Content-Type: application/x-www-form-urlencoded"
```


# sqlx-cli
```
cargo install --version="~0.7" sqlx-cli --no-default-features \
  --features rustls,postgres
```

# psql
```
apt install postgresql-client
```
## Connect to a database
```
psql -U postgres -d newsletter -h localhost -p 5432
```

### Installing pgadmin
```
https://www.pgadmin.org/download/pgadmin-4-apt/
```
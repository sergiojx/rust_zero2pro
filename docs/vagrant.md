
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
# conc-url-pinger

Birden fazla URL'yi eşzamanlı olarak ping atan basit bir Rust uygulaması.

## Özellikler

- Tokio ile asenkron HTTP istekleri
- Eşzamanlı istek sınırlaması (semaphore)
- Basit hata yönetimi

## Kullanım

```bash
cargo run
```

URL'leri ve eşzamanlılık limitini `main.rs` dosyasından düzenleyebilirsiniz.

## Bağımlılıklar

- `tokio` - Async runtime
- `reqwest` - HTTP client

## Lisans

MIT

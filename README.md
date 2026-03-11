# 🕸️ oxigraph-web

![Rust](https://img.shields.io/badge/rust-1.71+-orange?style=flat-square&logo=rust) 
![License](https://img.shields.io/badge/license-MIT-blue?style=flat-square) 
![Crates.io](https://img.shields.io/crates/v/oxigraph-web?style=flat-square) 
![Build](https://img.shields.io/github/actions/workflow/status/<your-username>/oxigraph-web/rust.yml?style=flat-square) 

A **high-performance**, full-featured web API for **Oxigraph** – a modern Rust-based RDF/SPARQL database.  
Provides REST endpoints, SPARQL query support, RDF import/export, graph management, and OpenAPI documentation for building scalable **semantic web applications**.  

---

## 🚀 Features

- ⚡ **High-performance async REST API** built with [Salvo](https://salvo.rs/)  
- 🧠 **SPARQL support** via GET & POST
- 📡 **SPARQL UPDATE support** via POST
- 📦 **CRUD operations** for RDF triples  
- 📥 **RDF import**: Turtle, N-Triples, RDF/XML  
- 📤 **RDF export** in multiple formats  
- 🌐 **Named graph management**  
- 💾 **Persistent Oxigraph store**  
- 📖 **OpenAPI / Swagger UI documentation**  
- 🏗️ Ready for **production and scalable semantic web apps**

---

## 🏗️ Installation

Make sure you have Rust installed:  

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

Clone the repository:

```bash
git clone https://github.com/<your-username>/oxigraph-web.git
cd oxigraph-web
cargo build --release
```

## 🖥️ Running the Server
```bash
cargo run --release
```

Server runs on:
```
http://localhost:8080
```

## 🧩 API Endpoints

| Endpoint         | Method        | Description                                  |
| ---------------- | ------------- | -------------------------------------------- |
| `/health`        | GET           | Server health check                          |
| `/triples`       | POST          | Insert a triple                              |
| `/sparql`        | POST          | SPARQL query                                 |
| `ssparql/udate`  | POST          | SPQRQL update query
| `/sparql?query=` | GET           | SPARQL query via URL                         |
| `/graphs`        | POST / DELETE | Create or delete named graphs                |
| `/rdf/import`    | POST          | Import RDF data (Turtle, N-Triples, RDF/XML) |
| `/rdf/export`    | GET           | Export RDF data in selected format           |
| `/docs`          | GET           | OpenAPI / Swagger UI                         |


## 📝 Example Requests

**Insert Triple**
```bash
curl -X POST http://localhost:8080/triples \
-H "Content-Type: application/json" \
-d '{
  "subject":"http://example.com/alice",
  "predicate":"http://xmlns.com/foaf/0.1/name",
  "object":"http://example.com/Alice"
}'
```

**SPARQL Query (POST)**
```bash
curl -X POST http://localhost:8080/sparql \
-H "Content-Type: application/json" \
-d '{"query":"SELECT ?s ?p ?o WHERE { ?s ?p ?o }"}'
```

**Export RDF in Turtle**
```bash
curl -X GET "http://localhost:8080/rdf/export?format=ttl"
```

## ⚙️ Configuration

- Persistent database folder: data/ (can be changed in `db/oxigraph.rs`)

- Supported RDF formats for import/export: `ttl`, `nt`, `rdf`

## 📄 License

This project is licensed under the MIT License. See the LICENSE
 file for details.

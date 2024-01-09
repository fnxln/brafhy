
<p align="center">
  <img src="https://github.com/VitorCarvalho67/Barium/assets/102667323/ea71a41e-e3d9-4cbd-9797-5dba72b27a0a" />
</p>

# Brafhy (Messages App)

Brafhy is an advanced application developed in Rust and Vue.js, integrating Axum as the web framework and PostgreSQL for database management. Its core functionality revolves around a messaging environment, where users can exchange messages. The critical feature of this application is the use of WebSocket technology, which enables real-time communication between users. This ensures that messages are delivered and received instantly, enhancing the user experience with seamless, real-time interactions. The combination of these technologies - Rust for performance and safety, Vue.js for a reactive front-end, Axum for an efficient web framework, and PostgreSQL for reliable data storage - makes this project a robust solution for real-time messaging needs.

## How To Use
Prerequisites
- [Rust](https://www.rust-lang.org/tools/install)
- [Node.js](https://nodejs.org/en/download/)
- [PostgreSQL](https://www.postgresql.org/download/)
- [Docker](https://docs.docker.com/get-docker/)
- [Docker Compose](https://docs.docker.com/compose/install/)

Clone this repository

```bash
git clone hhttps://github.com/fnxln/brafhy.git
```

Navigate to the project directory

```bash
cd brafhy
```

### Server

Navigate to the server directory

```bash
cd back
```

Create a `.env` file in the server directory and add the following environment variables

```bash
DATABASE_URL=postgres://postgres:postgres@localhost:5432/brafhy
```

Build the server

```bash
cargo build --release
```

Run the server

```bash
# Windows
.\target\release\brafhy.exe
#Linux
./target/release/brafhy
#Macos
./target/release/brafhy
```

### Client

Navigate to the client directory

```bash
cd front
```

Install the dependencies

```bash
npm install
```

Run the client

```bash
npm run dev
```

## Contribute

**[Lin]**<br>
**[Vitor Carvalho]**

[Lin]: https://github.com/fnxln
[Vitor Carvalho]: https://github.com/VitorCarvalho67

## License
 The software is licensed under GPL-3.

<p >
  <img src="https://i.imgur.com/9kXfG6P.png" />
</p>
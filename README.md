# 7eblog_backend
Zomby7e's Blog - Backend, is just a **micro** blog backend.

This project is written in Rust, it depends on Actix, uses SQLite to store data.

Frontend (I developed it together): [7eblog_frontend](https://github.com/Zomby7e/7eblog_frontend)

Contribute: fork it and create pull request.

# Getting started

1. [Download and install rust.](https://www.rust-lang.org/learn/get-started)

2. Create a file named `about.md`, which is used to store blog information.

3. Create a blank SQLite database file (command line):

   ```bash
   sqlite3 blog_main.db "create table t(f int); drop table t;"
   ```

   *This is the database used to store articles.*

4. Change to the root directory of the project, simply run:

   ```bash
   cargo run
   ```

   *When the program is run for the first time, the data tables will be automatically generated.*

5. Use a software to modify the database file, then you can preview articles on frontend.

   if you have no idea, try [this](https://sqlitebrowser.org/). Articles are stored in pure markdown.

# TODO list

- [ ] Refactor the project to improve code readability.

- [ ] Build a picture storage service.
- [ ] In site search engine.
- [ ] Multithreading.
- [ ] Make a standard: include module paths, naming conventions for variables (or functions).
- [ ] Use a JSON file to configure the server, about(about.md) content should be included in it.
- [x] Will add more if needed.

# Contact Me

[Telegram: @zomby7e](https://t.me/zomby7e)

Email: zomby7e@gmail.com

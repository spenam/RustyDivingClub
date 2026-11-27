# 🤿 Diving Club Manager

**A modern, open-source management platform for scuba diving clubs.**

Diving Club Manager helps scuba diving clubs keep their members, equipment and equipment loans organised in one place. Built with **Rust**, **Axum** and **PostgreSQL**, it aims to provide a simple, reliable and maintainable solution for everyday club operations.

[![CI](https://github.com/OWNER/REPOSITORY/actions/workflows/ci.yml/badge.svg)](https://github.com/OWNER/REPOSITORY/actions/workflows/ci.yml)
![Rust](https://img.shields.io/badge/Rust-powered-orange?logo=rust)
![PostgreSQL](https://img.shields.io/badge/Database-PostgreSQL-blue?logo=postgresql)
![Docker](https://img.shields.io/badge/Deployment-Docker-2496ED?logo=docker)
![License](https://img.shields.io/badge/License-TBD-lightgrey)

---

## 🌊 Why Diving Club Manager?

Running a diving club involves more than organising dives. Members have qualifications, equipment needs regular maintenance, and borrowed gear needs to be tracked.

Diving Club Manager brings these activities together in a single application, helping clubs spend less time managing spreadsheets and more time enjoying the underwater world.

## ✨ Features

### 👥 Member management
- Keep track of club members.
- Record diving qualifications and certification levels.
- Support qualification levels:
  - N1, PN1, N2, PN2
  - N3, PN3, N4, PN4
  - E1, E2, E3, E4

### 🧰 Equipment management
- Organise equipment into categories.
- Record individual equipment items.
- Store equipment brands.
- Record regulator connection types:
  - DIN
  - Étrier
- Store sizes for BCDs and wetsuits.
- Track equipment condition:
  - Ready
  - Needs repair

### 📦 Equipment loans
- Check equipment out to club members.
- Include multiple items in a single loan.
- Record checkout and expected return dates.
- Track whether equipment is available or currently borrowed.
- Keep a history of equipment movements and comments.

### 📊 Excel export
- Export equipment information to Excel.
- Make inventory information easier to review and share.

### ⚙️ Continuous Integration
- GitHub Actions workflow.
- Automated Rust checks and tests.
- Clippy linting.
- Docker image build verification.

---

## 🛠️ Technology stack

| Technology | Purpose |
|---|---|
| Rust | Backend application |
| Axum | Web framework |
| SQLx | Database access |
| PostgreSQL | Relational database |
| HTML / CSS / JavaScript | User interface |
| Docker | Containerisation |
| GitHub Actions | Continuous integration |

---

## 🚀 Getting started

### Prerequisites

Before running the project, make sure you have:

- [Docker Desktop](https://www.docker.com/products/docker-desktop/)
- [Git](https://git-scm.com/)

### 1. Clone the repository

```bash
git clone https://github.com/OWNER/REPOSITORY.git
cd REPOSITORY
```

### 2. Configure environment variables

Check the project's Docker Compose configuration and `.env` requirements.

Create your local `.env` file using the example configuration if one is provided.

**Never commit passwords, database credentials or other secrets to GitHub.**

### 3. Start the application

```bash
docker compose up --build -d
```

### 4. Open the application

Visit:

**http://localhost:8080**

To follow application logs:

```bash
docker compose logs -f app
```

To stop the application:

```bash
docker compose down
```

---

## 🗂️ Project structure

```text
.
├── .github/
│   └── workflows/
│       └── ci.yml
├── migrations/
├── src/
├── Cargo.toml
├── Cargo.lock
├── Dockerfile
├── docker-compose.yml
└── README.md
```

*The exact structure may vary as the project evolves.*

---

## 🧪 Development

If you have Rust installed locally, you can run the following checks:

```bash
cargo check
cargo test
cargo clippy --all-targets --all-features
```

The GitHub Actions workflow runs automated checks when changes are pushed or pull requests are opened against the configured main branches.

---

## 🗺️ Roadmap

Planned improvements for future versions:

- [ ] Excel import for equipment inventory.
- [ ] More detailed equipment maintenance tracking.
- [ ] Improved member and equipment search.
- [ ] Equipment history and reporting improvements.
- [ ] Authentication and user roles.
- [ ] QR code support for faster equipment identification.

---

## 🤝 Contributing

Contributions, ideas and bug reports are welcome!

1. Fork the repository.
2. Create a feature branch.
3. Make your changes.
4. Run the relevant tests.
5. Open a pull request with a clear description.

If you find a bug or have an idea for an improvement, feel free to open an issue.

---

## 📜 License

Add the project's chosen open-source license here before publishing or redistributing the software.

---

## 🐠 Built for diving clubs

Made with ❤️ and Rust, for the people who make diving clubs work.

**Less paperwork. Better equipment tracking. More time underwater.**
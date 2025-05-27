# 🚨 Failtracker
[![Dockerized](https://img.shields.io/badge/Dockerized-yes-blue?logo=docker)](https://www.docker.com/)
[![Rust](https://img.shields.io/badge/Backend-Rust-orange?logo=rust)](https://www.rust-lang.org/)
[![React](https://img.shields.io/badge/Frontend-React-blue?logo=react)](https://reactjs.org/)
[![MIT License](https://img.shields.io/badge/license-MIT-green.svg)](LICENSE)
[![Cargo Build](https://img.shields.io/badge/build-cargo-informational?logo=rust)](https://doc.rust-lang.org/cargo/)
[![Node](https://img.shields.io/badge/node.js-frontend-%23339933?logo=node.js)](https://nodejs.org/)


**Failtracker** é uma aplicação web full stack para registrar, listar e apagar falhas relacionadas a hábitos pessoais. Toda vez que você "escorrega" em um hábito que deseja manter, pode registrar essa falha com apenas um clique — de forma rápida, leve e visual.

---

## 🧠 Visão Geral

O projeto é composto por dois serviços principais:

- 📦 **Backend**: API escrita em Rust com Axum, utilizando SQLite como banco de dados local.
- 🎨 **Frontend**: Interface web em React com Vite.
- 🐳 **Docker Compose**: Forma ideal de execução, unindo tudo automaticamente.

---
## ▶️ Como Rodar o Projeto

### ✅ Método Recomendado: Docker Compose

> Requisitos: [Docker](https://www.docker.com/get-started) e [Docker Compose](https://docs.docker.com/compose/install/)

---

#### 🐧 Linux

##### Ubuntu/Debian

```bash
sudo apt update
sudo apt install -y docker.io docker-compose
sudo systemctl start docker
sudo systemctl enable docker
```

##### Arch Linux

```bash
sudo pacman -Syu docker docker-compose
sudo systemctl start docker.service
sudo systemctl enable docker.service
```

> Para usar docker sem sudo, adicione seu usuário ao grupo docker:
> ```bash
> sudo usermod -aG docker $USER
> newgrp docker
> ```

---

#### 🍎 macOS

- Baixe e instale o [Docker Desktop para Mac](https://docs.docker.com/desktop/mac/install/)

---

#### 🪟 Windows

- Baixe e instale o [Docker Desktop para Windows](https://docs.docker.com/desktop/windows/install/)

> Obs: Docker Desktop requer Windows 10/11 Pro, Enterprise ou Education com Hyper-V habilitado.  
> Para Windows Home, considere usar Docker Toolbox ou WSL 2.

---

### ▶️ Rodando com Docker Compose

1. Clone o repositório:

```bash
git clone https://github.com/vejotaa/failtracker.git
cd failtracker
```

2. Suba tudo com Docker Compose:

```bash
sudo docker compose up --build
```

3. Acesse:

- Interface web: http://localhost:5173  
- API REST:       http://localhost:3000

---

### 🐳 Rodando pelo Docker Desktop (GUI) — Windows & macOS

Se você instalou o Docker Desktop, pode rodar o projeto usando a interface gráfica, sem linha de comando:

1. Abra o **Docker Desktop**.

2. Clique em **"Containers / Apps"** (ou similar).

3. Na barra lateral, clique em **"Create"** ou **"Add"** para importar seu projeto:

   - Você pode usar a opção para abrir a pasta do projeto onde está o `docker-compose.yml`.
   - Ou, caso não tenha, crie manualmente os containers usando a opção “Add container” e configurando as portas e volumes conforme seu `docker-compose.yml`.

4. Após importar, clique em **"Start"** para subir os containers do backend e frontend.

5. Aguarde a inicialização e acesse:

   - Interface web: [http://localhost:5173](http://localhost:5173)
   - API REST: [http://localhost:3000](http://localhost:3000)


Dica: No Docker Desktop você também pode ver logs, reiniciar e parar os containers facilmente pela interface, ideal para quem prefere não usar o terminal.

---

---
### 🛠️ Rodando Manualmente (Alternativa)

> Essa opção é útil para desenvolvimento local sem Docker.  
> Abaixo estão as instruções para instalar os requisitos no **Linux** e **Windows**.



#### 🧰 Requisitos

##### Backend (Rust)
- [Rust (via rustup)](https://www.rust-lang.org/tools/install)

**Linux/macOS**:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

**Windows**:

Acesse: [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)  
Baixe e execute o instalador (`rustup-init.exe`)

Após a instalação, verifique (no powershell):
```bash
rustc --version
cargo --version
```

##### Frontend (Node.js + Vite)

Você precisa do Node.js (que já inclui o npm).

**Linux (Debian/Ubuntu)**:
```bash
sudo apt update
sudo apt install nodejs npm
```

**Linux (Arch)**:
```bash
sudo pacman -S nodejs npm
```

**Windows**:  
Baixe e instale via [https://nodejs.org/](https://nodejs.org/)

Após a instalação, verifique (no PowerShell ou cmd):
```shell
node -v
npm -v
```

---

## ▶️ Rodando o Projeto Manualmente

### Backend (Rust)
```shell
cd failtracker-backend
cargo run
```

### Frontend (React)
```shell
cd failtracker-frontend
npm install
npm run dev
```

#### Acesse no navegador:

🌐 Frontend: http://localhost:5173  
🔌 Backend: http://localhost:3000

---

## 🚀 Tecnologias Utilizadas

### Backend
- **Linguagem**: Rust 🦀
- **Framework Web**: [Axum](https://github.com/tokio-rs/axum)
- **Banco de Dados**: SQLite com [SQLx](https://github.com/launchbadge/sqlx)
- **Async Runtime**: [Tokio](https://tokio.rs/)
- **CORS Middleware**: [tower-http](https://docs.rs/tower-http/)
- **Randomização de nomes**: [rand](https://crates.io/crates/rand)

### Frontend
- **Framework**: [React](https://reactjs.org/)
- **Empacotador/Dev Server**: [Vite](https://vitejs.dev/)

### Orquestração
- **Docker & Docker Compose**: Para rodar frontend e backend integrados, com apenas um comando.

---

## ✨ Funcionalidades

### API Backend

- `GET /` – Verifica se a API está ativa.
- `GET /falhas` – Lista todas as falhas.
- `POST /falhas` – Cria uma nova falha com nome aleatório (ex: *brave-yellow-panda*).
- `DELETE /falhas` – Apaga todas as falhas.

⚙️ A tabela `falhas` no SQLite é criada automaticamente se não existir.

### Frontend

- Interface simples e responsiva.
- Visualização da lista de falhas.
- Criação de nova falha.
- Remoção de todas as falhas.
- Feedback instantâneo nas ações.

---
## 🛣️ Próximos Passos

- 🔐 Autenticação (ex: JWT)
- 📆 Filtros por data ou categorias
- 📊 Dashboard com estatísticas
- ☁️ Deploy em nuvem (Render, Railway, Vercel...)

---
## 📄 Licença

Este projeto está licenciado sob a MIT License.
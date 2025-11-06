# FinanceVault 💰

A secure financial management application with end-to-end encryption, built with Rust (Axum) backend and SvelteKit frontend.

[![Build and Push Docker Image](https://github.com/CodeMaster4711/FinanceVault/actions/workflows/docker-build-push.yml/badge.svg)](https://github.com/CodeMaster4711/FinanceVault/actions/workflows/docker-build-push.yml)
[![Release](https://github.com/CodeMaster4711/FinanceVault/actions/workflows/release.yml/badge.svg)](https://github.com/CodeMaster4711/FinanceVault/actions/workflows/release.yml)

## 🚀 Features

- 🔐 **Secure Authentication** - JWT-based authentication with bcrypt password hashing
- 🔒 **RSA Encryption** - End-to-end encryption for sensitive data
- 💸 **Expense Tracking** - Track and categorize your expenses
- 📊 **Subscription Management** - Monitor recurring subscriptions
- 🗄️ **SQLite Database** - Lightweight and portable database
- 🐳 **Docker Ready** - Single unified container for easy deployment
- 📱 **Responsive UI** - Modern SvelteKit frontend with Tailwind CSS
- 📚 **API Documentation** - Interactive Swagger UI

## 🏃 Quick Start

### Using Docker (Recommended)

#### Option 1: Pull from GitHub Container Registry

```bash
docker run -d \
  --name financevault \
  -p 8000:8000 \
  -p 3000:3000 \
  -v financevault_data:/data \
  -e JWT_SECRET=$(openssl rand -hex 32) \
  -e RUST_LOG=info \
  ghcr.io/codemaster4711/financevault:latest
```

#### Option 2: Using the start script

```bash
# Download and run the start script
curl -O https://raw.githubusercontent.com/CodeMaster4711/FinanceVault/main/start-docker.sh
chmod +x start-docker.sh
./start-docker.sh
```

#### Option 3: Using Docker Compose

```bash
# Create .env file with JWT secret
echo "JWT_SECRET=$(openssl rand -hex 32)" > .env

# Start with docker-compose
docker-compose up -d

# View logs
docker-compose logs -f
```

### Access the Application

- **Frontend:** http://localhost:3000
- **Backend API:** http://localhost:8000
- **API Documentation:** http://localhost:8000/swagger-ui

## 🛠️ Development

### Prerequisites

- **Rust** (nightly) - Backend development
- **Node.js 20+** - Frontend development
- **Docker** - For containerized development
- **SQLite** - Database

### Local Development Setup

#### Backend

```bash
cd backend

# Install dependencies
cargo build

# Run migrations
cargo run --bin migration

# Start development server
cargo run
```

#### Frontend

```bash
cd frontend

# Install dependencies
npm install

# Start development server
npm run dev
```

### Building from Source

```bash
# Build Docker image
docker build -t financevault:local .

# Run locally built image
docker run -d \
  --name financevault \
  -p 8000:8000 \
  -p 3000:3000 \
  -v financevault_data:/data \
  -e JWT_SECRET=$(openssl rand -hex 32) \
  financevault:local
```

## 📦 Docker Commands

### Basic Operations

```bash
# Start container
docker start financevault

# Stop container
docker stop financevault

# View logs
docker logs -f financevault

# Restart container
docker restart financevault

# Remove container
docker rm -f financevault
```

### Troubleshooting

```bash
# Check container status
docker ps -a | grep financevault

# Access container shell
docker exec -it financevault /bin/bash

# View backend logs only
docker logs financevault 2>&1 | grep "\[BACKEND\]"

# View frontend logs only
docker logs financevault 2>&1 | grep "\[FRONTEND\]"
```

## 🔒 Security

### Environment Variables

| Variable       | Required | Default                   | Description                                        |
| -------------- | -------- | ------------------------- | -------------------------------------------------- |
| `JWT_SECRET`   | Yes      | -                         | Secret key for JWT token generation (min 32 chars) |
| `RUST_LOG`     | No       | `info`                    | Log level (`debug`, `info`, `warn`, `error`)       |
| `DATABASE_URL` | No       | `sqlite:/data/finance.db` | Database connection string                         |
| `FRONTEND_URL` | No       | `http://localhost:3000`   | Frontend URL for CORS                              |

### Generate Secure JWT Secret

```bash
# Generate a secure random JWT secret
openssl rand -hex 32
```

⚠️ **Important:** Always use a strong, randomly generated `JWT_SECRET` in production!

## 📊 Database

The application uses SQLite with automatic migrations. The database file is stored in `/data/finance.db` inside the container, which is persisted using Docker volumes.

### Backup Database

```bash
# Create backup
docker cp financevault:/data/finance.db ./backup-$(date +%Y%m%d).db

# Restore backup
docker cp ./backup-20250104.db financevault:/data/finance.db
docker restart financevault
```

## 🏗️ Architecture

```
┌─────────────────────────────────────────────┐
│         Docker Container                    │
│                                             │
│  ┌──────────────┐    ┌──────────────────┐ │
│  │  Frontend    │    │     Backend      │ │
│  │  SvelteKit   │───▶│   Rust (Axum)    │ │
│  │  Port 3000   │    │    Port 8000     │ │
│  └──────────────┘    └──────────────────┘ │
│                             │               │
│                      ┌──────▼────────┐     │
│                      │  SQLite DB    │     │
│                      │  /data/       │     │
│                      └───────────────┘     │
└─────────────────────────────────────────────┘
```

## 🧪 Testing

```bash
# Backend tests
cd backend
cargo test

# Frontend tests
cd frontend
npm test

# Test Docker build
./test-docker.sh
```

## 📝 API Endpoints

### Authentication

- `POST /api/register` - Register new user
- `POST /api/login` - Login user
- `POST /api/logout` - Logout user
- `GET /api/user` - Get user profile

### Expenses

- `GET /api/expenses` - List expenses
- `POST /api/expenses` - Create expense
- `GET /api/expenses/:id` - Get expense
- `PUT /api/expenses/:id` - Update expense
- `DELETE /api/expenses/:id` - Delete expense

### Subscriptions

- `GET /api/subscriptions` - List subscriptions
- `POST /api/subscriptions` - Create subscription
- `GET /api/subscriptions/:id` - Get subscription
- `PUT /api/subscriptions/:id` - Update subscription
- `DELETE /api/subscriptions/:id` - Delete subscription

## 🤝 Contributing

We use [Conventional Commits](https://www.conventionalcommits.org/) for semantic versioning.

See [CONTRIBUTING.md](CONTRIBUTING.md) for detailed guidelines.

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- Built with [Axum](https://github.com/tokio-rs/axum) web framework
- Frontend powered by [SvelteKit](https://kit.svelte.dev/)
- Database managed with [SeaORM](https://www.sea-ql.org/SeaORM/)
- UI components from [shadcn-svelte](https://www.shadcn-svelte.com/)

## 📞 Support

- 📖 [Documentation](https://github.com/CodeMaster4711/FinanceVault/wiki)
- 🐛 [Report Bug](https://github.com/CodeMaster4711/FinanceVault/issues)
- 💡 [Request Feature](https://github.com/CodeMaster4711/FinanceVault/issues)

---

Made with ❤️ by the FinanceVault Team

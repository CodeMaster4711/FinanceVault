# FinanceVault - Architektur

## Überblick

FinanceVault verwendet eine **Single-Binary-Architektur** mit einem Backend-Proxy für das Frontend.

## Architektur-Diagramm

```
┌─────────────────────────────────────────────────┐
│                  Docker Container                │
│                                                  │
│  ┌────────────────────────────────────────────┐ │
│  │         Backend (Rust/Axum)                 │ │
│  │         Port: 8000                          │ │
│  │                                             │ │
│  │  ┌──────────────────────────────────────┐  │ │
│  │  │  API Routes (/api/*)                 │  │ │
│  │  │  - /api/users                        │  │ │
│  │  │  - /api/expenses                     │  │ │
│  │  │  - /api/subscriptions                │  │ │
│  │  └──────────────────────────────────────┘  │ │
│  │                                             │ │
│  │  ┌──────────────────────────────────────┐  │ │
│  │  │  Swagger UI (/swagger-ui)           │  │ │
│  │  └──────────────────────────────────────┘  │ │
│  │                                             │ │
│  │  ┌──────────────────────────────────────┐  │ │
│  │  │  Frontend Proxy (fallback)          │  │ │
│  │  │  Proxies all other requests to:     │  │ │
│  │  │  → Frontend on port 3000            │  │ │
│  │  └──────────────────────────────────────┘  │ │
│  └────────────────────────────────────────────┘ │
│                       ↓                          │
│  ┌────────────────────────────────────────────┐ │
│  │       Frontend (SvelteKit/Node)            │ │
│  │       Port: 3000 (internal only)           │ │
│  └────────────────────────────────────────────┘ │
│                                                  │
│  ┌────────────────────────────────────────────┐ │
│  │       SQLite Database                      │ │
│  │       /data/finance.db                     │ │
│  └────────────────────────────────────────────┘ │
└─────────────────────────────────────────────────┘
                        │
                   Port 8000 (exposed)
                        ↓
                    🌐 Users
```

## Request Flow

### 1. API Requests (`/api/*`)

```
User → Backend:8000/api/users
     → Backend verarbeitet Request direkt
     → Rückgabe: JSON Response
```

### 2. Frontend Requests (alle anderen)

```
User → Backend:8000/
     → Backend proxied zu Frontend:3000/
     → Frontend rendert HTML/CSS/JS
     → Backend gibt Response zurück
     → Rückgabe: HTML/Assets
```

## Komponenten

### Backend (Rust/Axum)

- **Port**: 8000 (extern erreichbar)
- **Verantwortlichkeiten**:
  - REST API für User, Expenses, Subscriptions
  - JWT Authentication
  - Datenbank-Zugriff (SQLite)
  - Frontend Proxy für alle nicht-API Routen
  - Swagger UI Dokumentation

### Frontend (SvelteKit/Node.js)

- **Port**: 3000 (nur intern, nicht extern erreichbar)
- **Verantwortlichkeiten**:
  - Server-Side Rendering (SSR)
  - Client-Side Rendering (CSR)
  - UI/UX
  - Routing

### Datenbank (SQLite)

- **Location**: `/data/finance.db`
- **Type**: SQLite (file-based)
- **Migrations**: Automatisch beim Backend-Start

## Vorteile dieser Architektur

1. **Single Entry Point**: Nur Port 8000 muss exponiert werden
2. **Vereinfachte Deployment**: Nur ein Container nötig
3. **Bessere Performance**: Kein externes Netzwerk zwischen Backend und Frontend
4. **Einfachere CORS-Konfiguration**: Alles kommt von derselben Domain
5. **Produktions-Ready**: Frontend läuft auf internem Port, Backend handhabt alle externen Requests

## Umgebungsvariablen

| Variable       | Standard                  | Beschreibung           |
| -------------- | ------------------------- | ---------------------- |
| `DATABASE_URL` | `sqlite:/data/finance.db` | SQLite Datenbank-Pfad  |
| `RUST_LOG`     | `info`                    | Log-Level für Backend  |
| `FRONTEND_URL` | `http://localhost:3000`   | Interne Frontend-URL   |
| `JWT_SECRET`   | _required_                | Secret für JWT-Token   |
| `PORT`         | `3000`                    | Frontend Port (intern) |
| `ORIGIN`       | `http://localhost:8000`   | SvelteKit Origin       |

## Entwicklung vs. Produktion

### Entwicklung (Dev-Modus)

```yaml
# docker-compose.dev.yml
services:
  backend:
    build: ./backend
    ports:
      - "8000:8000"
    environment:
      FRONTEND_URL: "http://frontend:5173" # Vite dev server

  frontend:
    build: ./frontend
    ports:
      - "5173:5173" # Vite dev server
```

### Produktion

```yaml
# docker-compose.prod.yml
services:
  app:
    image: financevault:latest
    ports:
      - "8000:8000" # Nur Backend-Port exponiert
    volumes:
      - financevault_data:/data
```

## Build-Prozess

### Multi-Stage Docker Build

1. **Frontend Build Stage**:

   ```dockerfile
   FROM node:20-alpine AS frontend-builder
   # npm ci && npm run build
   ```

2. **Backend Build Stage**:

   ```dockerfile
   FROM rustlang/rust:nightly AS backend-builder
   # cargo build --release
   ```

3. **Final Runtime Stage**:
   ```dockerfile
   FROM debian:bookworm-slim
   # Kopiert Backend-Binary + Frontend-Build
   # Installiert nur Runtime-Dependencies
   ```

## Startup-Sequenz

1. **Frontend startet** (Port 3000, intern)

   - SvelteKit Node.js Server
   - Wartet bis bereit

2. **Backend startet** (Port 8000, extern)

   - Rust/Axum Server
   - Proxied zu Frontend für nicht-API Requests

3. **Health Checks**
   - Backend: `GET http://localhost:8000/`
   - Frontend: `GET http://localhost:3000/`

## Sicherheit

- **JWT Authentication**: Alle geschützten API-Routen
- **CORS**: Konfiguriert für Frontend-URL
- **SQLite**: File-based, gespeichert in `/data` Volume
- **Environment Secrets**: `JWT_SECRET` muss gesetzt werden

## Monitoring & Logging

- Backend-Logs: Präfix `[BACKEND]`
- Frontend-Logs: Präfix `[FRONTEND]`
- Strukturiertes Logging mit `tracing` (Rust)
- Request-Logging mit Latency-Tracking

## Deployment

### Docker Hub / GHCR

```bash
# Pull Image
docker pull ghcr.io/codemaster4711/financevault:latest

# Run Container
docker run -d \
  -p 8000:8000 \
  -v financevault_data:/data \
  -e JWT_SECRET=$(openssl rand -hex 32) \
  ghcr.io/codemaster4711/financevault:latest
```

### Zugriff

- Web Interface: http://localhost:8000
- API Docs: http://localhost:8000/swagger-ui
- API: http://localhost:8000/api

## Troubleshooting

### Backend kann Frontend nicht erreichen

```bash
# Prüfe, ob Frontend läuft
docker exec -it <container> curl http://localhost:3000

# Prüfe Logs
docker logs <container> | grep FRONTEND
```

### Frontend Proxy Fehler

```bash
# Prüfe FRONTEND_URL Environment Variable
docker exec -it <container> env | grep FRONTEND_URL

# Backend Logs für Proxy-Fehler
docker logs <container> | grep "Frontend proxy error"
```

### Datenbank-Probleme

```bash
# Prüfe Datenbank-Datei
docker exec -it <container> ls -lh /data/

# Prüfe Datenbank-Verbindung
docker exec -it <container> sqlite3 /data/finance.db ".tables"
```

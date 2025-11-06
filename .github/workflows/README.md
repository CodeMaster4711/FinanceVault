# GitHub Actions Workflows

Dieses Verzeichnis enthält alle CI/CD Workflows für FinanceVault.

## Workflows

### 📦 `release.yml` - Semantic Release
Automatisches Versioning und Release-Erstellung basierend auf Conventional Commits.

**Trigger:** Push auf `main`

**Funktionen:**
- 🔍 Analysiert Commits
- 🏷️ Erstellt neue Version (semver)
- 📝 Generiert CHANGELOG
- 🔄 Aktualisiert package.json und Cargo.toml
- 📦 Erstellt GitHub Release
- 🏷️ Erstellt Git Tag

### 🐳 `docker-build-push.yml` - Docker Build & Push
Baut Multi-Arch Docker Images und pushed sie zu GitHub Container Registry.

**Trigger:** Git Tags (`v*`)

**Funktionen:**
- 🏗️ Multi-Architecture Build (AMD64, ARM64)
- 📤 Push zu ghcr.io
- 🏷️ Smart Tagging (version, major.minor, latest)
- 💾 Build Cache für schnellere Builds

## Scripts

### `scripts/update-versions.sh`
Helper-Script zum Aktualisieren der Versionen in:
- `package.json` (Root)
- `frontend/package.json`
- `backend/Cargo.toml`

## Dokumentation

Siehe [CICD.md](./CICD.md) für detaillierte Workflow-Dokumentation.

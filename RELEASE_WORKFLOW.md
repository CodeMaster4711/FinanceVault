# 🚀 Automatischer Release und Build Workflow

## ✅ Setup abgeschlossen!

Dein FinanceVault Projekt ist jetzt mit einem vollautomatischen Release- und Build-Workflow ausgestattet!

## 🔄 So funktioniert es

### 1️⃣ Code entwickeln und committen

Verwende **Conventional Commits** für deine Commit-Messages:

```bash
# Feature hinzufügen → MINOR version bump (z.B. 0.1.0 → 0.2.0)
git commit -m "feat(dashboard): add expense charts"

# Bugfix → PATCH version bump (z.B. 0.1.0 → 0.1.1)
git commit -m "fix(api): correct expense calculation"

# Breaking Change → MAJOR version bump (z.B. 0.1.0 → 1.0.0)
git commit -m "feat(api)!: redesign authentication

BREAKING CHANGE: Authentication now requires OAuth2"
```

### 2️⃣ Pull Request erstellen und mergen

```bash
# Feature Branch erstellen
git checkout -b feat/my-feature

# Entwickeln und committen
git commit -m "feat(feature): add new feature"

# Push und PR erstellen
git push origin feat/my-feature

# Nach Review: Merge in main
```

### 3️⃣ Automatischer Release (nach Merge in main)

Wenn du in `main` mergst, passiert automatisch:

1. ✅ **Semantic Release** analysiert Commits
2. 📊 Bestimmt neue Version
3. 📝 Erstellt/Updated `CHANGELOG.md`
4. 🔄 Updated Versionen in:
   - `package.json`
   - `frontend/package.json`
   - `backend/Cargo.toml`
5. 🏷️ Erstellt Git Tag (z.B. `v1.2.3`)
6. 📦 Erstellt GitHub Release
7. 💾 Committet Änderungen zurück

### 4️⃣ Automatischer Docker Build

Der Git Tag (`v1.2.3`) triggert automatisch:

1. 🏗️ Multi-Arch Build (AMD64 + ARM64)
2. 🏷️ Smart Tagging:
   - `v1.2.3` (exakte Version)
   - `v1.2` (major.minor)
   - `latest` (neueste Version)
3. 📤 Push zu GitHub Container Registry

### 5️⃣ Fertig! 🎉

Deine neue Version ist jetzt verfügbar:

```bash
docker pull ghcr.io/codemaster4711/financevault:v1.2.3
# oder
docker pull ghcr.io/codemaster4711/financevault:latest
```

## 📋 Quick Reference

### Commit Types

| Type | Effekt | Beispiel |
|------|--------|----------|
| `feat:` | MINOR bump | `feat(auth): add OAuth2` |
| `fix:` | PATCH bump | `fix(api): correct calculation` |
| `perf:` | PATCH bump | `perf(db): optimize query` |
| `refactor:` | PATCH bump | `refactor(ui): restructure` |
| `build:` | PATCH bump | `build(docker): update base` |
| `docs:` | Kein Release | `docs(readme): update guide` |
| `test:` | Kein Release | `test(api): add tests` |
| `chore:` | Kein Release | `chore(deps): update deps` |

### Breaking Changes

Für MAJOR bump (z.B. 1.0.0 → 2.0.0):

```bash
git commit -m "feat(api)!: redesign endpoints

BREAKING CHANGE: API endpoints restructured"
```

## 🧪 Testen

Teste die Version-Update-Funktion lokal:

```bash
chmod +x .github/scripts/test-update-versions.sh
.github/scripts/test-update-versions.sh
```

## 📖 Weitere Dokumentation

- 📘 [Detaillierte CI/CD Dokumentation](.github/CICD.md)
- 📗 [Workflow README](.github/workflows/README.md)
- 📙 [Conventional Commits](https://www.conventionalcommits.org/)

## ⚠️ Wichtig

- ❌ **NIEMALS** Versionen manuell in `package.json` oder `Cargo.toml` ändern!
- ✅ Versionen werden **automatisch** durch semantic-release verwaltet
- ✅ Immer **Conventional Commits** verwenden
- ✅ Nach dem Merge in `main` läuft alles automatisch

## 🔧 Troubleshooting

### Release wird nicht erstellt

**Problem:** Nach Merge in `main` passiert nichts

**Lösung:**
1. Prüfe ob Commits das richtige Format haben
2. Check GitHub Actions Log
3. Mindestens ein `feat:`, `fix:`, etc. Commit muss vorhanden sein

### Docker Build schlägt fehl

**Problem:** Build-Fehler im Docker Workflow

**Lösung:**
1. Check `docker-build-push.yml` Actions Log
2. Teste Build lokal: `docker build -t test .`

### Version nicht aktualisiert

**Problem:** Versionen in Dateien nicht aktualisiert

**Lösung:**
1. Check `release.yml` Actions Log
2. Teste Update-Script lokal: `.github/scripts/test-update-versions.sh`

## 🎯 Nächste Schritte

1. ✅ Merge deinen aktuellen PR in `main`
2. ✅ Beobachte die GitHub Actions
3. ✅ Prüfe das neu erstellte Release
4. ✅ Teste das Docker Image

Viel Erfolg! 🚀

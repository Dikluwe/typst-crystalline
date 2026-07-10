# P678 — Sonda: suporte a pacotes `@preview`

**Passo:** 678
**Data:** 2026-07-10
**Foco:** mapear o mecanismo real de pacotes do vanilla e o estado actual do cristalino, antes de qualquer código.
**Tipo:** Sonda directa. Sem implementação.
**Commit base:** `73594ff67 — P677: adiciona hash do commit ao relatório`
**ADR-0108 em vigor. ADR-0114 em vigor** (funcionalidade nunca tocada; sonda obrigatória).

---

## 1. Mecanismo do vanilla (fonte: `lab/typst-original`)

### 1.1 Resolução — `SystemPackages::obtain` (`crates/typst-kit/src/packages.rs:95`)

Ordem de prioridade, por pacote:

1. **Data dir** (`FsPackages::system_data()`) — pacotes do utilizador/sistema:
   - Linux: `$XDG_DATA_HOME/typst/packages` ou `~/.local/share/typst/packages`
   - macOS: `~/Library/Application Support/typst/packages`
   - Windows: `%APPDATA%/typst/packages`
2. **Cache dir** (`FsPackages::system_cache()`) — pacotes descarregados automaticamente:
   - Linux: `$XDG_CACHE_HOME/typst/packages` ou `~/.cache/typst/packages`
3. **Download** (`UniversePackages`) — só se `spec.namespace == "preview"`; caso contrário `NotFound`.

A fonte dos pacotes é **mista**: directórios locais primeiro, registo online como fallback. Sem ligação, o que já estiver em cache funciona; o que não estiver falha (`NotFound`/`NetworkFailed`).

### 1.2 Registo — `UniversePackages` (`packages.rs:314`)

- URL base: `https://packages.typst.org` (`UniversePackages::new`, `packages.rs:330`).
- Namespace servido: `"preview"` (`UniversePackages::NAMESPACE`, `packages.rs:326`).
- Download de pacote: `GET {url}/preview/{name}-{version}.tar.gz` (`packages.rs:359`), descomprime gzip + untar em memória (`flate2` + `tar`).
- Índice de versões: `GET {url}/preview/index.json` (`packages.rs:426`), cacheado em memória por instância (`OnceCell`).
- Comentário do próprio código: "There is no standardized registry protocol. This is merely designed to work with the official Typst Universe package registry." (`packages.rs:311`) — o protocolo **não é estável**; a implementação segue o registo oficial.

### 1.3 Cache local — `FsPackages` (`packages.rs:145`)

Estrutura de directórios:

```text
{cache|data}/typst/packages/{namespace}/{name}/{version}/
```

Confirmado no ambiente de testes:

```text
~/.cache/typst/packages/preview/cetz/0.2.2/
~/.cache/typst/packages/preview/oxifmt/0.2.0/
~/.cache/typst/packages/preview/tidy/0.3.0/   # descarregado durante a sonda
```

Cada directório de versão contém `typst.toml` (manifesto) mais os ficheiros do pacote. O manifesto tem a forma (`cetz/0.2.2/typst.toml`):

```toml
[package]
name = "cetz"
version = "0.2.2"
compiler = "0.10.0"
entrypoint = "src/lib.typ"
# ... authors, license, description, categories, keywords, exclude
```

A escrita é feita via `Tempdir` + `rename` atómico (`packages.rs:225`) para tolerância a concorrência; não há verificação de integridade/checksum.

### 1.4 Resolução de versão (`packages.rs:127`, `packages.rs:385`)

- Versão explícita no spec (`@preview/nome:1.2.3`) — usada tal qual.
- Versão omitida → `latest_version`:
  - Para `preview`: consulta `index.json` e devolve o `max()` das versões do pacote.
  - Para outros namespaces: procura só na **data dir** (não na cache) e devolve o `max()` local; sem data dir, erro "please specify the desired version".
- Ranges de versão (`^1.2`, `~1.2`, `>=1.2`) **não existem** no spec de import; o spec é sempre uma versão exacta ou omissa. `VersionBound` existe só para o campo `compiler` do manifesto (compatibilidade), não para selecção de versão de pacote.

### 1.5 Sintaxe e parsing — `PackageSpec` (`crates/typst-syntax/src/package.rs`)

`@preview/nome:versao` parseia para `PackageSpec { namespace, name, version }`:
- `parse_namespace`: exige `@` inicial, ident até `/`.
- `parse_name`: ident até `:`.
- `parse_version`: `major.minor.patch`, todos numéricos, exactamente 3 componentes.

### 1.6 Eval — `import` (`crates/typst-eval/src/import.rs:212`)

```rust
pub fn import(engine, from, span) {
    if from.starts_with('@') {
        let spec = from.parse::<PackageSpec>().at(span)?;
        import_package(engine, spec, span)
    } else {
        // import de ficheiro local (caminho relativo)
        import_file(engine, path, span)
    }
}
```

`import_package` → `resolve_package` lê `typst.toml` via `world.file(VirtualRoot::Package(spec))`, valida o manifesto, e devolve o `FileId` do `entrypoint`. Depois `import_file` reutiliza o mecanismo normal de avaliação de ficheiro. **A resolução de pacote termina num `FileId`; a partir daí é import de ficheiro comum.**

O `World` do vanilla expõe ficheiros de pacote através de `VirtualRoot::Package(spec)`, que o `SystemWorld`/`FileStore` mapeia para `SystemPackages::obtain`.

---

## 2. Testes directos

### 2.1 Vanilla consegue descarregar (acesso de rede confirmado)

```bash
cat > /tmp/p678-net.typ <<'EOF'
#import "@preview/tidy:0.3.0": parse-module
EOF
lab/typst-original/target/release/typst compile /tmp/p678-net.typ /tmp/p678-net.pdf
```

Resultado:

```text
downloading @preview/tidy:0.3.0
  17.2 KiB /  17.2 KiB (100 %), 171.9 KiB/s, ETA: 0 s
Exit code: 0
```

`tidy/0.3.0` apareceu em `~/.cache/typst/packages/preview/tidy/0.3.0`. **O ambiente de testes tem acesso à internet** e o vanilla descarrega e cacheia pacotes reais.

### 2.2 Vanilla funciona offline com pacote já em cache

```bash
cat > /tmp/p678-pacote.typ <<'EOF'
#import "@preview/cetz:0.2.2": canvas, draw
EOF
lab/typst-original/target/release/typst compile /tmp/p678-pacote.typ /tmp/p678-vanilla.pdf
```

`cetz/0.2.2` já estava em cache. Resultado: **exit 0**, PDF gerado (2151 bytes), sem tentativa de download.

### 2.3 Cristalino — mensagem exacta hoje

```bash
./target/release/typst /tmp/p678-pacote.typ /tmp/p678-cristalino.pdf
```

Resultado:

```text
/tmp/p678-pacote.typ:1:2: error: import não implementado nesta versão do cristalino
Exit code: 1
```

A mesma mensagem aparece para `#import "ficheiro-local.typ": ...` — ou seja, **`#import` como um todo está por implementar**, não apenas pacotes.

### 2.4 Cristalino — `#include` funciona

```bash
#import "p678-utils.typ": saudacao   # → erro "import não implementado"
#include "p678-utils.typ"            # → exit 0
```

`eval_module_include` (`01_core/src/rules/eval/modules.rs:33`) está implementado; `eval_module_import` (`modules.rs:25`) é um stub:

```rust
pub(super) fn eval_module_import(import: ModuleImport<'_>) -> SourceResult<Value> {
    // import não implementado — Passo 33+
    Err(vec![SourceDiagnostic::error(import.span(),
        "import não implementado nesta versão do cristalino")])
}
```

---

## 3. Estado actual do cristalino, nível a nível

| Nível | O que é | Estado no cristalino | Evidência |
|---|---|---|---|
| **1. Parsing** | Reconhecer `@preview/nome:versao` (distinto de caminho local) | **Feito** | `01_core/src/entities/package_spec.rs` implementa `PackageSpec`, `VersionlessPackageSpec`, `PackageVersion`, `VersionBound` (wildcards); `ModuleImport::bare_name` já faz `PackageSpec::from_str` (`entities/ast/code.rs:164`) |
| **2. Resolução de rede** | Descarregar do registo oficial | **Ausente** | Sem cliente HTTP nas deps (`grep` por `reqwest`/`ureq`/`hyper` só devolve `flate2`, usado para PNG). Vanilla usa `native_tls` + HTTP/1.1 próprio (`typst-kit/src/downloader.rs`) |
| **3. Cache local** | Guardar pacotes descarregados | **Ausente** | Sem gestão de `~/.cache/typst/packages`. O `SystemWorld` só conhece ficheiros registados explicitamente |
| **4. Resolução de versão** | Versão exacta / omissa / `latest` | **Parcial** | Parsing de versão exacta e `VersionBound` existem; falta `latest_version` (consulta a `index.json`) e a distinção namespace `preview` vs local |
| **5. Execução (reuso de `#import`)** | Pacote resolvido → `FileId` → import de ficheiro | **Não reutilizável** | `#import` é stub; o mecanismo de módulo (escopo, `with_name`, items/wildcard) não existe. Só `#include` funciona |

**Conclusão-chave:** o maior bloqueador não é o mecanismo de pacotes em si — é que **`#import` está por implementar**. Pacotes apoiam-se num `#import` de ficheiros a funcionar. Sem ele, os níveis 2-4 não têm onde aterrar.

Diferença estrutural adicional: o vanilla distingue raízes virtuais (`VirtualRoot::Package(spec)` vs `VirtualRoot::Project`); o cristalino usa `FileId = NonZeroU16` simples (`01_core/src/entities/file_id.rs`), sem noção de raiz. A resolução de pacote teria de mapear `PackageSpec → directório local → SystemWorld::register_file → FileId` em L3.

---

## 4. Estimativa de tamanho por nível

| Nível | Tamanho | Justificação |
|---|---|---|
| 1. Parsing | **(feito)** | Já existe em L1. |
| 5a. `#import` de ficheiros locais | **L** | Pré-requisito. Implementar `eval_module_import`: resolver caminho, avaliar ficheiro em escopo de módulo, `with_name`, items/wildcard/rename, detecção de ciclos (já existe `Route::contains`). É a peça central em falta. |
| 5b. Modelo de raiz virtual | **M** | Introduzir distinção entre ficheiro do projecto e ficheiro de pacote (ou mapear pacote → `register_file` com path absoluto). Toca `World`/`FileId`. |
| 3. Cache local | **S–M** | Ler `~/.cache/typst/packages/{ns}/{name}/{ver}`; manifesto `typst.toml`; `entrypoint`. Reutiliza `register_file`. |
| 2. Download | **M–L** | Adicionar cliente HTTP a L3 (nova dependência — `native_tls` como o vanilla, ou `ureq`/`reqwest`), `tar` + `flate2` (flate2 já existe), escrita atómica tempdir→rename. I/O em L3 é permitido. |
| 4. Versão `latest` / índice | **S** | `GET index.json`, `max()` por nome; só relevante para specs sem versão. |
| Manifesto `typst.toml` | **S** | Parse TOML + validação (`name`, `version`, `compiler`). `toml` provavelmente já disponível; confirmar. |

**Sequência natural:** 5a → 5b/3 (offline, com pacotes já em cache) → 2 → 4. Os níveis 3+2+4 juntos equivalem a reimplementar `SystemPackages`/`UniversePackages` do vanilla em L3.

---

## 5. Proposta de divisão em passos menores

Seguindo o padrão de P614/P628 (funcionalidades grandes divididas):

1. **P-α — `#import` de ficheiros locais** (nível 5a). Pré-requisito absoluto. Implementar `eval_module_import` completo para caminhos relativos, com escopo de módulo, items/wildcard/rename e detecção de ciclos. Sem pacotes ainda. Critério: `#import "utils.typ": foo` funciona e é paridade com o vanilla.
2. **P-β — Pacotes só offline (níveis 5b + 3)**. Ler pacotes já presentes em `~/.cache/typst/packages` e na data dir; mapear `PackageSpec → directório → entrypoint → FileId`; parse de `typst.toml`. Critério: `#import "@preview/cetz:0.2.2": canvas, draw` compila usando a cache existente (sem rede).
3. **P-γ — Download e cache (nível 2)**. Adicionar cliente HTTP + `tar` a L3; replicar `GET /preview/{name}-{ver}.tar.gz`, tempdir→rename atómico em `~/.cache/typst/packages`. Critério: import de pacote não cacheado descarrega e funciona; segunda compilação usa a cache.
4. **P-δ — Versão omissa / `latest` (nível 4)**. `GET /preview/index.json`, `latest_version`, erro "please specify the desired version" para namespaces locais sem versão. Critério: `#import "@preview/cetz"` (sem versão) resolve a mais recente.

Cada passo fecha com relatório próprio e paridade medida contra o vanilla local em quarentena (que tem acesso de rede confirmado).

---

## 6. Decisão

Este passo não implementa nada. Produz o mapa acima e a proposta de divisão em quatro passos (α–δ). A implementação deve começar pelo nível 5a (`#import` de ficheiros locais), que é pré-requisito de tudo o resto e útil por si só — pacotes só ficam viáveis depois de `#import` existir.

---

## 7. Critério de fecho do passo

- [x] Mecanismo do vanilla mapeado (fonte dos pacotes, cache, resolução de versão) — §1.
- [x] Testado directamente se o vanilla local consegue mesmo descarregar — §2.1 (confirmado: rede disponível) e §2.2 (offline com cache).
- [x] Formato do cache local confirmado — §1.3, com evidência no filesystem.
- [x] Estado actual do cristalino confirmado, com mensagem de erro exacta — §2.3/§2.4.
- [x] Os cinco níveis mapeados, com estimativa de tamanho — §3/§4.
- [x] Confirmado quanto do `#import` existente pode ser reaproveitado — nível 5 **não** é reutilizável (stub); trabalho real começa em 5a.

---

## 8. Proveniência das medições

- **Commit base:** `73594ff67 — P677: adiciona hash do commit ao relatório`.
- **Hora da sonda:** 2026-07-10T14:24:51Z (working tree = commit base, sem alterações de código).
- **Vanilla usado:** `lab/typst-original/target/release/typst` — `typst 0.15.0 (969087ec)`.
- **Cristalino usado:** `./target/release/typst` (build do commit base).
- **Efeito colateral da sonda:** `@preview/tidy:0.3.0` foi descarregado para `~/.cache/typst/packages/preview/tidy/0.3.0` durante o teste de rede (§2.1). Nenhum ficheiro do repositório foi alterado.

---

## Hash do commit

`(a preencher após o commit)`

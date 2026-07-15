---
# P763a — Implementação: download automático de pacotes `@preview`

> **Passo:** 763a
> **Data:** 2026-07-15
> **Foco:** L0 fechado em P763 (`00_nucleo/prompts/infra/package_downloader.md`, hash `b6450b35`). A sonda confirmou: URL de download (`GET https://packages.typst.org/preview/<nome>-<versão>.tar.gz`), URL do índice (`GET https://packages.typst.org/preview/index.json`), estrutura de destino (`{cache-dir}/preview/{nome}/{versão}/`), mensagem exacta de erro de rede indisponível, e que o vanilla **não** verifica checksum/assinatura após mover o pacote para o destino final. Este passo implementa, sem reabrir nenhuma dessas decisões.
> **Tipo:** Implementação directa.
> **Tamanho:** L — rede em L3, extracção de `tar.gz`, escrita atómica, ligação ao `SystemWorld`.
> **ADR-0109 EM VIGOR** — `PackageDownloader` como trait em L1, cliente HTTP e sistema de ficheiros em L3, nenhum tipo de biblioteca HTTP atravessa a fronteira.
> **Dependências:** P763 (L0 e sonda fechados, hash `b6450b35`).

---

## Implementação

### Dependências novas em `03_infra`

Cliente HTTP síncrono (ex: `ureq`) e extracção de `tar.gz` (ex: `flate2` + `tar`). Confirmar antes de escolher se alguma já está na árvore de dependências por outro caminho (evitar duplicar crate de HTTP se já existir uma via `wasmi`/outra infra).

```bash
grep -n "ureq\|reqwest\|tar\b\|flate2" 03_infra/Cargo.toml
```

### Trait em L1

```rust
// 01_core/src/contracts/package_downloader.rs (caminho a confirmar contra a estrutura real)
pub trait PackageDownloader {
    fn download(&self, spec: &PackageSpec) -> Result<PathBuf, PackageDownloadError>;
}

pub enum PackageDownloadError {
    NotFound { spec: String },
    NetworkError { url: String, cause: String },
    IoError { path: PathBuf, cause: String },
}
```

Tipos de erro em L1; nenhum tipo de `ureq`/`tar`/`flate2` cruza a fronteira — a implementação em L3 converte para `PackageDownloadError`.

### Implementação em L3

`HttpPackageDownloader`, seguindo exactamente o que a sonda confirmou:

1. Construir a URL: `format!("{}/{}/{}-{}.tar.gz", base_url, "preview", spec.name, spec.version)`.
2. Descarregar para um directório temporário: `{cache-dir}/preview/{nome}/.tmp-{versão}-{rand}/`.
3. Extrair o `tar.gz` para esse directório temporário.
4. Rename atómico do directório temporário para `{cache-dir}/preview/{nome}/{versão}/`.
5. **Não** verificar checksum nem assinatura (decisão registada no L0 — paridade de defeitos, regra 4 do handoff).
6. Traduzir erros de rede/IO para as mensagens exactas confirmadas pela sonda:
   - Rede indisponível: replicar o formato `failed to download package (<url>: Connection Failed: ...)`.
   - Pacote inexistente no índice remoto.

### Ligação a `SystemWorld::resolve_package`

`03_infra/src/world.rs:430` — quando o pacote não está na cache local e `spec.namespace == "preview"`, invocar `HttpPackageDownloader::download` antes de reportar erro.

---

## Validação

```bash
# Reproduzir o caso da sonda com cache limpa
rm -rf ~/.cache/typst/packages/preview/fletcher/0.5.4
cat > /tmp/p763a-test.typ <<'EOF'
#import "@preview/fletcher:0.5.4": diagram
EOF
./target/release/typst compile /tmp/p763a-test.typ /tmp/p763a-out.pdf
```

Esperado: compila sem intervenção manual, pacote gravado na mesma estrutura observada no vanilla.

```bash
# Caso de rede indisponível — confirmar mensagem de erro
https_proxy=http://127.0.0.1:1 ./target/release/typst compile /tmp/p763a-test.typ /tmp/p763a-out2.pdf
```

```bash
cargo test --workspace
crystalline-lint .
```

---

## Critério de fecho do passo

- [ ] Trait `PackageDownloader` em L1, sem tipos externos vazados.
- [ ] `HttpPackageDownloader` em L3: download, extracção, escrita atómica (temp dir + rename).
- [ ] Ligado a `SystemWorld::resolve_package`.
- [ ] Mensagens de erro replicadas e testadas uma a uma (rede indisponível, pacote inexistente) contra as confirmadas por P763.
- [ ] Documento real (`@preview/fletcher:0.5.4` ou outro) compila com cache vazia, sem intervenção manual.
- [ ] `cargo test --workspace` verde.
- [ ] `crystalline-lint .` zero violações.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p763a.md`, com hash do commit.

---

## Próximo passo

Validar `cetz` (ou outro pacote com dependências `@preview` encadeadas) num ambiente com cache totalmente vazia, de ponta a ponta, sem pré-popular manualmente — teste de aceitação real da funcionalidade fechada aqui.

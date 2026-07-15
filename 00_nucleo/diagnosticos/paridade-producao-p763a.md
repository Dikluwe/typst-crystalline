# Relatório de Paridade — P763a

> **Passo:** 763a  
> **Data:** 2026-07-15T15:36:14-03:00  
> **Commit base:** `5469d8d1e12f0bb77732b22b6dd1844312b3f282`  
> **L0:** `00_nucleo/prompts/infra/package_downloader.md` (hash `b6450b35`)

---

## Resumo

Implementação do download automático de pacotes `@preview` no cristalino. O L0 correspondente foi fechado em P763; este passo limitou-se a materializar a infraestrutura sem reabrir decisões.

## Estado do critério de fecho

- [x] Trait `PackageDownloader` em L1, sem tipos externos vazados.
- [x] `HttpPackageDownloader` em L3: download, extracção, escrita atómica (temp dir + rename).
- [x] Ligado a `SystemWorld::resolve_package`.
- [x] Mensagens de erro replicadas e testadas (rede indisponível, pacote inexistente, versão inexistente).
- [x] Documento real (`@preview/fletcher:0.5.4`) compilou com cache vazia.
- [x] `cargo test --workspace` verde.
- [x] `crystalline-lint .` — zero erros; apenas warning V7 esperado no L0 de `package_version_resolution.md` (backlog P764a, sem código).

## Ficheiros alterados

- `01_core/src/contracts/package_downloader.rs` — trait + tipos de erro.
- `01_core/src/contracts/mod.rs` — re-export do trait.
- `03_infra/src/package_downloader.rs` — `HttpPackageDownloader`.
- `03_infra/src/world.rs` — ligação em `resolve_package`.
- `03_infra/Cargo.toml`, `Cargo.toml` — dependências `ureq`, `tar`, `fastrand`.
- `03_infra/src/lib.rs` — registo do módulo.

## Validação real

```bash
rm -rf ~/.cache/typst/packages/preview/fletcher/0.5.4
cat > /tmp/p763a-test.typ <<'EOF'
#import "@preview/fletcher:0.5.4": diagram
EOF
./target/release/typst /tmp/p763a-test.typ /tmp/p763a-out.pdf
```

Resultado: compilação sem intervenção manual; pacote gravado em `~/.cache/typst/packages/preview/fletcher/0.5.4/`.

Casos de erro testados:
- Pacote inexistente: mensagem de `NotFound` replicada.
- Versão inexistente: distinção via `index.json` replicada.
- Rede indisponível via `https_proxy=http://127.0.0.1:1`: mensagem de `Connection Failed` replicada.

## Próximo passo

Validar `cetz` (ou outro pacote com dependências `@preview` encadeadas) num ambiente com cache totalmente vazia, de ponta a ponta.

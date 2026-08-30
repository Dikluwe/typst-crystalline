# P1188 — separar `info` L2/L3 e nuclear a projeção sanitizada

**Data:** 2026-08-25
**Estado:** `EXECUTADO — GREEN EM 2026-08-25`
**Dependências:** P1187 GREEN e primeiro Núcleo V26 válido
**Classe ADR-0127:** linhagem/norma interna; contrato público de `typst info` inalterado

## Objetivo e classificação

Resolver:

```text
shell/info.md
├── 02_shell/src/info.rs
└── 03_infra/src/runtime_info.rs
```

L2 define DTO e serialização humana/JSON (hash `d2942c4d`); L3 captura paths,
env allowlisted e presença de certificado (hash `16249cb`). O contrato comum é
a projeção sanitizada L3→L2: mesmas categorias, sem segredos, sem download ou
varredura. Isso justifica Núcleo. Baseline condicionado: V15=19, V26=0,
V5=411.

## L0 e Núcleo primeiro

1. Criar `_nuclei/shell/info-projection.toml` com claims `must`/`must-not`:
   snapshot read-only; allowlist explícita; valores de proxy/certificado nunca
   expostos; presença sensível pode ser booleano; serializações representam as
   mesmas categorias sem exigir igualdade de paths entre máquinas.
2. Atualizar `shell/info.md` como owner exclusivo de `02_shell/src/info.rs`:
   DTOs, kebab-case, JSON newline/pretty, saída humana e truncamento somente
   humano. Remover ownership declarado de cli/runtime/main e pô-los em
   scope-out. Pinar o Núcleo.
3. Criar `infra/runtime_info.md` como owner exclusivo de
   `03_infra/src/runtime_info.rs`: resolução HOME/XDG, font paths, allowlist e
   `custom_cert_configured`, sem rede/scan. Pinar o mesmo Núcleo.
4. Não tocar `shell/cli.md` nem `wiring.md`; esses continuam donos de parsing e
   composição. Nenhum contrato público ou default muda.

Seguir a ordem Núcleo→pins→Hash do Código→hash integral dos prompts→headers.
V26 deve validar schema, dois consumers e pins completos antes do resselo V5.

## GREEN

Exigir V15 19→18, V26=0, V5 esperado 411→409, sources focais ausentes de V5 e
byte-idênticos fora da linhagem. `info.rs` mantém seu path; `runtime_info.rs`
reponta para `infra/runtime_info.md`. Dry-run bloqueado pelas 18 V15 restantes
e zero writes.

Executar `cargo test -p typst-shell info`, `cargo test -p typst-infra runtime_info`,
com fallback para a crate se algum filtro for vazio; depois `cargo build`, diff
limpo e índice vazio. Fechar em
`00_nucleo/diagnosticos/typst-p1188-saneamento-info-owners.md`.

## Próximo passo

P1189 separa `label_kind` do layout de referências e nucleariza sua tradução.

# Prompt L0 — features do compilador
Hash do Código: 3ed1ce25

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/compiler-feature-gates.toml sha256:59d8938dc06d347ccc9db23ae1b740876b369227daacd266a219811a661b3cb9

**Camada:** L1
**Ficheiro alvo exclusivo:** `01_core/src/entities/compiler_features.rs`
**Vanilla ratificado:** `586e1bd43`
**ADRs:** ADR-0029, ADR-0107, ADR-0108, ADR-0127, ADR-0128, ADR-0129

## Medição histórica P1288 anterior à decisão

- `01_core/src/entities/html.rs:12-42` mede que `Feature` e `Features` ainda
  pertencem fisicamente ao owner HTML e representam somente `Html`.
- `01_core/src/entities/mod.rs:59` expõe `entities::html` publicamente; os
  consumers atuais usam `entities::html::{Feature, Features}` em
  `compiler/eval/mod.rs:119`, `02_shell/src/cli.rs:82-91`,
  `03_infra/src/pipeline.rs:77-82` e `04_wiring/src/main.rs:374`.
- A fonte vanilla pinada mede `Feature::{Html, Bundle, A11yExtras}` em
  `lab/typst-original/crates/typst-library/src/lib.rs:272-305`. O recorte
  P1288 materializou `Html` e `A11yExtras`; P1484 transporta `Bundle`.
- Inferência marcada: o path público anterior pode já ser consumido fora do
  workspace. Um inventário de dependentes externos que demonstre ausência de
  uso refutaria a necessidade do re-export, mas não existe nesta fase.

## P1484 — medição anterior à decisão (gate ADR-0127 confirmado)

Sobre HEAD `bde47c744e0bffb6229005e4b5f04c2ed0e9ef08`, sem diff
rastreado, `01_core/src/entities/compiler_features.rs:7-32` contém
apenas duas variantes e dois bits. O vanilla ratificado `586e1bd43`,
`lab/typst-original/crates/typst-library/src/lib.rs:283-306`, contém
`Feature::Bundle` independente. A enum Rust e os bits são mecânica
interna; o set efetivo por perfil é semântica pública. A hipótese de
que a flag já basta é refutada por `02_shell/src/cli.rs:182`, que a
descarta. Adicionar a variante pública exige confirmação antes de L1.

## Decisão P1484 (confirmada pelo dono em 2026-09-23)

O owner define os dados públicos puros e fechados:

```rust
pub enum Feature {
    Html,
    Bundle,
    A11yExtras,
}

pub struct Features { /* representação interna livre */ }
```

`Features::default()` e `Features::empty()` representam o conjunto vazio.
`contains` consulta uma feature e `enable` a acrescenta idempotentemente;
ordem e repetição não alteram o conjunto. `Html`, `Bundle` e `A11yExtras` são
independentes. P1484 acrescenta somente o carrier e transporte de `Bundle`;
o constructor `asset(path, data)` e o target bundle permanecem incompletos até
P1485. Nem o formato de saída nem outra feature ativam `Bundle` implicitamente.

O tipo é domínio puro: sem I/O, env, target, formato, exporter, diagnóstico
ou estado global mutável. Nenhum constructor ativa feature por default.

## Compatibilidade e ownership

`entities::compiler_features::{Feature, Features}` é o path canônico novo.
Como o path antigo é público no workspace medido, `entities::html` preserva
compatibilidade por re-export explícito dos dois tipos; não mantém cópia,
wrapper nem segundo set. `entities/html.rs` continua exclusivamente dono de
`HtmlElem`, `HtmlBody` e `HtmlAttrs`.

Este prompt possui exclusivamente `01_core/src/entities/compiler_features.rs`.
O re-export em `entities/html.rs` preserva compatibilidade sem criar segundo
owner ou segunda identidade de tipo.

## Aceitação

- default/empty não contêm `Html`, `Bundle` nem `A11yExtras`;
- habilitar uma não habilita as outras duas;
- repetição e ordem produzem o mesmo conjunto;
- o path canônico é `entities::compiler_features` e o path HTML antigo é
  somente re-export da mesma identidade de tipo;
- nenhum target ou formato participa da entidade;
- L1 permanece puro e o linter prova um único Prompt L0 proprietário para o
  consumer.

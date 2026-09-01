# Prompt L0 — features do compilador
Hash do Código: 9dbe99eb

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/compiler-feature-gates.toml sha256:59d8938dc06d347ccc9db23ae1b740876b369227daacd266a219811a661b3cb9

**Estado:** PROPOSTO — PARAGEM OBRIGATÓRIA ADR-0127; não materializável até
confirmação humana.
**Camada:** L1
**Owner futuro exclusivo:** `01_core/src/entities/compiler_features.rs`
**Vanilla ratificado:** `a51e02804`
**ADRs:** ADR-0029, ADR-0107, ADR-0108, ADR-0127, ADR-0128, ADR-0129

## Medição anterior à decisão

- `01_core/src/entities/html.rs:12-42` mede que `Feature` e `Features` ainda
  pertencem fisicamente ao owner HTML e representam somente `Html`.
- `01_core/src/entities/mod.rs:59` expõe `entities::html` publicamente; os
  consumers atuais usam `entities::html::{Feature, Features}` em
  `compiler/eval/mod.rs:119`, `02_shell/src/cli.rs:82-91`,
  `03_infra/src/pipeline.rs:77-82` e `04_wiring/src/main.rs:374`.
- A fonte vanilla pinada mede `Feature::{Html, Bundle, A11yExtras}` em
  `lab/typst-original/crates/typst-library/src/lib.rs:272-305`. O recorte
  P1288 materializa `Html` e `A11yExtras`; `Bundle` permanece fora do escopo.
- Inferência marcada: o path público anterior pode já ser consumido fora do
  workspace. Um inventário de dependentes externos que demonstre ausência de
  uso refutaria a necessidade do re-export, mas não existe nesta fase.

## Decisão proposta após o gate humano

O owner futuro define os dados públicos puros e fechados:

```rust
pub enum Feature {
    Html,
    A11yExtras,
}

pub struct Features { /* representação interna livre */ }
```

`Features::default()` e `Features::empty()` representam o conjunto vazio.
`contains` consulta uma feature e `enable` a acrescenta idempotentemente;
ordem e repetição não alteram o conjunto. `Html` e `A11yExtras` são
independentes. `Bundle` não é variante aceita por este recorte e continua
scope-out explícito, nunca inferido da infraestrutura HTML.

O tipo é domínio puro: sem I/O, env, target, formato, exporter, diagnóstico
ou estado global mutável. Nenhum constructor ativa feature por default.

## Compatibilidade e ownership

`entities::compiler_features::{Feature, Features}` é o path canônico novo.
Como o path antigo é público no workspace medido, `entities::html` preserva
compatibilidade por re-export explícito dos dois tipos; não mantém cópia,
wrapper nem segundo set. `entities/html.rs` continua exclusivamente dono de
`HtmlElem`, `HtmlBody` e `HtmlAttrs`.

Este prompt e `compiler/eval/table.md` são entradas ex-ante pré-gate sem
consumer presente por imposição da Fase C4. Não alegam ownership materializado.
Após confirmação, criar exatamente o consumer acima, remover qualquer exceção
transitória de órfão e ressellar a relação 1:1 antes de novo código dependente.

## Aceitação pós-confirmação

- default/empty não contêm `Html` nem `A11yExtras`;
- habilitar uma não habilita a outra;
- repetição e ordem produzem o mesmo conjunto;
- o path canônico é `entities::compiler_features` e o path HTML antigo é
  somente re-export da mesma identidade de tipo;
- nenhum target ou formato participa da entidade;
- L1 permanece puro e o linter prova um único Prompt L0 proprietário para o
  consumer.

# Diagnóstico: estado de autorização de `ecow` em L1

**Escopo**: verificação factual do alcance da ADR-0024 face aos tipos
da crate `ecow` (`EcoString`, `EcoVec`, `EcoMap`).
**Data**: 2026-04-22
**Contexto**: Passo 87 — verificação que precede a materialização de
`Sink` e `Styles` (identificados no Passo 86 como bloqueados por
`EcoVec`).

**Natureza**: registo factual do texto do ADR, da configuração do
linter e do uso actual em código. Este ficheiro não decide — apenas
classifica o estado e enuncia a implicação directa.

---

## 1. Texto literal do escopo do ADR-0024

Título: **"`ecow` → `[l1_allowed_external]` para `Value::Str`"**
(`00_nucleo/adr/typst-adr-0024-ecow-value.md:1`).

Secção "Decisão" (linhas 46–64):

> `ecow` é adicionado a `[l1_allowed_external]`:
> `"ecow",  # ADR-0024 — EcoString em Value::Str; clone O(1) no hot path de eval()`
> (…) `Value::Str` usa `EcoString`. (…) `EcoString` não entra nas
> assinaturas públicas de L1 além de `Value::Str`.

Secção "O que esta ADR não decide" (linhas 96–99):

> - Outros usos de `ecow` além de `Value::Str` — avaliar caso a caso
> - `EcoVec` para colecções em `Value::Array` — ADR separada quando
>   Array migrar

Leitura: a ADR autoriza `EcoString` para um caso nominado (`Value::Str`)
e declara explicitamente que `EcoVec` e outros usos ficam fora do
escopo, dependendo de ADR adicional.

---

## 2. Configuração do linter

`crystalline.toml:64–82`:

```toml
[l1_allowed_external]
rust = [
    ...
    "ecow",   # ADR-0024 — EcoString em Value::Str; clone O(1) no hot path de eval()
]
```

`01_core/Cargo.toml:21`:
```toml
ecow = { workspace = true }  # ADR-0024 — EcoString em Value::Str
```

Granularidade: **crate-level**. O whitelist lista nomes de crate, não
tipos. Isto significa que o linter aceita `use ecow::EcoVec` com a
mesma passividade com que aceita `use ecow::EcoString` — a restrição
pontual do ADR-0024 não é enforced mecanicamente. O comentário do
whitelist ancora a autorização em `EcoString`/`Value::Str`, mas é
documentação, não regra executável.

---

## 3. Uso actual em `01_core/src/`

Comando: `grep -rn "EcoString\|EcoVec\|EcoMap\|ecow::" 01_core/src/ --include="*.rs"`.

Uso em **código** (declarações, expressões, assinaturas):

| Tipo | Ocorrências | Ficheiros |
|------|-------------|-----------|
| `EcoString` | frequente (~40+) | `entities/{value,content,args,layout_types}.rs`, `rules/{eval,stdlib,layout/mod,math/layout}.rs` |
| `ecow::EcoString` (qualificado) | 3 | `rules/math/layout.rs:829,856,872` |
| `EcoVec` | **0 em código** | — |
| `EcoMap` | **0** | — |

Ocorrências de `EcoVec` aparecem exclusivamente em **comentários de
divergência** que documentam o tipo vanilla correspondente:
`entities/source_result.rs:37,85`, `entities/world_types.rs:137,172`,
`entities/layout_types.rs:368`. Nenhuma é `use` ou declaração de tipo.

Conclusão factual: o código actual usa **só `EcoString`**.

---

## 4. Classificação

**A — Pontual explícita.**

Justificação: o ADR-0024 declara o escopo em termos literais ("para
`Value::Str`") e nomeia explicitamente `EcoVec` como fora do escopo
("ADR separada quando Array migrar"). O código respeita esse escopo —
nenhum ficheiro de L1 importa ou usa `EcoVec`. A ambiguidade que
caracterizaria C (código/linter a permitir mais do que o ADR declara)
não se materializa no código; existe apenas como gap mecânico no
linter (whitelist crate-level), não como prática instalada.

Nota: o gap mecânico é consequência do desenho do `crystalline.toml`
(whitelist por crate, não por tipo). Não é ambiguidade no ADR.

---

## 5. Implicação directa para `Sink` e `Styles`

`EcoVec` **não pode ser usado em L1 sem ADR adicional** — seja
extensão ao ADR-0024 ou ADR novo que estenda a autorização a `EcoVec`
(campos de `Sink`, `Styles`, `Value::Array` futuro). O ADR-0024
antecipa este momento ao declarar "`EcoVec` (…) ADR separada quando
Array migrar", pelo que o caminho natural é um ADR de extensão antes
dos passos de materialização que tocam `Sink` ou `Styles`.

---

## Referências

- `00_nucleo/adr/typst-adr-0024-ecow-value.md:1,46–64,96–99`
- `00_nucleo/adr/typst-adr-0018-rustc-hash.md` — critério de pureza
  funcional (não de origem) que informa qualquer extensão.
- `crystalline.toml:64–82` — whitelist crate-level.
- `01_core/Cargo.toml:21` — declaração da dependência.
- `00_nucleo/diagnosticos/diagnostico-stubs-comemo-passo-86.md:189–192,200–207,300–301`
  — identificação de `EcoVec` como bloqueador de `Sink`/`Styles`.

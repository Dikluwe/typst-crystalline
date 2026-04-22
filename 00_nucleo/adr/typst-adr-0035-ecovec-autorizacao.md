# ⚖️ ADR-0035: `ecow::EcoVec` autorizado em L1

**Status**: `EM VIGOR`
**Data**: 2026-04-22

---

## Contexto

A ADR-0024 autorizou `ecow::EcoString` em L1 de forma **pontual**,
ligada ao uso em `Value::Str`. A secção "O que esta ADR não decide"
da ADR-0024 registou explicitamente duas limitações:

> - Outros usos de `ecow` além de `Value::Str` — avaliar caso a caso
> - `EcoVec` para colecções em `Value::Array` — ADR separada quando
>   Array migrar

Desde então duas coisas mudaram:

1. A série de materializações dos stubs `#[comemo::track]` iniciada no
   Passo 88 (`Traced`) prossegue para `Sink` e `Styles`. A forma
   vanilla destes tipos usa `EcoVec` como contentor primário:
   - `Sink` em `lab/typst-original/crates/typst-library/src/engine.rs:150`
     — `EcoVec<Introspection>`, `EcoVec<SourceDiagnostic>`,
     `EcoVec<(Value, Option<Styles>)>`.
   - `Styles` em `lab/typst-original/crates/typst-library/src/foundations/styles.rs:24`
     — `EcoVec<LazyHash<Style>>`.

2. O diagnóstico do Passo 87
   (`00_nucleo/diagnosticos/diagnostico-ecow-autorizacao-passo-87.md`)
   classificou o estado actual como **A (pontual explícita)**: o ADR
   declara escopo restrito, o código respeita, mas o linter (whitelist
   crate-level) não enforça type-by-type. Materializar `Sink`/`Styles`
   sem ADR adicional introduziria `EcoVec` em L1 em divergência
   explícita com a limitação declarada pelo ADR-0024.

Esta ADR estende a autorização, tornando `EcoVec` legítimo em L1 sem
alterar o carácter pontual da ADR-0024 (que continua `IMPLEMENTADO`
para `EcoString`). Não é revogação — é extensão por ADR novo, como a
ADR-0024 previu.

---

## Análise de pureza (ADR-0018)

| Propriedade | Estado |
|-------------|--------|
| Zero I/O | ✓ — estrutura de dados em memória |
| Zero estado global mutável | ✓ |
| Determinismo total | ✓ |
| Clone O(1) para vectores não mutados | ✓ — copy-on-write via refcount |
| Sem dependências externas novas | ✓ — `ecow` já em `01_core/Cargo.toml` |

`EcoVec` satisfaz o critério da ADR-0018 (pureza funcional) pelos
mesmos motivos que `EcoString`: ambos são vectores/strings de
refcount com semântica CoW, e a crate `ecow` é mantida pela equipa
do Typst vanilla, sem I/O interno.

---

## Decisão

`ecow::EcoVec<T>` é autorizado em L1, sem restrição a tipo nominado
específico, para colecções de tipos que já satisfazem as regras de L1
(tipos de L1, ou tipos externos autorizados pelas próprias ADRs que
cobrem esses externos).

Escopo da decisão:

- **Coberto**: `ecow::EcoVec<T>` em campos de struct, assinaturas de
  métodos internos a L1, e tipos de retorno internos — sempre com
  `T` já admissível em L1.
- **Não coberto**: `ecow::EcoMap`, `ecow::EcoArc`, ou outros tipos da
  crate `ecow`. Cada um exige extensão futura ou ADR próprio quando
  surgir necessidade documentada.
- **Não coberto**: usos de `EcoVec` com `T` que viole regras de L1
  (ex: `T` com I/O). A regra de composição continua a valer — a
  autorização é sobre o tipo contentor, não sobre o conteúdo.

A ADR-0024 **não é revogada**. Permanece `IMPLEMENTADO` para
`EcoString` em `Value::Str`. Esta ADR coexiste como extensão: uma
trata do caso `EcoString` concreto (ponto de clone O(1) em eval),
outra trata do caso `EcoVec` genérico (contentor para push-only sinks
e estilos).

---

## Alternativas Consideradas

| Alternativa | Razão rejeitada |
|-------------|-----------------|
| `Vec<T>` da stdlib | Perde clone-on-write; clones em APIs que passam colecções tornam-se O(n). `Sink::clone()` no vanilla é barato precisamente por causa do `EcoVec`. |
| `Arc<[T]>` | Imutável após criação. `Sink` é push-only — precisa de `push`/`extend` com mutação. `EcoVec` permite mutação via CoW sem perder partilha. |
| Tipo custom de L1 com semântica CoW | Reinventar a roda; `EcoVec` já é o contentor canónico no vanilla, testado e mantido. Divergência sem benefício arquitectural. |
| Não autorizar; escrever ADR por cada uso (`Sink`, `Styles`, `Value::Array`, etc.) | Processo ruidoso. O escopo genérico de `EcoVec` é justificável de uma vez — a análise de pureza é a mesma em todos os usos. |
| Revogar ADR-0024 e absorver nesta | Perda de rastreabilidade. ADR-0024 documenta decisão histórica ligada a `Value::Str`; é registo válido. Extensão preserva história. |

---

## Consequências

**Positivas**:

- `Sink` e `Styles` podem replicar a forma vanilla sem divergência
  estrutural — paridade directa (ADR-0033).
- Clones O(1) em APIs que passam colecções. Consistente com a escolha
  já feita para `EcoString` em eval.
- Um único ADR cobre todos os usos futuros de `EcoVec` em L1, em vez
  de sequência de ADRs pontuais.

**Negativas**:

- Enforcement automático **ainda não existe**. O linter continua com
  whitelist crate-level (`ecow` como nome da crate), não type-level.
  A disciplina de respeitar as fronteiras entre tipos autorizados
  (`EcoString`, `EcoVec`) e não autorizados (`EcoMap`, `EcoArc`)
  depende de revisão humana. Este gap é registado como **DEBT-43**.

**Neutras**:

- Não muda `01_core/Cargo.toml` — `ecow` já estava listada desde a
  ADR-0024.
- Não altera comportamento observável de nenhum código actualmente
  em L1 — é autorização, não materialização. Os stubs `Sink`/`Styles`
  continuam `Tipo(())` até passos de construção dedicados.

---

## Referências

- ADR-0018 — critério de autorização externa (pureza funcional, não
  origem).
- ADR-0024 — autorização pontual de `EcoString` em `Value::Str`; esta
  ADR estende sem revogar.
- ADR-0029 — pureza física de L1; RAM/refcount é domínio, não I/O.
- ADR-0033 — paridade funcional com vanilla; motivação para replicar
  `EcoVec` em `Sink`/`Styles`.
- Passo 86 —
  `00_nucleo/diagnosticos/diagnostico-stubs-comemo-passo-86.md`
  identificou `EcoVec` como dependência bloqueadora de
  `Sink`/`Styles`.
- Passo 87 —
  `00_nucleo/diagnosticos/diagnostico-ecow-autorizacao-passo-87.md`
  classificou o estado anterior como A (pontual) e motivou esta ADR.
- DEBT-43 — gap de enforcement type-level no linter
  (`00_nucleo/DEBT.md`).
- `ecow` (crate): https://github.com/typst/ecow

# ADR-0111 — Autorização de crates de parsing para o módulo `loading` (data import)

**Estado:** `IMPLEMENTADO` (Passo 387 — módulo `loading` materializado; `cargo build --workspace` + `crystalline-lint .` verdes; 13 testes de decode L1 passam).
**Decisão do dono (registada):** escolhas por formato confirmadas — **yaml = `saphyr`** (parser YAML **genuinamente mantido**; mapa `Yaml → Value` manual, sem serde) após apurar-se que **tanto `serde_yaml` (arquivada) como `serde_yml` (shim DEPRECATED na crates.io) estão não-mantidas** (ADR-0108: a fonte refutou o enquadramento "fork mantido"); **6 formatos materializados, zero parciais por omissão**; `read` modo texto (`Str`) apenas, binário deferido; cbor byte-strings graded documentado. `Value::Bytes` registado como **DEBT-62** (ver `00_nucleo/DEBT.md`) para passo dedicado de modelagem de tipos. Materialização do módulo `loading` segue o L0 `00_nucleo/prompts/rules/stdlib/loading.md`.
**Histórico de numeração:** `00_nucleo/adr/` varrido — 0110 ocupada (marcador `@vanilla`, P385); **0111 é o primeiro livre** (precedente P160A/P376: nunca assumir número).
**ADRs relacionadas:** ADR-0029 (pureza L1 / zero I/O), ADR-0107 (paridade é com a língua — o `Value` de saída, não a árvore do parser), ADR-0054 (perfil graded), ADR-0017 (`Value::Bytes` ausente — prerequisito graded). Precedentes de autorização de crate: ADR-0018 (rustc_hash), ADR-0023 (indexmap), ADR-0024 (ecow), ADR-0057 (hypher), ADR-0077 (regex).

---

## Contexto

A Lista B do Passo 386 isolou o cluster `loading` (data import: `read`/`csv`/`json`/`yaml`/`toml`/`cbor`/`xml`) como a única dívida user-facing **não catalogada** e confirmada ausente em L1 — e bloqueante de `bibliography` (a maior dívida Model). O Passo 387 materializa-o.

O decode é **L1 puro** (bytes→`Value`; ADR-0029). Parsing exige crates externas, que entram em `[l1_allowed_external]` de `crystalline.toml` — exige ADR (V14 `ExternalTypeInContract`). Esta ADR autoriza **por formato, não em bloco**.

## Decisão

Autorizar em `[l1_allowed_external].rust` as crates abaixo. O critério de escolha foi **paridade de saída (ADR-0107)** — matchar a crate do vanilla dá o mesmo `Value` de saída — somado a **pureza** (sem I/O próprio, sem C/build-script frágil) e **manutenção viva**.

| Formato | Crate | Vanilla usa? | Pureza | Nota de decisão |
|---------|-------|:---:|--------|-----------------|
| json | `serde_json` | ✓ | pura, sem I/O | padrão de facto; paridade direta |
| yaml | **`saphyr`** | ✗ (vanilla usa `serde_yaml`) | pura, **mantida** | **Decisão do dono:** `saphyr` (parser YAML mantido), após a fonte refutar `serde_yml` ("DEPRECATED" na crates.io) e `serde_yaml` (arquivada). Sem serde: mapa `saphyr::Yaml → Value` **manual** no `decode_yaml`. Paridade de saída **provada por testes de bytes literais** (§7 do L0) — independente da crate (ADR-0107). Custo: ~mapa explícito vs serde, em troca de dep viva. |
| toml | `toml` | ✓ | pura | a mesma que o Cargo usa; já provável no grafo |
| cbor | `ciborium` | ✓ | pura, **puro-Rust sem C** | sem byte-strings na 1ª fase (§graded) |
| xml | `roxmltree` | ✓ (read) | pura, read-only, sem alloc pesada | `xmlwriter` (write) do vanilla **não** é necessário (só lemos) |
| csv | `csv` | ✓ | pura | **avaliado parser próprio L1**: rejeitado para a 1ª fase — a crate `csv` dá paridade exata de edge-cases (quoting, embedded newlines) que um parser próprio reabriria; o custo de dep é baixo e a crate é pura. Reavaliar se a dep trouxer peso. |
| (comum) | `serde` | ✓ | pura | dependência-base de `serde_json`; sem derive em tipos L1 de contrato |

**Regra de fronteira (V14):** nenhum tipo de parser (`serde_json::Value`, `toml::Value`, `roxmltree::Document`, …) aparece em **contrato público L1**. As funções de decode devolvem `Value` cristalino; a conversão parser→`Value` é interna ao módulo `loading`. Isto preserva a topologia (o parser é detalhe de implementação, não tipo de domínio).

## Consequências

- **Positivas.** Fecha a lacuna `loading`; destrava `bibliography`/`cite` (cluster Model XL); o decode puro fica diffável contra o vanilla sem disco (ADR-0107). Paridade de saída por matchar as crates do vanilla.
- **Custos.** +6 crates (+serde) em L1 — todas puras; aumentam a superfície de dependência de L1. Justificado: parsing correto de formatos padrão não é candidato a reimplementação própria (exceto onde avaliado, csv).
- **Risco YAML não-mantido — resolvido.** Apurou-se que `serde_yaml` (arquivada) **e** `serde_yml` (DEPRECATED na crates.io) estão não-mantidas. Decisão: `saphyr`, parser mantido, com mapa manual `Yaml → Value`. A paridade do `Value` de saída é garantida pelos testes de bytes literais, independentes da crate.

## Alternativas consideradas

- **Parser próprio L1 para todos os formatos.** Rejeitada: reabre edge-cases (quoting csv, números yaml, datas toml) que as crates resolvem; viola "paridade é com a língua, não com a mecânica" ao gastar esforço em reimplementar mecânica de parsing sem ganho de paridade.
- **Autorização em bloco ("crates de parsing").** Rejeitada (passo §5): cada formato é uma decisão de pureza/manutenção própria; bloco esconde o risco do yaml.
- **Decode em L3 (junto da leitura).** Rejeitada: tiraria o decode do estrato puro/testável e mataria o diff-contra-vanilla sem disco (o ganho central, ADR-0029).

## Política "sem novas reservas"

Esta ADR autoriza as crates e fixa a regra de fronteira; **não** reserva trabalho além da materialização do `loading` (L0 dedicado). `Value::Bytes` (prerequisito do modo binário de `read` e de byte-strings cbor) é dívida Lista A **independente**, registada como **DEBT-62** para passo dedicado de modelagem de tipos. Promoção a `IMPLEMENTADO` quando o módulo materializar e `crystalline-lint` passar.

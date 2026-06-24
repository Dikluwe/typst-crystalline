# Mapa de migração vanilla ↔ cristalino — snapshot P384

**Tipo:** medição mecânica (lente DSM `tekt-cargo-dsm`), read-only. **Data:** 2026-06-20.
**HEAD:** `9caefa004` (pós-P384). **Lente:** `~/.cargo/bin/lente --comparar` (fonte em
`~/Documentos/Antigravity/tekt-cargo-dsm`).

> Este é um **snapshot pontual** do P384, não o documento-mestre. O mestre é
> `mapa-migracao-vanilla-cristalino.md` (com a coluna *declarada* curada à mão). Aqui só vive a
> **parte mecânica** desta corrida, para servir o pedido "comparar o que temos com o vanilla no lab".

## Como foi medido (reprodutível)
```
ln -sf Cargo.toml.original lab/typst-original/Cargo.toml
RUST_MIN_STACK=33554432 lente --comparar --antes lab/typst-original --depois . > /tmp/comparar-typst-itens.json
rm -f lab/typst-original/Cargo.toml          # removido SEMPRE (símlink temporário; árvore limpa)
```
- **antes** = `lab/typst-original` (vanilla, `typst-original`); **depois** = raiz (`typst-crystalline`).
- escopo `seu-codigo`; chave de pareamento `path_completo` (nome normalizado na raiz do crate).
- corrida limpa (exit 0, sem `stderr`); 2 falhas de resolução do lado vanilla (`typst-macros`
  colisão inconclusiva; `typst-tests` sem alvo analisável) — não afetam o censo de itens.

## Manchete — estrutura (o sinal mais fiável da reescrita Tekt)

| Métrica | Vanilla | Cristalino | Δ |
|---|---:|---:|---|
| **Crates** | **21** | **4** | −17 (consolidação) |
| Deps third-party | 434 | 40 | ~11× menos |
| **Ciclos de dependência** | 11 (maior **203** nós) | **4** (maior **95**) | menos e menores |

Os **4 crates** cristalinos: `typst_core` (L1), `typst_infra` (L3), `typst_shell` (L2),
`typst_bundle` (L4). Os **21** vanilla incluem `typst_library`, `typst_syntax`, `typst_layout`,
`typst_pdf`, `typst_html`, `typst_svg`, `typst_render`, `typst_ide`, `typst_docs`, `typst_eval`,
`typst_realize`, `typst_kit`, `typst_utils`, … A reescrita **colapsou** a árvore de 21 para 4 e
cortou os ciclos quase a metade — o objetivo arquitetural Tekt, medido.

## Censo de itens (pareamento por nome — portão laudo 0078)

| Conjunto | Nº | Portão esperado | Estado |
|---|---:|---:|---|
| **Pareados** (vanilla↔cristalino) | **1625** | ~1474 | ✓ acima |
| Ambíguos (vários candidatos no cristalino) | 113 | — | ex.: `Assoc` → `ast::expr` **e** `operators` |
| **Só-vanilla** (sem par) | **10745** | ~10910 | ✓ na ordem |
| Só-cristalino (novo/específico) | 1717 | ~1203 | acima (núcleo consolidado) |

**Kinds dos 1625 pareados:** fn 1388 · struct 172 · enum 38 · type 22 · trait 4 · macro 1.
**Destino cristalino dos pareados:** `typst_core` 1618 · `typst_infra` 7 — quase tudo aterrou no
núcleo L1. Exemplo de par: `typst_syntax::ast::Arg` → `typst_core::entities::ast::expr::Arg`.

## Mecânica por crate vanilla (pareado | só-vanilla | % pareado)

| crate vanilla | pareados | só-vanilla | total | % | leitura |
|---|--:|--:|--:|--:|---|
| `typst_syntax` | 1060 | 267 | 1327 | **80%** | parser/AST **genuinamente migrado** para `typst_core::entities` |
| `typst_library` | 545 | 7876 | 8421 | **6%** | stdlib — subconjunto implementado; muitos `*Elem`→variant **escondem** migrados (ver limitação) |
| `typst_layout` | 11 | 720 | 731 | 2% | reimplementado em `typst_core::rules::layout` com nomes diferentes |
| `typst_pdf` | 3 | 500 | 503 | 1% | export reimplementado em `typst_infra::export` |
| `typst_html` | 0 | 359 | 359 | 0% | **backend fora de escopo** (cristalino é PDF) |
| `typst_utils` | 2 | 192 | 194 | 1% | utilidades reescritas/inlinadas |
| `typst_kit` | 2 | 148 | 150 | 1% | font/world reimplementados em `typst_infra` |
| `typst_docs` | 0 | 142 | 142 | 0% | fora de escopo (gerador de docs) |
| `typst_svg` | 0 | 131 | 131 | 0% | **backend fora de escopo** |
| `typst_ide` | 0 | 122 | 122 | 0% | fora de escopo (IDE/LSP) |
| `typst_bundle` | 1 | 66 | 67 | 1% | wiring vanilla ≠ wiring cristalino |
| `typst_eval` | 1 | 65 | 66 | 2% | eval reimplementado em `typst_core::rules::eval` |
| `typst_render` | 0 | 49 | 49 | 0% | **backend raster fora de escopo** |
| `typst_realize` | 0 | 47 | 47 | 0% | realização reescrita (introspect/fixpoint) |
| `typst_timing` | 0 | 22 | 22 | 0% | ADR-0006 (PROPOSTO; não materializado) |
| outros (`typst`,`test_wrapper`,`typst_fuzz`,`typst_macros`) | 0 | 39 | 39 | 0% | harness/macros não migrados |
| **TOTAL** | **1625** | **10745** | **12370** | 13% | — |

## A limitação que faz o número mecânico mentir para baixo (README do mapa)
A lente pareia por **nome de símbolo** normalizado. O cristalino **reescreveu** `typst_library` em
`typst_core` com tipos **renomeados** — sobretudo `…Elem` structs → **variantes de enum `Content`**
(ADR-0026/0105). Logo uma feature **migrada-e-renomeada nunca pareia** e aparece como "só-vanilla"
(0% mecânico). Por isso:
- **`typst_library` 6% mecânico ≠ 6% de cobertura.** A cobertura *declarada* (Inventário 148, P299)
  é **user-facing ~69%** e **Content variants 95%** — medida por feature, não por nome de símbolo.
- O `typst_syntax` 80% é alto **porque** o parser manteve os nomes (`ast::Arg`, `SyntaxKind`, …) —
  pareia bem. Onde houve renome (library), o pareamento desce sem que a migração tenha falhado.
- **Sinal real de para-onde-foi:** a coluna *destino dominante* + o cruzamento §4.1 do mestre
  (declarado-fechado vs mecânico-não-iniciado), **não** a percentagem mecânica crua.

## Leitura honesta
- **Estrutura:** reescrita muito mais consolidada e acíclica (4 vs 21 crates, 40 vs 434 deps, 4 vs 11
  ciclos). Este é o ganho Tekt, e é **medido**, não declarado. [mecânico, fiável]
- **Features:** subconjunto focado em PDF/CLI. Dos 10745 não-pareados, o grosso é **(a)** stdlib-tail
  de `typst_library` (renomes + features não implementadas) e **(b)** backends fora de escopo
  (`html`/`svg`/`render`/`ide`/`docs` = ~800 itens que o cristalino não persegue). [mecânico +
  inferência de escopo]
- **O pareamento é um PISO da migração real** — os renomes `Elem→variant` deprimem-no. Para "quanto
  falta de feature", a fonte é o Inventário 148 (P299/P384), não esta tabela.

## Gates
- read-only: zero `.rs` alterado; símlink temporário removido; árvore de código limpa.
- lente: corrida limpa (exit 0); censo bate o portão laudo 0078 (pareados 1625 ≈ 1474+).
- este `.md` é snapshot; **o mestre `mapa-migracao-vanilla-cristalino.md` não foi reescrito** (a sua
  regeneração mecânica + reconciliação da coluna declarada é processo próprio, decisão do dono).

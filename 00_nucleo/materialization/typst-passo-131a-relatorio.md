# Passo 131A — Relatório (diagnóstico `Lang` + ADR-0052 PROPOSTO)

**Data**: 2026-04-24
**Precondição**: Passo 130 encerrado; 1057 total tests; zero
violations; 51 ADRs activas; 11 DEBTs abertos.
**Natureza**: passo L0-puro. Sem código. Sem testes. **Produz 2
artefactos de design**.
**ADR**: **ADR-0052** criada em status `PROPOSTO`.

---

## Sumário

Dois artefactos produzidos:

1. **Diagnóstico** em
   `00_nucleo/diagnosticos/diagnostico-lang-passo-131a.md` com
   os 7 itens mínimos de ADR-0034 preenchidos a partir de
   leitura profunda de `lab/typst-original/.../text/lang.rs`.

2. **ADR proposta** em
   `00_nucleo/adr/typst-adr-0052-lang-tipo-semantico.md` com
   status `PROPOSTO`, decisão arquitectural completa, 5
   alternativas, plano de materialização para 131B.

**Zero código tocado**. **1057 total tests preservado**. Lint
zero violations.

---

## 131A.1 — Inventário vanilla

Leitura de `lab/typst-original/crates/typst-library/src/text/lang.rs`.

**Decisões estruturais vanilla**:
- `struct Lang([u8; 3], u8)` — compact 4-byte newtype: 3 bytes
  ASCII + 1 byte length discriminator (2 ou 3).
- `Copy + Clone + Eq + PartialEq + Ord + PartialOrd + Hash`.
- **~260 constantes** `pub const X: Self` (`ABKHAZIAN…ZULU`).
- `impl FromStr`: 2-3 ASCII letters, lowercase normalizado.
- `cast!` macro adiciona hint condicional para `"en-GB"`
  (detecta composto e sugere usar param `region`).
- `Dir` usado em `lang.dir()` — tipo de `layout` (não
  relevante para o subset deste passo).

## 131A.2 — Inventário cristalino

- **Zero colisão de nome**: `grep Lang\\b` em `01_core/src/`
  não encontra tipo com esse nome.
- **Tipos tipográficos L1 existentes**: `FontWeight`,
  `FontStretch` em `entities/font_book.rs`. `Length`, `Abs`
  em `entities/layout_types.rs`.
- **Crates L1 autorizadas**: `ecow` (ADR-0024),
  `rustc-hash` (ADR-0018), `thiserror`. **Nenhuma crate nova
  necessária**.
- **ADRs 0001-0051 existem**; próximo livre: **0052** ✓.

## 131A.3 — Diagnóstico escrito

`diagnostico-lang-passo-131a.md` cobre:

- **Item 1**: localização vanilla + linhas exactas.
- **Item 2**: declaração + derives + 2 exemplos concretos
  (`ENGLISH` 2-letter, `FILIPINO` 3-letter).
- **Item 3**: métodos (`as_str`, `dir`), `FromStr` literal,
  `cast!` macro com hint.
- **Item 4**: dependências externas (ecow, rustc_hash, std).
- **Item 5**: semântica detalhada + 7 casos edge concretos
  (empty, 1-letter, 4-letter, case, hyphen, non-ASCII, digits).
- **Item 6**: mensagem de erro literal + hint condicional.
- **Item 7**: divergências propostas com razões — forma
  interna fiel, validação idêntica, constantes **só `ENGLISH`**
  (on-demand para resto), API mínima (sem `dir()` neste passo),
  mensagem fiel, localização em ficheiro novo `entities/lang.rs`.

**Itens adicionais**:
- Impacto em 3 call-sites (`style_chain.rs`, `rules.rs`,
  `mod.rs`).
- Plano de teste 131B — 8-10 unit tests + 3 integration tests
  adaptados + 1 novo.
- Recomendação quarta nota em ADR-0038.

## 131A.4 — ADR-0052 proposta

Status: **`PROPOSTO`**. 7 decisões numeradas:

1. `struct Lang([u8; 3], u8)` fiel vanilla.
2. `impl FromStr` idêntico (com mensagem literal vanilla).
3. Erro hard em inválido.
4. `StyleDelta.lang: Option<Lang>`.
5. `entities/lang.rs` ficheiro novo.
6. Apenas `Lang::ENGLISH` inicialmente.
7. `Dir` + hint de region deferidos.

Alternativas: 5 consideradas (manter raw, inline validation,
materializar fiel ✓, crate externa `unic-langid`, enum 260
variantes). Escolha justificada.

Consequências: positivas (paridade, base extensões),
negativas (custo S 1-2h, primeiro Err em loop args),
neutras (nota ADR-0038).

Referências: 5 ADRs relevantes + diagnóstico + Passo 130
relatório + vanilla path.

---

## Verificação

1. ✓ Diagnóstico existe com 7 itens ADR-0034 preenchidos.
2. ✓ ADR-0052 existe com status `PROPOSTO`.
3. ✓ Diagnóstico referencia ADR (`ADR alvo: ADR-0052`).
4. ✓ ADR referencia diagnóstico (link relativo).
5. ✓ ADR lista 5 alternativas com prós/contras.
6. ✓ Zero ficheiros L1/L2/L3/L4 tocados (confirmado `git
   status` — só L0 new files).
7. ✓ `cargo test --workspace` inalterado: 826+186+24+21 =
   1057, 6 ignorados.
8. ✓ `crystalline-lint` zero violations.

---

## Ficheiros criados

| Ficheiro | Natureza | Linhas |
|----------|----------|-------:|
| `00_nucleo/diagnosticos/diagnostico-lang-passo-131a.md` | L0 diagnóstico ADR-0034 | ~340 |
| `00_nucleo/adr/typst-adr-0052-lang-tipo-semantico.md` | L0 ADR PROPOSTO | ~170 |
| `00_nucleo/materialization/typst-passo-131a-relatorio.md` | L0 relatório (este) | ~180 |

**Nenhum** ficheiro de código ou teste tocado.

---

## Lições

1. **Estrutura compacta vanilla é surpreendentemente simples**:
   esperava-se tipo com enum de 260 variantes, struct complexo
   com parser RFC 5646 completo, ou dependência em `icu_locid`.
   Realidade: newtype `[u8;3]+u8` com FromStr de 10 linhas.
   Cristalino replica sem fricção. Ler o vanilla antes de
   adivinhar paga-se — **eixo central do ADR-0034**.

2. **Constantes on-demand é decisão honesta**: vanilla tem
   260 constantes porque tem consumers (translations,
   hyphenation) que iteram ou referem por nome. Cristalino
   não tem esses consumers hoje. Replicar 260 linhas sem uso
   é dívida vazia. `ENGLISH` apenas, adicionar outras quando
   consumer pedir.

3. **Erro hard no loop de args é ruptura do pattern**:
   `eval_set_rule` actual emite **warnings** para todos os
   problemas de propriedade (DEBT-49). Introduzir `Err` em
   `"lang"` é primeiro caso de erro fatal. Decisão
   correcta (paridade vanilla) mas merece menção explícita
   na ADR — readers futuros podem se confundir com o mix
   warning/erro dentro do mesmo loop.

4. **Diagnóstico antes de código evita tecido morto**: se
   tivesse escrito `Lang` antes do diagnóstico, provavelmente
   teria incluído `dir()` e `Locale` e talvez algumas
   constantes "úteis". Diagnóstico forçou auditoria: `dir()`
   exige `Dir`, `Locale` exige `Region` — ambos fora
   de escopo. Poupa 100+ linhas de código não usado.

5. **ADR-0034 é regulatório não-invasivo**: a regra
   "diagnóstico antes de código" não retarda
   significativamente um XS/S — leitura do vanilla demora
   20-30 min, escrita do diagnóstico 30-40 min. Hoje saimos
   do passo com **artefactos reutilizáveis**: 131B reúne o
   diagnóstico e implementa, 132 pode consultar como
   precedente quando materializar próximo tipo vanilla.

6. **Passo L0-puro primeira vez em 5 passos**: série
   126-130 foi toda L1 código. 131A muda ritmo para L0
   documentação. Transição de ritmo permitiu leitura atenta
   sem pressão de compilar — valor do split 131A/131B é
   **disciplinar**, não técnico.

---

## Próximo passo: 131B

Após aprovação deste diagnóstico pelo utilizador, 131B
enunciar-se-á com base em:

- Diagnóstico como fonte de verdade (decisões 1-7 do "Item 7").
- ADR-0052 como contrato arquitectural (status transita para
  `IMPLEMENTADO` ao fechar 131B).
- Plano de migração detalhado em 6 passos.
- 8-10 unit tests + 3 integration tests adaptados + 1 novo.

**Estimativa 131B**: S, 1-2h implementação.

---

## Estado pós-Passo 131A

### Ficheiros L0

- Diagnóstico `Lang` disponível para referência futura.
- ADR-0052 em `PROPOSTO`.
- Roadmap DEBT-1 revisto: 131A→131B→132→133→134→135.

### Código L1/L2/L3/L4

**Inalterado**. `StyleDelta.lang: Option<EcoString>` continua
a capturar raw sem validação até 131B.

### Divergência ADR-0033

**Documentada** mas **não resolvida** neste passo. Resolução
em 131B. Teste canary
`eval_set_text_lang_bcp47_composto_passo_130` continua a
passar como "aceita tudo" — será **invertido** em 131B.

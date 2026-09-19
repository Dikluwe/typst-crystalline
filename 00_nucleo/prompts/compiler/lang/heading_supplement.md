# Prompt L0 — suplemento localizado de heading
Hash do Código: 28eba642

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/lang/defaults.toml sha256:c8f920865f8895a89d9c42659c15e773e9bb2603bd614e390805f620834babd9

**Camada:** L1
**Ficheiro alvo:** `01_core/src/compiler/lang/heading_supplement.rs`
**ADRs:** ADR-0107, ADR-0108, ADR-0127, ADR-0129.

## Medição anterior à decisão

No commit `c7577a031847ff38e4780044bc4132c1d92a14b9`, a localização de
heading tinha três decisões produtivas: uma tabela privada em
`compiler/introspect/heading.rs` e branches `pt`/`en` em
`compiler/introspect/labelled.rs` e `compiler/layout/references.rs`. A fixture
P1459 produziu `Abschnitt 1` no vanilla ratificado `586e1bd43` e `Section 1`
no cristalino para alvo alemão. Quando a referência francesa apontou para o
mesmo alvo, o vanilla conservou `Abschnitt`: a língua causal é a do alvo, não
a língua no ponto da referência.

A fonte ratificada `crates/typst-library/translations/*.txt`, linha `heading`,
tem 45 entradas base não vazias. A única diferença entre essa fonte e a tabela
cristalina anterior é francês: a fonte vigente diz `Section`, enquanto a
tabela antiga, transcrita do pin substituído, dizia `Chapitre`. Entradas
regionais `pt-PT` e `zh-TW` não pertencem ao domínio atual de `Lang`, que
transporta apenas códigos ASCII de duas ou três letras.

## Contrato

`heading_supplement_for_lang(Option<&Lang>) -> &'static str`:

- faz igualdade exata de `Lang::as_str()` numa única tabela canónica;
- contém as 45 entradas base não vazias do pin ratificado, inclusive
  `fr → Section`;
- devolve `Section` para `None` ou código sem tradução de heading;
- é puro, determinístico e independente de ambiente, introspecção, eval,
  layout e `lab/` em runtime;
- tem visibilidade limitada a `crate::compiler`; nenhuma assinatura pública
  da crate é criada;
- é a única decisão linguística de suplemento automático de heading dentro
  dos consumers de compiler L1: esses consumers não mantêm tabelas, branches
  `is_pt` nem literais sucessores;
- o serializer raw-Content legacy de L2 permanece uma dívida distinta já
  medida em `shell/cli.md` P1307-R5; não conta como owner desta função interna
  e deve ser eliminado num passo próprio que resolva o gate público ADR-0127,
  não por duplicação silenciosa nesta unidade.

## Aceitação

- todas as 45 entradas da fonte ratificada são exercitadas por teste de owner;
- `None` e língua válida sem entrada usam inglês;
- introspecção de heading em francês expõe `Section`, não `Chapitre`;
- referências a alvos `de`, `es`, `fr` e `it` usam a língua causal do alvo
  mesmo quando o texto da referência tem outra língua;
- supplement explícito e vazio conservam precedência, e o join público
  conserva U+00A0.

Esta é correção interna de paridade e ownership: não muda API, default de
produto, fase ou compatibilidade, logo segue o fluxo contínuo ADR-0127.

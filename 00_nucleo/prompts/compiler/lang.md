# Prompt L0 — hub `compiler/lang`
Hash do Código: 9076d4b1

**Camada:** L1
**Ficheiro alvo:** `01_core/src/compiler/lang/mod.rs`
**ADRs:** ADR-0057, ADR-0060, ADR-0107, ADR-0108, ADR-0127, ADR-0129.

## Medição anterior à decisão

O consumer vigente contém apenas as declarações públicas dos módulos
`equation_supplement`, `figure_supplement`, `outline_title` e `quotes`. As
tabelas linguísticas e seus fallbacks já vivem nesses ficheiros atomizados.
P1459 mediu uma exceção: a tabela de heading estava enterrada em
`compiler/introspect/heading.rs` e outros dois consumers repetiam uma decisão
`pt`/`en`. Isso deixa uma obrigação linguística sem owner de fase neutra.

## Contrato do hub

- Expor estaticamente os quatro módulos lang-aware públicos.
- Declarar `heading_supplement` como submódulo privado e reexportar somente o
  helper com visibilidade `pub(in crate::compiler)`, suficiente para
  introspecção e layout sem criar API pública nova.
- Não implementar localização, fallback, lookup ou geração de texto.
- Manter hyphenation em `compiler/layout/hyphenation.rs`; este saneamento não
  muda fase nem ownership dessa feature.
- Permanecer L1 puro, sem wrappers ou despacho dinâmico.

## Aceitação

Os quatro módulos anteriores permanecem publicamente acessíveis. O quinto
owner permanece interno a `compiler`; o hub não contém tabela, branch de
língua ou wrapper comportamental.

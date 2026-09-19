# Prompt L0 — hub `compiler/lang`
Hash do Código: 23aa74d0

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

## P1460 — reexport público estreito do owner de heading

### Medição anterior à decisão

No commit `7f7a7ab1f9afc3e92ed29800408d9508525a6094`, o submódulo
`heading_supplement` é privado e o reexport usa `pub(in crate::compiler)`.
L2 pode depender de L1, mas não atravessa essa visibilidade; por isso
`shell/cli.rs` conserva um literal sucessor.

### Decisão

Manter o submódulo privado e promover o reexport existente para
`pub use heading_supplement::heading_supplement_for_lang`. O hub não ganha
wrapper, constante, tabela ou lógica. A única API nova é a função do owner
reexportada no namespace já público `typst_core::compiler::lang`.

Gate ADR-0127 confirmado pelo dono no pedido P1460 de 2026-09-18. Nenhum
default, fase ou assinatura existente é alterado.

### Aceitação

Uma crate L2 importa a função pelo hub; consumers L1 continuam pelo mesmo
símbolo; o módulo físico e a tabela permanecem privados.

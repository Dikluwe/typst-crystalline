# Prompt L0 — hub `compiler/lang`
Hash do Código: f4ab393c

**Camada:** L1
**Ficheiro alvo:** `01_core/src/compiler/lang/mod.rs`
**ADRs:** ADR-0057, ADR-0060, ADR-0107, ADR-0108, ADR-0129.

## Medição anterior à decisão

O consumer vigente contém apenas as declarações públicas dos módulos
`equation_supplement`, `figure_supplement`, `outline_title` e `quotes`. As
tabelas linguísticas e seus fallbacks já vivem nesses ficheiros atomizados.

## Contrato do hub

- Expor estaticamente os quatro módulos lang-aware.
- Não implementar localização, fallback, lookup ou geração de texto.
- Manter hyphenation em `compiler/layout/hyphenation.rs`; este saneamento não
  muda fase nem ownership dessa feature.
- Permanecer L1 puro, sem wrappers ou despacho dinâmico.

## Aceitação

Os quatro módulos permanecem publicamente acessíveis e o corpo produtivo,
desconsideradas somente as linhas de linhagem, fica idêntico ao estado anterior
ao P1201.

# L0 — mod
Hash do Código: fa84a49f


**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/mod.rs`

## Consumer evidence

`01_core/src/compiler/mod.rs:7` locates `pub mod eval;`.
This audit locates the existing consumer; it does not establish language parity.
Historical evidence and unresolved discrepancies are archived in
`00_nucleo/diagnosticos/sanitization-remaining-compiler-history.md`.

## Ownership and boundary

This owner governs only the static compiler facade in `01_core/src/compiler/mod.rs`.
Declare the public modules `eval`, `introspect`, `lang`, `layout`,
`lexer`, `math`, `parse`, `scopes` and `stdlib`. Do not add execution
logic or behavioral reexports. Each module's own prompt owns its API and behavior.

The compiler remains pure L1: no infrastructure imports, I/O or mutable global
state. Evaluation, paginated layout and semantic HTML are distinct responsibilities;
this facade does not execute, reorder or merge their phases.

Canonical owners include `00_nucleo/prompts/compiler/eval.md`,
`00_nucleo/prompts/compiler/introspect.md`, `00_nucleo/prompts/compiler/lang.md`,
`00_nucleo/prompts/compiler/layout.md`, `00_nucleo/prompts/compiler/lexer/mod.md`,
`00_nucleo/prompts/compiler/math/mod.md`, `00_nucleo/prompts/compiler/parse.md`,
`00_nucleo/prompts/compiler/scopes.md` and `00_nucleo/prompts/compiler/stdlib/_comum.md`.

## Custos textuais personalizados — 2026-09-28

### Medição anterior à decisão

`correcao-custos-paragrafo-2026-09-28/before-pdf/` registra 66 entradas no
estado `initial.json` e `before/`, contra pin 9f2b6e8715237cb086899a42873660fe744622e8.
`text/mod.rs:1507-1564` valida Dictionary, Ratio por campo e composição parcial;
`inline/mod.rs:240` lê costs do escopo compartilhado, não de cada fragmento;
`inline/linebreak.rs:29-30,961-962` aplica defaults 135/100 vezes o ratio.
O Tekt descarta costs em eval/rules e aceita sem transportar no constructor;
paragraph_hyphen e paragraph_select usam defaults fixos. Os erros e spans
estão no stderr vanilla, inclusive negativos aceitos. Não inferir intenção.

### Decisão vigente

Correção interna/de paridade ADR-0127 em fluxo contínuo, sem novo contrato
público, flag ou fase. Módulo privado compiler/text_costs possui owner único
para cast, transporte por componente e resolução dos custos compartilhados.
Hifenização/runt globais passam a consultar a configuração. Widow/orphan são
validados e transportados, mas influência na paginação continua incompleta.
Custos de fragmentos parciais não substituem o custo compartilhado; os quatro
canais validados são neutros para glyphs na whitelist Styled. Um wrapper integral
é fatorado pela realização existente antes de formar Par. Largura variável
permanece limitada. Não deduzir custo pelo glyph. Preservar todos os defaults.

Declarar somente mod text_costs privado; manter fachada estática, sem lógica/reexports novos.


## Realização com avaliação única

Expor compiler::realize, fachada de domínio puro para realização após a execução comum.

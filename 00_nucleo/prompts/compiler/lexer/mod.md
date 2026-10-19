# L0 — lexer/mod
Hash do Código: 3fecf689


Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/lexer/mode-boundaries.toml sha256:aae80d538980eeec87b884712269e3b777fe44c0dae6b3503b5fa9ee4f9ad76f

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/lexer/mod.rs`

## Consumer evidence

`01_core/src/compiler/lexer/mod.rs:14` locates `pub mod scanner;`.
This audit locates the existing consumer; it does not establish language parity.
Historical evidence and unresolved discrepancies are archived in
`lab/results/construction/diagnostics/sanitization-remaining-compiler-history.md`.

## Lexer hub

Own `Lexer<'s>`, universal trivia, token assembly and mode dispatch in
`01_core/src/compiler/lexer/mod.rs`, not the child mode grammars.
Preserve its scanner, current SyntaxMode, newline flag and pending SyntaxError;
construction, mode/set_mode, cursor/jump, newline/column and next interfaces
remain unchanged. Column is measured in characters from the last newline,
while cursor positions are byte offsets.

The parser controls mode changes; lexical handlers do not change modes.
Before mode dispatch, recognize end, whitespace, initial shebang, line comments,
nested block comments and raw outside Math. Whitespace left outside Markup
text yields Space, or Parbreak for multiple newlines in Markup. Preserve
newline tracking and error-to-SyntaxNode conversion, original token text,
offsets and diagnostic hints, including `unexpected end of block comment`.

Delegate each remaining token to the active mode exactly once:
`00_nucleo/prompts/compiler/lexer/markup.md`,
`00_nucleo/prompts/compiler/lexer/code.md` or
`00_nucleo/prompts/compiler/lexer/math.md`.
Raw mechanics live with Markup despite universal entry outside Math.
`00_nucleo/prompts/compiler/scanner.md` owns cursor safety.
Quote localization and evaluation are not lexer responsibilities.

Acceptance covers universal precedence, nested comments, lossless token
boundaries, mode isolation, newline/column behavior and pending diagnostics.
Do not infer language parity from matching tokenization code.


## Nomes de domínio e traits Rust — 2026-10-16

### Medição anterior à decisão

Lexer.next aparece no inventário Clippy de 13 avisos. A assinatura e o corpo vigentes foram lidos antes desta decisão; snapshot lab/results/construction/diagnostics/clippy-traits-formatacao-2026-10-16/before.json.gz conserva fontes/L0 completos, hashes globais, HEAD, working tree não commitado, UTC e diffstat. O contrato de retorno difere do trait sugerido. É comparação de interfaces Rust, sem nova medição de paridade ou benefício de leitura por IA.

### Responsabilidade exclusiva e aceitação

Preservar next(&mut self) -> (SyntaxKind, SyntaxNode), token End explícito inclusive em chamadas repetidas ao fim, erros/newline e mudança de modo pelo parser. Não implementar Iterator nem alterar o retorno para Option. Corrigir somente a descrição do tipo para tokenizer com sentinela explícita; next recebe expectativa nominal de should_implement_trait.

Usar somente #[expect(clippy::should_implement_trait, reason="S001: tokenizer com token End explícito e modo/cursor controlados pelo parser; retorno não é Option")] sobre o método nomeado. Não suprimir módulo/impl/crate, elevar limiar ou estender a exceção a Styles::from_iter. Implementar o trait proposto aqui exigiria contrato novo ou semântica diferente; a exceção preserva o contrato existente. Fluxo contínuo ADR-0127 para metadados/documentação, sem API, default ou fase novos. L0 primeiro e resselo restrito; RED do lint e GREEN incluindo unfulfilled_lint_expectations como erro; preservar corpos e usar os testes funcionais existentes sem alterar esperados. Validar workspace/build/Clippy/doc/fmt/linhagem sob 12 GiB, swap zero e um job.

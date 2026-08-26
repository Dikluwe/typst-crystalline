# Prompt L0 — `compiler/lexer/markup` — modo Markup

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/lexer/mode-boundaries.toml sha256:aae80d538980eeec87b884712269e3b777fe44c0dae6b3503b5fa9ee4f9ad76f

Hash do Código: f5d2c97a

**Camada**: L1
**Ficheiro alvo**: `01_core/src/compiler/lexer/markup.rs`
**Criado em**: 2026-08-26 (P1199; individualização de `lexer/mod.md`)
**ADRs**: ADR-0003, ADR-0037, ADR-0129

---

## Medição antes da decisão

`markup.rs` possui marcadores, links, labels, refs, escapes, texto e raw. O hub
apenas chama essas rotinas após resolver tokens universais. P1199 move a
especificação exclusiva de Markup para owner próprio, sem mudar tokenização.

## Responsabilidade

Tokenizar conteúdo textual e estrutural em `SyntaxMode::Markup`.

Reconhece escapes e linebreaks, links automáticos HTTP(S), labels, referências,
shorthands, delimitadores de conteúdo/math, smart quotes e marcadores de
heading, lista, enumeração e termos. `*` e `_` só viram delimitadores fora de
palavra; scripts CJK não são tratados como palavra alfanumérica ocidental para
essa decisão.

## Texto e trivia

`text()` agrupa conteúdo até uma fronteira lexical. Conforme P1137, absorve
exatamente um espaço ASCII quando o próximo escalar é alfanumérico. Tabs,
newlines, whitespace Unicode e espaço antes de pontuação continuam como trivia
separada pelo hub. Prefixos parecidos com link, ref ou shorthand permanecem
texto quando não completam a forma especial.

## Raw, links e labels

Raw inline preserva whitespace não-newline e marca newlines como
`RawTrimmed`. Raw block reconhece language tag, calcula dedent mínimo por
caracteres e separa delimitadores, texto e trechos aparados. Links automáticos
rejeitam brackets desequilibrados. Labels exigem conteúdo e `>` final; refs
removem `.` e `:` finais prováveis do texto adjacente.

## Critérios de verificação

- `hello world` é um `Text`; espaço antes de pontuação e tab permanecem trivia.
- Smart quotes existem somente em Markup.
- Raw preserva nós e dedent documentados.
- Marcadores só são reconhecidos nas fronteiras de espaço/fim adequadas.
- O smoke test e a suíte completa do lexer passam.
- P1199 não modifica corpos Rust.

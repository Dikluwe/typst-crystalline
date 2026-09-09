# P1321 — revisão do L0 anterior ao candidato

Veredito: conteúdo apto para congelamento; fluxo contínuo ADR-0127. GO de
implementação ainda depende de RED e freeze A/B. Esta é revisão do contrato
observável, sem escrita no L0 ou produto pelo revisor.

Entrada L0 SHA-256
`03de416f06983fa6a5a29253e672a795f357fda18e9322e79b4f66b925940f96`, seção
P1321 em `00_nucleo/prompts/compiler/stdlib/loading.md:243`. O arquivo inteiro
anterior já foi lido; li integralmente a seção inserida. Baseline JSON SHA-256
`4b4de0feeb4c1df1b9007de81f11f6a4d1ae90b25cdc1c9ca1eadaa211e58a96` e medição
SHA-256 `f13c4ea33fe8d8f91a5ef443d3183d1cbb3f77d9ac6726c0e4c5e23c54fbef05`
conferidos em disco. A medição registra HEAD d31047d7b8af7837c84adae4ded3d2ff50c62093
e working tree P1319, entre 2026-09-08T20:15:24.348977+00:00 e
2026-09-08T20:15:27.319704+00:00, com diff/stat e fontes.

Li as 24 observações bilaterais (12 expressões): missing, named source sem
positional e com positional, excesso sobre Bytes válido/malformado e arquivo
ausente, cast/opção vencendo unknown, inversões de unknown/excesso e With.
Todas sustentam as regras que o L0 declara. Não são prova isolada de todas
as rotas; A/B e testes locais devem completar carriers/sintéticos/preservação.

A seção substitui explicitamente ordem/missing/unknown/excesso P1313–P1319,
preserva parser e diagnósticos após validação válida, requer dividir os
controles antigos de parsing+excesso sem descartá-los e mantém Symbol como
fronteira normativa. Define fallback sintético compatível com
`Args::occurrence_sequence` (`entities/args.rs:66–80`): items antes de named,
origens individuais detached, missing no args.span. Não pede alteração no
owner de Args, dispatch ou I/O. O código da macro vanilla coloca handlers,
finish e chamada nessa ordem (`typst-macros/src/func.rs:411–422`), reforçando
que a precedência especificada vem da fonte e não de inferência conveniente.

Recomendação editorial enviada antes do freeze: usar delimitador Markdown
duplo para mensagens que contêm backticks em torno de source. Não é
ambiguidade semântica nem novo gate público. Esta revisão trouxe confirmação
das fronteiras, sem calibração repetitiva ou ajustes a oráculos privados.

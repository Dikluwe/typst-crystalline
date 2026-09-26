# Prompt L0 — segmentação de palavra para quebra de linha
Hash do Código: 2964977f

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/coordinates.toml sha256:2ccbb1e5daf5f6806e58cd48272b1463fbc9107300c0bce6ea1c3ccf7b0b8748

**Camada:** L1
**Ficheiro alvo:** `01_core/src/compiler/layout/word_segment.rs`
**Vanilla ratificado:** `9f2b6e8715237cb086899a42873660fe744622e8`
**ADRs:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129

## P1504 — medição antes da decisão

Na working tree P1503 registrada em
`00_nucleo/materialization/typst-passo-1504.md`,
`compiler/layout/cursor.rs:1192-1303` contém a seleção de scripts sem
espaços, a criação de `icu_segmenter`, o tailoring de aspas CJK e a
fragmentação; `layout_word:242-299` chama esse caminho, e `layout_chunk`
posiciona os fragmentos. As tabelas ICU estão embutidas em tempo de
compilação, sem leitura de arquivo em L1.

## Decisão e contrato

Este consumer é dono somente da segmentação de palavra para scripts sem
espaços e da filtragem de oportunidades que colocariam aspas CJK na borda
errada. Ele emite fragmentos não vazios ao hook de chunk existente, na
ordem dos breakpoints, sem inserir espaço nem reinterpretar idioma, estilo,
shaping ou hifenização. `compiler/layout/cursor.md` conserva medição,
baseline, wrap, `layout_word` e `layout_chunk` e decide quando chamar a
segmentação. O segmentador é reconstruído por invocação; não introduzir
estado global mutável, I/O ou assinatura pública. O dispatcher permanece
estático e exaustivo.

Aceitação: CJK com aspas de abertura/fecho, Thai/Lao/Myanmar/Khmer, texto
latino, RTL e palavra com hifenização preservam fragmentos, páginas,
posições e texto antes/depois. Breakpoints em UTF-8 devem continuar em
fronteiras válidas. Refutam a fronteira: o owner novo decidir a paginação,
duplicar wrap, mudar o tratamento de spaces ou alterar a fase de layout.

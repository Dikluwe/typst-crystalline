# Archived pre-sanitation L0 — 00_nucleo/prompts/wiring/tests/p1463-format-png.md

This is historical evidence, not a materializable prompt or an approval.
Original SHA-256: `af95be1d4150ca61097190fc15449cd94f242e3f3de4b6a1eac3e4cab946ca31`.
The original text below is preserved without rewriting; dates, line references,
working-tree inventories, proposals and old parity claims retain their original scope.
Dead or unpinned references are archival only and cannot prove current behavior.

---

# Prompt L0 — integração de `format.png`
Hash do Código: 079b8f39

**Camada:** L4
**Consumer exclusivo:** `04_wiring/tests/p1463_format_png.rs`

## Contrato

Executar o binário cristalino e ler dimensões diretamente do IHDR PNG.

- default documental: 144 pixels por polegada e A4 `1191 × 1684`;
- documento e CLI convertem o PPI efetivo por `ppi / 72`;
- CLI vence a opção documental quando explícita;
- documento e CLI com o mesmo PPI produzem o mesmo IHDR;
- zero e negativos finitos produzem dimensão mínima segura;
- integer/float, cascata, regra vazia, diagnósticos e rejeição em container;
- aliases de compile/watch aceitam a flag somente onde ela é pública;
- SVG, PDF, HTML, `pretty`, `compact`, tags e features permanecem neutros;
- os ramos PNG com e sem fontes usam a mesma resolução efetiva.
- sucessão P1465: sem feature, `format.html` produz o gate público de `html`;
  com feature, sua superfície pertence exclusivamente ao teste P1465.
  `format.pdf` não é tratado como sibling futuro.

Não comparar bytes comprimidos, metadados, tamanho do arquivo, hashes, scores
ou artefatos administrativos. Não exercitar NaN ou infinito com risco de OOM.

## Retificação P1466

A assimetria histórica P1463 fica explicitamente superada: ela confundia a
unidade pública do documento com pixels por ponto e fazia o default A4 tentar
alocar `41568028160` bytes de RGBA. Os testes devem ler somente IHDR e impor
dimensões semanticamente derivadas, com timeout e limite de memória; morte do
processo, ausência de arquivo ou OOM nunca contam como sucesso.

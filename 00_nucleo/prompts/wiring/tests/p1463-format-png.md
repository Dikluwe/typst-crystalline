# Prompt L0 — integração de `format.png`
Hash do Código: 130ddd7c

**Camada:** L4
**Consumer exclusivo:** `04_wiring/tests/p1463_format_png.rs`

## Contrato

Executar o binário cristalino e ler dimensões diretamente do IHDR PNG.

- default documental: 144 pixels por polegada;
- documento entrega `ppi` diretamente como pixels por ponto;
- CLI entrega `ppi / 72` e vence a opção documental quando explícita;
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

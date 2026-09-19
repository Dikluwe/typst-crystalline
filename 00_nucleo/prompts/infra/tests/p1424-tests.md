# Prompt L0 — integração HTML `html.frame` P1424

Hash do Código: 30273a3c

**Camada:** L3 (testes)  
**Ficheiro alvo exclusivo:** `03_infra/tests/p1424_html_frame.rs`  
**Owners produtivos observados:** `infra/pipeline.md`, `infra/export/html.md`,
`infra/export/svg.md`

## Obrigação

Este consumer exclusivamente de teste exerce a fachada pública HTML. Deve
provar no nível da linguagem:

- feature desligada falha; feature HTML ligada expõe `repr(html.frame) ==
  "frame"` e preserva os três diagnósticos de assinatura;
- texto produz exatamente uma raiz SVG inline não escapada, block-level,
  intrínseca, positiva e sem tag literal `frame` ou canvas A4;
- rect com `fill: red` 10pt×5pt produz dimensões físicas e
  `viewBox 0 0 10 5` com tolerância de 0,001pt e preserva a cor pública
  ratificada `#ff4136`; não aceita a tradução para vermelho CSS `#ff0000`;
- nesting produz uma única ilha com a mesma projeção geométrica/visual da ilha
  simples;
- `html.elem("frame")` permanece tag literal e a rota sem frames preserva o
  DOM existente;
- a própria fachada/pipeline rejeita explicitamente um `html.frame` cujo body
  é exclusivamente HTML e portanto não é suportado pela ilha paginada; não
  pode convertê-lo em página/SVG vazio bem-sucedido;
- um carrier não resolvido entregue diretamente ao exporter também falha
  fechado, sem omissão, fallback textual ou tag literal.

IDs, ordem de atributos, paths e igualdade de bytes SVG não são aceitação. O
teste não lê nem adapta expectativas à implementação candidata.

## Proveniência e limite

Manifesto SHA-256 `e4164634bd958e5889778484d42ac2b37c11f7e8eaccefff89e0ab9271b2dfab`;
contrato SHA-256 `d92d2ccf4efd592c92cfa59a30dcfc8beb5707976aa167f7ac6f90e87f6b8f31`.
Baseline `79c7a6d60a29bcf53a3b06f15d800dcf286381a6`. Execução:
**executado sem atestação de isolamento**.

R4 acrescenta somente o witness executável de P13/M20 exigido pelo relatório
independente P1424 (SHA-256 `bcb706743a34c786364c816028763546791a397bc206f5893ae9a5c34a0e9c91`)
e pela verificação FAIL correspondente; os demais casos permanecem intactos.

P1454 corrige a expectativa de paint do rect após medir bilateralmente a
paleta pública: `red.to-hex()` é `#ff4136` nos dois produtos. A expectativa
anterior `#ff0000` legitimava uma pós-reescrita indevida exclusiva do SVG
embutido e não era paridade de linguagem.

# Prompt L0 — integração HTML `html.frame` P1424

Hash do Código: 6e81ca73

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
- rect vermelho 10pt×5pt produz dimensões físicas e `viewBox 0 0 10 5` com
  tolerância de 0,001pt e pintura vermelha;
- nesting produz uma única ilha com a mesma projeção geométrica/visual da ilha
  simples;
- `html.elem("frame")` permanece tag literal e a rota sem frames preserva o
  DOM existente;
- um carrier não resolvido entregue diretamente ao exporter falha fechado, sem
  omissão, fallback textual ou tag literal.

IDs, ordem de atributos, paths e igualdade de bytes SVG não são aceitação. O
teste não lê nem adapta expectativas à implementação candidata.

## Proveniência e limite

Manifesto SHA-256 `e4164634bd958e5889778484d42ac2b37c11f7e8eaccefff89e0ab9271b2dfab`;
contrato SHA-256 `d92d2ccf4efd592c92cfa59a30dcfc8beb5707976aa167f7ac6f90e87f6b8f31`.
Baseline `79c7a6d60a29bcf53a3b06f15d800dcf286381a6`. Execução:
**executado sem atestação de isolamento**.

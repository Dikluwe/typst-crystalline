# Prompt — typst-passo-834: `image::svg` — suporte a SVG (achado #19, decisão de escopo pendente)

**Origem**: achado #19 de P831 (lote 5)
**Estado**: aguardando decisão do dono — **não implementar sem essa decisão**, mesmo padrão de P807/P812-C

---

## Achado (medição de P831)

`#image("svg1.svg")` válido — cristalino `error: SVG images are not supported yet` (exit 1, rejeição explícita já no código em `figure_image.rs:167-173`); vanilla compila e renderiza. É scope-out já registrado (desde P772k, segundo o handoff antigo), não um bug de omissão.

---

## Passo 1 — Levantar informação para a decisão (não implementar ainda)

1. Confirmar se já existe uma entrada de dívida formal (ADR ou `00_nucleo/diagnosticos/debt/DEBT.md`) documentando esse scope-out, com a razão registrada (peso de dependência — `usvg`/`resvg`, mencionado no handoff antigo de P772k/P781). Se não existir formalmente, isso já é um achado de processo a registrar (mesmo padrão de P807, que descobriu que o scope-out de `pdf.attach` nunca tinha sido formalizado).
2. Medir o peso real de adicionar suporte a SVG: quais crates seriam necessárias (`usvg`/`resvg` ou alternativa), se already estão na whitelist `l1_allowed_external` do `crystalline.toml`, e uma estimativa de esforço de integração com o pipeline de export existente.
3. Testar os sub-casos de erro que P831 notou divergirem "por arrastamento" (SVG malformado, imagem linkada em falta dentro do SVG) — confirmar se essas mensagens específicas importam para a decisão ou são só consequência de a feature não existir.

## Passo 2 — Decisão (do dono)

Apresentar ao dono, com base no levantamento: manter o scope-out (formalizando como dívida, se ainda não estiver) ou implementar agora. Não decidir sozinho.

## Passo 3 — Implementação (só se a decisão for implementar)

Adicionar suporte a SVG usando a crate escolhida, integrando ao pipeline de export existente (mesma estrutura usada para PNG/JPEG).

## Passo 4 — Validação (só se implementado)

SVG simples renderizando idêntico ao vanilla (comparação de pixel ou de estrutura, conforme o método já usado no projeto para imagens). Suíte completa, comando + contagem antes/depois.

## Relatório

`00_nucleo/diagnosticos/typst-passo-834-relatorio.md` — se a decisão for manter: só a formalização da dívida (com a medição de peso anexada). Se for implementar: relatório completo com medição antes/depois e testes.

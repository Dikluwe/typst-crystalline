# Passo 1360 — `compile --pretty` por target, com execução limitada

## Objetivo

Materializar a opção tri-state `--pretty` do vanila ratificado para compile,
preservando como eixos independentes o modo de operadores PDF (`--compact`),
tagging, target, feature HTML e modo de escaping HTML.

## Ordem causal

1. congelar a medição em `diagnosticos/p1360-preflight.md`;
2. atualizar os Prompts L0 proprietários antes de código;
3. congelar contrato, oráculos, mutantes, manifesto e selo;
4. escrever testes independentes e confirmar RED somente com filtros focais;
5. implementar L2 → L4 → L3;
6. confirmar GREEN focal e executar somente gates estáticos/arquiteturais;
7. emitir veredito segregado, sem equivalência funcional geral.

## Recorte produtivo

- L2 aceita bare, `=true`, `=false` e transporta `Option<bool>`;
- L4 resolve ausência como `false` e encaminha o booleano somente à opção do
  target selecionado;
- PDF pretty deixa content streams de página sem compressão e conserva
  `StreamMode`/`PdfTags`;
- SVG reutiliza `SvgOptions.pretty`;
- HTML acrescenta indentação estrutural sem alterar conteúdo inline;
- PNG ignora semanticamente pretty.

## Contenção de memória obrigatória

- é proibido iniciar `watch`/`w` como subprocesso;
- parser de `watch`/`w` usa somente `Args::try_parse_from`;
- qualquer processo de produto permitido usa apenas `compile`, `timeout <=
  20s`, stdout/stderr em ficheiro e artefacto individual limitado a 4 MiB;
- não executar suíte workspace ampla nem filtro textual que capture testes de
  watch preexistentes;
- executar no máximo um crate/teste focal por vez; `CARGO_BUILD_JOBS=1`;
- após RED, não repetir compilação integral se a causa já estiver localizada;
- `Unknown`, timeout ou limite excedido recebem crédito zero e interrompem o
  gate dinâmico.

## Aceitação

- parsing tri-state e diagnósticos equivalentes ao vanila;
- SVG/HTML: default=false e false compactos; true adiciona whitespace
  estrutural sem alterar a árvore observável;
- PDF: true remove `FlateDecode` dos content streams de página e preserva os
  operadores; false/default conservam compressão quando rentável;
- quatro combinações `pretty × compact` e `pretty × tagging` permanecem
  expressáveis;
- PNG preserva assinatura e morfologia;
- mutation score 1.0 no contrato selado;
- `cargo fmt --check`, `git diff --check` e `crystalline-lint .` passam.

## Fora do recorte

Seleção de páginas, standards PDF, igualdade byte a byte com krilla, bundle,
pretty configurado dentro da linguagem e reformatação de eval/query/info.


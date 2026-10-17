# L0 — stdlib/shapes/paint
Hash do Código: d23e5a56

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/stroke-paint-inputs.toml sha256:eef6bb1d6fb79379c1e42e22ef305bc31d31dab01539ba4e9fcffe49587c3c4f

## Ownership e medição

Camada L1. Consumer produtivo único: `01_core/src/compiler/stdlib/shapes/paint.rs`. No HEAD `a5c6b93c258acb7cb378b9b147a8c6cd0e8d17b9` e árvore limpa, `01_core/src/compiler/stdlib/shapes.rs:26-76` contém o comportamento extraído. SHA-256, horário e estado estão em `00_nucleo/diagnosticos/atomizacao-shapes-2026-09-30-before.json.gz`. Esta é separação interna por comportamento.

## Contrato

Converter cores tipadas, Paint e Stroke para shapes, preservando tipo, espessura, diagnostics e o fallback somente quando contratado.

As regras detalhadas, evidências históricas, limites de paridade e diagnósticos continuam em `00_nucleo/prompts/compiler/stdlib/shapes.md`. A extração não altera assinatura pública, defaults, fase ou compatibilidade. Nenhum I/O ou estado global mutável.

## Aceitação

Preservar a suíte L1 e os testes de shapes, compilar e passar `crystalline-lint .` com ownership e hashes corretos. Não inferir ganho de desempenho.

## Contornos com Paint e dicionário uniforme — 2026-10-14

### Medição anterior à decisão

before/results.json.gz no diagnóstico stroke-paint-2026-10-14 conserva HEAD,
working tree, diffstat, UTC, fontes/owners, comandos e hashes dos dois binários.
layout/stroke.rs:53-63 limita named paint a Color; shapes/paint.rs:45-64 entrega
os demais paints ao caster legado que só aceita Length/Color/Stroke;
shapes/rect.rs:40-43 descarta todos os Dict. O controle uniforme renderiza 1pt
preto em Tekt; named Gradient/Tiling/Auto e circle Gradient são rejeitados.
O binário vanilla pinado 9f2b6e871 aceita esses valores. Dict vazio desenha
contorno preto em Tekt e nenhum contorno no vanilla. A fonte pinada stroke.rs:
236-243 e sides.rs:238-258 distingue Paint smart, dict vazio e dict uniforme.
A medição de square(size:...) falha por dimensão no Tekt e não prova cast de
stroke; suas dimensões e defaults permanecem abertos, sem mudar este contrato.

### Responsabilidade e aceitação

parse_shape_stroke encaminha Gradient, Tiling e Dict uniformes ao conversor
positional_stroke existente, sem reduzir Paint para cor. Preservar os caminhos
legados Color/Length/Stroke e seu tratamento de espessura, além da rejeição
específica de String. O argumento de ausência é responsabilidade do constructor.
A conversão uniforme já implementada valida campos ricos e chaves desconhecidas.
Não modificar o caster genérico de layout nem introduzir lógica de render.

Correção interna de paridade ADR-0127 em fluxo contínuo: L0 primeiro, RED→GREEN,
resselo restrito, build, suíte e linhagem. Aceitação preserva significado da
pintura e sua forma disponível; igualdade binária com vanilla não é requisito.
Recolher PDF/raster e erros, conservando as divergências não corrigidas.

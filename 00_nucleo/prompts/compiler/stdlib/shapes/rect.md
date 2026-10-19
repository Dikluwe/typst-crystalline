# L0 — stdlib/shapes/rect
Hash do Código: 6f53e379

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/shapes/optional-stroke.toml sha256:2e8b819c96592ad1c548acb8736b6cc880b1db0e9e8866f7dc9068ed5483b1e3

## Ownership e medição

Camada L1. Consumer produtivo único: `01_core/src/compiler/stdlib/shapes/rect.rs`. No HEAD `a5c6b93c258acb7cb378b9b147a8c6cd0e8d17b9` e árvore limpa, `01_core/src/compiler/stdlib/shapes.rs:77-228` contém o comportamento extraído. SHA-256, horário e estado estão em `lab/results/construction/diagnostics/atomizacao-shapes-2026-09-30-before.json.gz`. Esta é separação interna por comportamento.

## Contrato

Construir rect e square com width/height, fill, stroke e radius, preservando defaults e o Rect comum sem tipo Square novo.

As regras detalhadas, evidências históricas, limites de paridade e diagnósticos continuam em `00_nucleo/prompts/compiler/stdlib/shapes.md`. A extração não altera assinatura pública, defaults, fase ou compatibilidade. Nenhum I/O ou estado global mutável.

## Aceitação

Preservar a suíte L1 e os testes de shapes, compilar e passar `crystalline-lint .` com ownership e hashes corretos. Não inferir ganho de desempenho.


## Correção de ausência explícita — 2026-10-06

Medição anterior à decisão: as sondas visual-rect/visual-circle em
`lab/results/construction/diagnostics/paridade-dominios-2026-10-06/measurement.json.gz`
compilam no vanilla pinado 9f2b6e871 e são rejeitadas pelo Tekt no cast de stroke.
`lab/typst-original/crates/typst-layout/src/shapes.rs:125-129,335-339` distingue
Auto condicionado a fill de Custom(None). O consumer sempre enviava None ao
parser de Stroke obrigatório e seu fallback ignorava a presença do argumento.
Sem inferir intenção histórica, corrigir aceitação/forma da linguagem.

Aplicar o núcleo optional-stroke na leitura do argumento e na decisão de
fallback dos dois construtores deste owner. Não modificar o parser obrigatório
compartilhado, strokes por-lado ainda não representados, assinaturas, dimensões
ou radius. Testar None com e sem fill e omissão sem fill, RED→GREEN.
Correção interna de paridade ADR-0127: fluxo contínuo, sem API/fase/default novo,
unsafe ou dependência. Revalidar PDF/raster e a matriz macro contra o pin atual.

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

Distinguir dicionário uniforme não vazio de dicionário por lados: paint,
thickness, cap, join, dash e miter-limit pertencem ao stroke uniforme e devem
ser encaminhados ao parse_shape_stroke; chaves uniformes desconhecidas ou paint
inválido não podem ser silenciosamente descartados. Em rect e square, (:)
é ausência explícita de contorno, tal como none para efeito do fallback, mesmo
sem fill. Omissão conserva seu fallback. Para square usar largura existente,
sem introduzir size/default de dimensão nesta correção.

Dicionários com left/top/right/bottom/x/y/rest continuam a pertencer à lacuna
preexistente por-lado da entidade uniforme. Esta correção não afirma que seu
render ou validação completa está correto, inclusive misturas de chaves; não
transformar seus valores em stroke uniforme. Requer decisão/materialização
própria. O scope-out anterior não autoriza descartar todos os dicionários.

Correção interna de paridade ADR-0127 em fluxo contínuo: L0 primeiro, RED→GREEN,
resselo restrito, build, suíte e linhagem. Aceitação preserva significado da
pintura e sua forma disponível; igualdade binária com vanilla não é requisito.
Recolher PDF/raster e erros, conservando as divergências não corrigidas.

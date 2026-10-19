# L0 — stdlib/shapes/ellipse
Hash do Código: 83dcf664

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/shapes/optional-stroke.toml sha256:2e8b819c96592ad1c548acb8736b6cc880b1db0e9e8866f7dc9068ed5483b1e3

## Ownership e medição

Camada L1. Consumer produtivo único: `01_core/src/compiler/stdlib/shapes/ellipse.rs`. No HEAD `a5c6b93c258acb7cb378b9b147a8c6cd0e8d17b9` e árvore limpa, `01_core/src/compiler/stdlib/shapes.rs:229-350` contém o comportamento extraído. SHA-256, horário e estado estão em `lab/results/construction/diagnostics/atomizacao-shapes-2026-09-30-before.json.gz`. Esta é separação interna por comportamento.

## Contrato

Construir ellipse e circle; circle converte raio em diâmetro e usa ShapeKind::Ellipse. Preservar fill/stroke e fallback.

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

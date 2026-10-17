# Memória 5 — pacote L0 proposto para compactar Value

Status: pronto para revisão; aguarda confirmação humana ADR-0127 antes de importar rascunhos L0 e materializar. Nenhum código L1/L2/L3/L4, header ou L0 ativo alterado. Os 14 rascunhos completos conservam owners 1:1, núcleos e pins existentes. Não ressellar código antigo contra a forma nova.

## Medição anterior à proposta

HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado. Estado integral, status/stat e UTC: `value-compacto-memoria5-2026-10-08/before.json.gz`. Binário atual pós-etapa 4 SHA-256 `564b1fe69f92880736485e882e2e1409a7a9df59712632783e6eae5f74470784`, build ratificado no diagnóstico da etapa 4. A sonda externa usa rlib release atual, dependências únicas, hashes/argv/UTC registrados em sizes.json. Não é implementação do compilador nem teste de desempenho CLI.

| Tipo atual | sizeof bytes | align bytes |
|---|---:|---:|
| Value | 272 | 16 |
| Binding | 272 | 16 |
| Stroke | 264 | 8 |
| Paint | 200 | 8 |
| Tiling | 200 | 8 |
| Content | 120 | 8 |
| IntrospectedContent | 128 | 8 |
| Func | 16 | 8 |
| Vec<Value> (header) | 24 | 8 |

Value::Stroke inline domina a largura (`entities/value.rs:111`); Stroke inclui Paint, cujo payload Tiling já tem 200 bytes. Tiling/Stroke/Paint possuem campos/variantes públicos, portanto compactá-los também requer gate. Mudar apenas armazenamento privado de IntrospectedContent não remove o maior payload de Value; Func já usa Arc. Compartilhar Array alteraria outra variante pública e tem fan-out maior, fora deste pacote.

Modelo isolado com todas as variantes atuais e apenas `Stroke(Arc<Stroke>)` mede 144 bytes/align16 (layout-probe.rs.txt, sizes.json). Redução física da largura: (272−144)/272 ≈47,06%. O modelo usa os tipos reais ligados ao rlib; é uma medição de layout no host, sem alterar o Value real. Não comprova redução de 47% do heap/RSS ou ganho de velocidade, nem ABI portátil. Para arrays/IndexMap/capturas há hipótese de menor densidade por slot; strokes únicos passam a ter alocação própria e podem consumir mais memória total.

## Contrato público proposto

```rust
// Somente o payload desta variante muda:
Value::Stroke(Arc<Stroke>)
// Assinatura existente preservada:
impl From<Stroke> for Value
```

O From transfere o Stroke para novo Arc. Clone de Value compartilha a carga. Leituras projetam referência Stroke; conversões que retornam ou modificam Stroke produzem valor próprio. O fold de Style/TableCell substitui somente o payload do delta, preservando o base e snapshots antigos. Não usar igualdade de ponteiro como igualdade/identidade de linguagem.

Sem alteração de semântica/sintaxe/morfologia, type/repr/fields, specified, defaults, errors, phases, flags, geometry Stroke/Paint/Tiling, campos dos carriers TableCell/MathMatrix, Array/Dict ou métodos/traits públicos. Há **quebra de compatibilidade Rust** para construção/desestruturação/mutação direta da variante: essa é a razão da confirmação obrigatória. A ponte From<Stroke> permanece.

## Adaptação autorizável por owner

| Consumer | L0 canônico de destino | Hash de prompt proposto |
|---|---|---|
| `02_shell/src/cli.rs` | `00_nucleo/prompts/shell/cli.md` | `e2b1a2c0` |
| `01_core/src/entities/elements/table_cell.rs` | `00_nucleo/prompts/entities/elements/table_cell.md` | `3aefebaa` |
| `01_core/src/entities/elements/p1431_math_matrix_tests.rs` | `00_nucleo/prompts/entities/elements/p1431-math-matrix-tests.md` | `1ffe4d4a` |
| `01_core/src/entities/elements/math_matrix.rs` | `00_nucleo/prompts/entities/elements/math_matrix.md` | `c27a9ea8` |
| `03_infra/src/export/p1431_line_cap_tests.rs` | `00_nucleo/prompts/infra/export/p1431-line-cap-tests.md` | `af5f52bd` |
| `01_core/src/entities/value.rs` | `00_nucleo/prompts/entities/value.md` | `0487cc06` |
| `01_core/src/compiler/math/layout/p1431_tests.rs` | `00_nucleo/prompts/compiler/math/layout/p1431-tests.md` | `5407c594` |
| `01_core/src/compiler/stdlib/tests/stroke.rs` | `00_nucleo/prompts/compiler/stdlib/tests/stroke.md` | `fea9af95` |
| `01_core/src/compiler/stdlib/tests/shapes.rs` | `00_nucleo/prompts/compiler/stdlib/tests/shapes.md` | `81e01ec2` |
| `01_core/src/compiler/stdlib/layout/stroke.rs` | `00_nucleo/prompts/compiler/stdlib/layout/stroke.md` | `4f6d8e91` |
| `01_core/src/compiler/stdlib/layout/casting.rs` | `00_nucleo/prompts/compiler/stdlib/layout/casting.md` | `4e6fb1cc` |
| `01_core/src/compiler/eval/repr.rs` | `00_nucleo/prompts/compiler/eval/repr.md` | `a95a5125` |
| `01_core/src/compiler/eval/operators/arithmetic.rs` | `00_nucleo/prompts/compiler/eval/operators/arithmetic.md` | `ca5021f5` |
| `01_core/src/compiler/eval/rules/set.rs` | `00_nucleo/prompts/compiler/eval/rules/set.md` | `86f5b2a0` |

Os arquivos completos estão em `value-compacto-memoria5-2026-10-08/l0-drafts/`, com espelho dos caminhos canônicos. Manifesto enumera cada destino, SHA antes/depois, owner e prompt hash calculado com pins. Depois de confirmação: importar os drafts, escrever testes RED, adaptar esses consumers, calcular CodeHash e ressellar somente os pares realmente materializados. Outros leitores que só fazem match/read continuam sob seus L0s vigentes e devem compilar sem ampliar escopo. Inventário completo de matches/owners: inventory.json (25 consumers, incluindo leitores/comentários que não precisam mudar). Nenhum Núcleo novo necessário; o carrier pertence exclusivamente ao owner Value.

## Execução e aceitação depois da confirmação

1. Congelar os estados atuais novamente se algo tiver mudado e importar os 14 drafts confirmados. Fonte baseline deve permanecer rastreável; nunca sobrescrever a medição original.
2. Testes antes do código: footprint reduzido no host/alvo medido, clone que projeta a mesma carga por referência (RED antes, GREEN depois), reconstrução/modificação isolada de um alias, snapshot de introspecção/Style, componentes Stroke completos. Testemunha física é critério de RAM, não paridade.
3. Usar From<Stroke> nos construtores existentes e adaptações específicas dos L0s: casts para Stroke próprio; fold com Option<&Stroke>; getters de TableCell/MathMatrix; testes de repr, stroke, shapes, matemática e export. Não relaxar expectativas nem redefinir defaults para compilar.
4. Preservar igualdade/hash de linguagem por seus owners; testes type/repr/fields, cap/join/dash/miter/overhang/provided bits, gradient/tiling, avisos/erros, snapshots, closures/capturas e context causal. Core/infra/shell suites necessárias, builds dev/release e lint (ownership/grafo antes de hashes). Considerar stack dos testes herdados: 64 MiB para suite core/infra, não config do produto.
5. Comparação CLI pareada: bateria de 11 casos da etapa 4, caso com arrays/dicts/escopos grandes, strokes únicos e clonados, controle sem strokes/arrays. Mesmas fontes, CPU, env e binários congelados; não executar simultâneo a builds/testes/GDB. `%M` + wall, blocos alternados, texto/geom/estilos/raster e SVG/PNG/HTML nos escopos pertinentes. Capturar compatibilidade específica de Stroke::from, getters e retorno a geometria própria.
6. Conservar apenas com ganho medido e custo de strokes explícito. Nenhuma promessa de speedup/RSS baseada somente na sonda de tamanho. Não somar percentuais com etapas anteriores; não publicar paridade global.

## Gate e manifesto

A ADR-0127 determina “PARAGEM OBRIGATÓRIA antes de escrever código” para compatibilidade/contrato público. A alteração da variante muda a forma pública e quebra código Rust que passa Stroke diretamente; a autorização genérica para executar prioridade 5 não substitui confirmação deste pacote específico. Rascunhos separados mantêm a linhagem ativa válida enquanto se revisa a proposta.

Manifesto SHA-256: `6badf07a4f58f08e7c60ad2e34dfb9a14a3a6ba55db90045afe046be8e988237`. Confirmar este manifesto autoriza importar seus L0s e executar o plano acima. Não pressupõe ganho de performance: a decisão final continua condicionada à medição.

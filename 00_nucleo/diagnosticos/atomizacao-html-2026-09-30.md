# Atomização de `stdlib/html` — 2026-09-30

## Proveniência

Estado inicial: commit `7a813038b9af2eca0eac7c9a0f808ae8f6509d5b`, árvore limpa, código com 3233 linhas e SHA-256 `71aba1e05ca2d8e728ec0aacfca55b4273d3985d67fff1802a7ef1aba4c66be7`. O manifesto [atomizacao-html-2026-09-30-before.json.gz](atomizacao-html-2026-09-30-before.json.gz) registra também o hash do L0 e o horário UTC. Estado final: working tree não commitada do mesmo HEAD, medida em 2026-09-30T20:44:38.195253+00:00; [atomizacao-html-2026-09-30-after.json.gz](atomizacao-html-2026-09-30-after.json.gz) guarda a lista exata de alterações, os caminhos, linhas e SHA-256 de cada arquivo de código. Estes números descrevem estrutura, não desempenho.

## Unidades

A fachada mantém a interface pública, o registro dos módulos e os testes históricos. As funções, tabelas e coerções passam a consumers da mesma camada L1 com ownership 1:1 em `00_nucleo/prompts/compiler/stdlib/html/`.

| Unidade | Linhas finais |
|---|---:|
| `fachada` | 1179 |
| `namespace` | 34 |
| `frame` | 41 |
| `typed_tags` | 342 |
| `attr_specs` | 53 |
| `tag_attrs` | 701 |
| `global_attrs` | 244 |
| `typed_element` | 98 |
| `attr_cast` | 210 |
| `attr_values` | 362 |
| `elem` | 94 |

O registro de 112 tags e as categorias de atributos continuam estáticos. `attr_specs` contém apenas os tipos, `tag_attrs` as tabelas específicas e `global_attrs` as tabelas globais; isso permite procurar alternativas para coerção e validação sem percorrer o módulo inteiro. Os arquivos resultantes somam 3358 linhas, incluindo headers, delegação e o teste novo.

## Verificação

- Teste novo: RED antes da extração; GREEN após mover os blocos e separar as tabelas.
- Suíte `cargo test -p typst-core --lib --offline --locked --quiet`: **6448 passaram, 2 ignorados**.
- `cargo build --offline --locked --quiet`: passou.
- `crystalline-lint .`: passou com zero violações.
- `git diff --check`: passou.

A mudança é interna; preserva contratos públicos, defaults, fase do pipeline e semântica observada pelos testes. Não foi feito benchmark nem alegação de ganho de tempo ou memória.

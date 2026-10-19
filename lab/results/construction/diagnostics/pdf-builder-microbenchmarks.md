# Microbenchmarks do builder PDF

`benches/pdf_builder_bench.rs` organiza fixtures por responsabilidade do
builder atomizado. A construcao de documentos, os bytes de imagens e a
preparacao dos vetores de fontes ficam fora das closures cronometradas.
Cada iteracao exporta um PDF inteiro e consome os bytes com `black_box`.
Os grupos nao isolam tempos de fases internas nem incluem eval/layout.
Os IDs começam em `pdf_export_cohorts/builder/`; todos incluem discovery,
construcao de objetos, emissao e serializacao, inclusive os casos de
paginas vazias e fontes. O perfil debug e rejeitado antes de medir.

| Grupo | Fixture e caminho exercitado |
|---|---|
| `text_serialization_modes` | Documento curto e texto repetido; Compact e Pretty |
| `empty_pages` | Paginas vazias em escala; xref, objetos e trailer |
| `metadata` | Titulo Unicode, autor e keywords; Info e XMP |
| `navigation` | Links URL em documento multipagina |
| `resources` | Reutilizacao do mesmo JPEG RGB como XObject e ICC |
| `fonts/single` | Texto fallback com uma fonte embutida; subset/CID |
| `fonts/multifont_path` | Texto alternado entre duas fontes embutidas |

Execucao completa:

```sh
cargo bench -p typst-benches --bench pdf_builder_bench
```

Filtro por familia e validacao rapida das fixtures:

```sh
cargo bench -p typst-benches --bench pdf_builder_bench -- pdf_export_cohorts/builder/fonts
cargo bench -p typst-benches --bench pdf_builder_bench -- --test
```

Nao comparar `resources` diretamente com `navigation` como custo de uma
fase: as fixtures possuem quantidades diferentes de conteudo. Para medir
uma regressao, comparar a mesma fixture e configuracao entre revisoes,
registrando commit, diff completo da working tree, toolchain, maquina,
comando e horario. O smoke test confirma execucao, nao desempenho.

PDF/A, PDF/UA, gradients, anexos, CFF2/variaveis e bitmap-only continuam
cobertos pelas suites funcionais existentes; este benchmark nao mede
isoladamente essas variantes nem prova sua conformidade.

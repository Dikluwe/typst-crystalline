# Documentação de API e formatação Rustdoc

Correção dos comentários de documentação que bloqueavam o Clippy no core.
Nenhuma assinatura, expressão, corpo de função, teste ou contrato L0 foi
alterado. Os selos reversos foram recalculados somente para os owners afetados.

## Proveniência e medição anterior à decisão

Branch `Tekt`, HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, **working tree
não commitado**. O diretório `rustdoc-ci-2026-10-13/` preserva UTC, HEAD,
status, `git diff HEAD --stat`, fontes e L0 em `before.json.gz`,
`pre-validation.json.gz` e `after.json.gz`. Receitas, logs, inventários,
diffs delimitados e resultados ficam em `evidence.json.gz`.

O inventário anterior, reproduzido no relatório `pre-shaping-ci-2026-10-13.md`,
continha 446 diagnósticos distintos no core lib/testes. Desses, 39 eram
`doc_lazy_continuation` e 11 eram `empty_line_after_doc_comments`. A contagem
desduplica código, mensagem e spans primários completos; não soma as emissões
repetidas de lib e lib/testes. Não é uma contagem completa do workspace.

Antes da edição, a inspeção encontrou:

- `compiler/stdlib/primitives_constructors/decimal.rs:36`: documentação de
  `duration` ligada ao módulo de testes de decimal.
- `compiler/stdlib/primitives_constructors/duration.rs:104`: documentação de
  `version` ligada a `parse_duration`.
- `compiler/stdlib/primitives_constructors/version.rs:190`: documentação do
  parser de duração ligada aos testes de version.
- `entities/content/construction.rs:34`: normalização de sequências descrita
  em `Content::strong`, embora o owner vigente exclua essa responsabilidade.
- `compiler/eval/mod.rs:985` e `compiler/stdlib/counter.rs:545`: notas de
  extração histórica anexadas pelo Rustdoc ao item seguinte.

Os owners de duration/version/decimal e content construction/sequence já
especificam as responsabilidades corretas. A classificação é manutenção de
documentação e vinculação, sem decisão nova de arquitetura ou de paridade.
Não foi necessária mudança normativa de L0.

## Correção

Os comentários de construtores e do parser retornaram às respectivas funções.
O comentário de duração agora reconhece os argumentos nomeados negativos já
previstos no L0; o parser documenta os tipos efetivamente usados (`i64` e
`f64` finito), sem a antiga afirmação incorreta de `u64` não negativo.
O texto de version não apresenta uma versão solta como prova de paridade.

Notas históricas de funções extraídas passaram a comentários comuns. Listas
ganharam separação de parágrafos ou continuação indentada conforme o sentido.
Sinais de adição no início de linhas foram escapados para não abrir listas
acidentais. Separadores vazios foram removidos quando a documentação já
pertencia à função seguinte.

`final-scope.json` comprova que as linhas não vazias que não são comentários
inteiros permanecem idênticas nos 22 arquivos Rust afetados. Nos 22 owners,
a única diferença é `Hash do Código`. Essa comparação delimita a edição;
não constitui prova nova de paridade da linguagem.

## Validação

Medição encerrada em `2026-10-13T23:01:27.208832+00:00`, sem mudança nas
fontes durante os comandos. Resultados finais e comandos estão em
`summary.json` e `validation-results.json`, preservados nas evidências.

| Verificação | Resultado |
| --- | --- |
| Clippy core lib/testes com `-D warnings` | 446 → 396 diagnósticos distintos; ainda falha |
| `doc_lazy_continuation` / `empty_line_after_doc_comments` | 39 / 11 → zero; demais contagens inalteradas |
| `cargo build --offline --locked` | passou |
| Rustdoc workspace, sem dependências, `RUSTDOCFLAGS=-Dwarnings` | passou |
| `cargo fmt --check --all` e `git diff --check` | passaram |
| Linhagem estrita V1/V5/V15/V26 | passou |
| Linhagem reversa | 965 consumers, zero erros |
| Linter completo | zero erros, 381 avisos, 1.619 notas |

Os trechos de HTML gerados de `native_duration`, `native_version`,
`Content::strong` e `Content::sequence` também foram conferidos e preservados
em `rendered-docs.json`: os comentários pertencem aos itens esperados.

Não foram criados testes de comportamento para uma alteração exclusivamente
de comentários. A suíte completa não foi reexecutada nesta etapa.

## Pendências

O CI geral permanece bloqueado pelos demais diagnósticos do Clippy.
Os avisos arquiteturais, lacunas funcionais da auditoria de produção e a
matriz remota de release continuam abertos. Build e documentação locais não
certificam os artefatos Linux/Windows/macOS/i686/cross/Docker.

A medição seguinte está em [imports e mutabilidade local](imports-ci-2026-10-13.md).

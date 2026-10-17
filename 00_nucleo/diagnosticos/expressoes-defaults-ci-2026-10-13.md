# Referências, retornos e Default equivalentes

Este lote simplifica referências e retornos redundantes, substitui clones
usados somente para construir slices de leitura em testes e deriva Default
onde o trait já existia com o mesmo valor. Não escolhe novos defaults do
produto nem modifica contratos públicos ou fases.

## Proveniência e medição anterior à decisão

Branch `Tekt`, HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, **working tree
não commitado**. `expressoes-defaults-ci-2026-10-13/before.json.gz` conserva
UTC, status, `git diff HEAD --stat`, fontes, L0 e sugestões inspecionadas.
`pre-validation.json.gz` e `after.json.gz` delimitam a validação. Logs,
receitas, inventários e diffs ficam em `evidence.json.gz`.

O inventário de entrada do lote `helpers-testes-ci` contém 183 diagnósticos
distintos no core lib/testes. Foram selecionados 43: dez needless_return,
nove derivable_impls, oito needless_borrow, cinco cloned_ref_to_slice_refs,
três default_constructed_unit_structs, dois get_first, dois useless_format,
e um de cada needless_question_mark, redundant_closure, map_identity e
borrow_deref_ref. Cada span foi conferido contra a fonte corrente antes da
edição. Linhagem estrita e registro de testes passaram nessa baseline.

## Default: mesmo trait e mesmo valor

As implementações anteriores foram lidas antes da substituição. Nenhum
desses tipos é genérico; a derivação não acrescenta bounds a parâmetros de
API. Foram preservados os campos e as variantes, marcando explicitamente
`#[default]` nas variantes selecionadas.

| Tipo | Valor obrigatório preservado |
| --- | --- |
| CitationForm | Normal |
| CitationStyle | Numeric |
| PageMarginal | Auto |
| PageSupplement | Auto |
| ListMarker | Default, que renderiza o bullet existente |
| Sink | diagnostics e seen vazios |
| Decimal | InnerDecimal::default() |
| Version | components vazio |
| MockWorld privado dos testes de plugin | files vazio; library e book por seus defaults existentes |

Os nove owners receberam essa decisão antes do código. Os exemplos antigos
de impl Default continuam representando o valor exigido; os novos parágrafos
normativos autorizam a derivação equivalente. Nos outros 21 owners, somente
o Hash do Código foi alterado. O resselo é restrito aos 30 consumers.

`Version` conserva PARITY_VERSION e seus impls manuais de igualdade, ordem
e hash com zero-pad. `PageNumberAlign::default` continua manual e center/bottom.
`Sink` conserva Clone e os métodos de deduplicação/comemo. A mudança em
CitationStyle não resolve nem reclassifica diferenças bibliográficas contra
o vanilla; o valor Numeric permanece exatamente o mesmo.

## Simplificações de expressão e fixtures

Os retornos removidos eram finais de função ou de seu match final, como em
`content_methods.rs:45-48`, `rules/set/page.rs:531` e os casts CSV. As chamadas,
resultados, campos e mensagens permanecem; não foi retirada uma saída
antecipada de um loop ou bloco intermediário.

As referências extras eram coerções redundantes sobre referências já
existentes: argumentos de geometria de células, referência de função em repr
e reborrow de Content em introspect. `get(0)` passa a `first()` mantendo
Option e tratamento de ausência. A remoção de `map_err(|e| e)` e do par
`Ok(...?)` conserva o mesmo tipo de erro. A closure de is_node_rule não
capturava estado. Os dois format! de literais passam a to_string com os
mesmos bytes das mensagens/fontes de teste.

As cinco sugestões de slice estavam marcadas MaybeIncorrect pelo Clippy e
foram examinadas individualmente. As funções chamadas só leem os valores
para essas fixtures; os testes verificam normalização, geometria/tracking e
identidade de seletor, não Clone. `std::slice::from_ref` preserva o valor e
sua disponibilidade para o uso posterior. Em particular,
`grid/measurement_cache.rs:139-163` não retém a referência ao slice: lê
predicados de placement e cria metadados próprios. Esse consumer e os tipos
Content/Value/FrameItem foram preservados como fontes somente de leitura.

As três construções de tipos unitários envolvem NullWorld derivado no
próprio módulo de testes e FxBuildHasher. O Cargo.lock fixa rustc-hash 2.1.1;
a fonte local dessa versão declara FxBuildHasher como unit struct com
Default derivado. A fonte e seu hash foram incluídos no snapshot. Não foi
retirada uma chamada a um Default customizado com efeitos.

Esta etapa não introduz allow de lint, muda expectativa de teste ou afirma
ganho de desempenho. A verificação de paridade do projeto continua usando
os contratos existentes; não foi feita nova comparação bilateral aqui.

## Validação final

A rodada principal terminou em `2026-10-13T23:50:20.290072+00:00`; a documentação
encerrou em `2026-10-13T23:50:33.153447+00:00`. Fontes e owners ficaram
inalterados durante a validação. `validation-results.json` e
`rustdoc-results.json` preservam comandos, horários e códigos de saída.

| Verificação | Resultado |
| --- | --- |
| Clippy core lib/testes com `-D warnings` | 183 → 140 diagnósticos distintos; ainda falha |
| Categorias selecionadas | 43 diagnósticos removidos; demais contagens inalteradas |
| Check de todos os targets do workspace | passou |
| Suíte core | 6.701 passaram, zero falhas, cinco ignorados |
| Registro de testes | 6.706 nomes, idênticos antes e depois |
| Build, formatação e `git diff --check` | passaram |
| Rustdoc workspace sem dependências, `RUSTDOCFLAGS=-D warnings` | passou |
| Linhagem estrita V1/V5/V15/V26 | passou |
| Linhagem reversa | 965 consumers, zero erros |
| Linter completo | 0 erros, 381 avisos, 1618 notas |

Não foram adicionados testes que espelhem derives nem removidos testes
existentes. As expectativas de defaults, deduplicação, geometria e
normalização continuam verificadas pela suíte. Os 43 diagnósticos não
representam 43 defeitos comportamentais demonstrados.

## Pendências

Continuam abertos os 140 diagnósticos restantes do Clippy do core, os avisos
arquiteturais, as lacunas funcionais da auditoria original e a matriz remota
de release. Este resultado não certifica toda a matriz multiplataforma.
Não houve commit, push, publicação ou benchmark nesta etapa.

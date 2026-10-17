# Helpers exclusivos de testes e armazenamento sem leitura

Doze helpers internos utilizados apenas por testes passaram a compilar sob
`#[cfg(test)]`. Também foram retirados dois campos privados sem leitura de
`GridGeometry` e três helpers de teste sem chamadas. Os cálculos de matriz,
os entrypoints produtivos e as expectativas dos testes foram preservados.

## Proveniência e medição anterior à decisão

Branch `Tekt`, HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, **working tree
não commitado**. `helpers-testes-ci-2026-10-13/before.json.gz` registra UTC,
status, `git diff HEAD --stat`, fontes, L0, referências e spans. O inventário
de entrada `ordem-testes-ci` tem 200 diagnósticos distintos no core lib/testes,
dos quais 23 são `dead_code`. Este lote seleciona 17 desses diagnósticos;
os demais não são considerados resolvidos por esta classificação.

Cada span foi conferido com a fonte atual. Antes de editar, passaram a
linhagem estrita e a listagem de testes; comandos/horários ficam em
`baseline-results.json`. As versões anteriores e posteriores das fontes e
owners ficam em `before.json.gz`, `pre-validation.json.gz`, `after.json.gz`;
logs, inventários, receitas e diffs em `evidence.json.gz`.

## Inspeção de chamadas e decisão

A pesquisa de referências distingue wrappers de teste e seus caminhos
produtivos. Nenhuma das assinaturas limitadas neste lote é API externa
alcançável: são privadas, `pub(super)`, ou pertencem a módulos privados ou
`pub(crate)` sem reexport externo. As referências examinadas nas camadas
L2–L4 não usam esses wrappers.

| Helper restrito aos testes | Caminho produtivo preservado |
| --- | --- |
| eval::make_stdlib | make_stdlib_with_features |
| bibliography::load_bib_entries_from_path | nativa bibliography lê a origem e chama parse_bibliography; resolve_style permanece |
| layout::helpers::line_content_bottom | demais helpers de posição/extensão permanecem; a recursão do helper fica disponível ao teste existente |
| layout_accent | layout_accent_elem |
| layout_cancel | layout_cancel_elem com ocorrência |
| layout_cases | layout_cases_elem |
| lr_target_pt | lr_requested_target_pt e seleção produtiva de delimitadores |
| layout_frac | layout_frac_with_line e variantes de estilo |
| layout_matrix | layout_matrix_elem |
| spacing_between | spacing_between_class por compute_gaps |
| decode_cbor | decode_cbor_with_source e nativas CBOR |
| decode_csv | decode_csv_impl e nativa CSV com Bytes/File |

No estado anterior, `math/layout/mod.rs:1005,1056,1058,1061,1066` mostra o
despacho matemático produtivo. `stdlib/loading.rs:42-47` já condicionava as
reexportações dos wrappers de CBOR/CSV a testes.
`stdlib/structural/bibliography.rs:300` chama o parser produtivo.
Esses arquivos foram preservados byte a byte como fontes somente de leitura.

Os wrappers conservam seus corpos e suas assinaturas quando compilados
para testes. Em particular, a matriz mantém o cap Butt de compatibilidade
no wrapper, sem propagá-lo ao default produtivo. O wrapper de acento mantém
sua fixture histórica; o carrier completo continua resolvendo dotless.

`CsvErrorContext::Pure` só era construído por decode_csv. Sua variante e seu
braço de match acompanham `cfg(test)`; Bytes/File continuam com os mesmos
spans e mensagens. Não foi introduzido um allow de lint.

## Campos e fixtures retirados

Em `math/layout/matrix.rs:377-378`, row_ascent e row_descent eram armazenados
em GridGeometry sem leitura posterior. Os vetores locais em measure continuam
calculados e usados nas fórmulas de baseline e height. Foram removidos apenas
os campos privados e as respectivas entradas do inicializador. Não há
medição de desempenho ou alegação de equivalência geral de geometria.

Foram removidos somente ck em `introspect/fixpoint.rs:170`, bare_diagnostic
no módulo `calc::p1334_tests` em `stdlib/calc.rs:2835`, e DummyDownloader com
seu impl local em `contracts/package_downloader.rs:93`. Não tinham chamadas
ou instanciações. Os homônimos bare_diagnostic usados em outros módulos de
calc permanecem, assim como as assertivas de diagnóstico inlined em p1334.
O trait público PackageDownloader e seus testes de Display não mudam.

## L0 e alcance

Os 15 owners receberam a decisão específica antes da edição de Rust. A
classificação segue o fluxo interno da ADR-0127: sem API externa, default,
fase ou quebra de compatibilidade. O resselo foi limitado aos consumers
nomeados; headers e hashes correspondem aos L0 atualizados. Não foi criado
novo consumer ou núcleo. Esta etapa não é uma comparação com o vanilla.

## Ajuste encontrado na primeira validação

A primeira rodada terminou com 184 diagnósticos: os 17 `dead_code`
selecionados desapareceram, mas MathCancelExplicit ficou importado na
compilação normal embora somente o wrapper de teste o usasse. Essa rodada
não é o resultado final. `first-validation.json.gz` preserva suas fontes,
logs e inventários.

O L0 de cancel foi refinado antes de condicionar também esse import a
`cfg(test)`. MathCancelAngle e MathCancelElem permanecem produtivos. A
segunda rodada revalida o estado completo após esse ajuste; os diffs finais
ficam em `final-diffs.json` dentro do arquivo de evidências.

## Validação final

A segunda rodada encerrou em `2026-10-13T23:44:00.825946+00:00`; a documentação
terminou em `2026-10-13T23:44:15.331641+00:00`. Fontes e owners permaneceram
inalterados durante os comandos. `validation-results.json` e
`rustdoc-results.json` registram horários, argumentos e códigos de saída.

| Verificação | Resultado |
| --- | --- |
| Clippy core lib/testes com `-D warnings` | 200 → 183 diagnósticos distintos; ainda falha |
| Categoria dead_code | 23 → seis; demais contagens inalteradas |
| Check de todos os targets do workspace | passou |
| Suíte core | 6.701 passaram, zero falhas, cinco ignorados |
| Registro de testes | 6.706 nomes, idênticos antes e depois |
| Build, formatação e `git diff --check` | passaram |
| Rustdoc workspace sem dependências, `RUSTDOCFLAGS=-D warnings` | passou |
| Linhagem estrita V1/V5/V15/V26 | passou |
| Linhagem reversa | 965 consumers, zero erros |
| Linter completo | 0 erros, 381 avisos, 1618 notas |

Os 17 diagnósticos tratados correspondem a 12 wrappers, uma variante
privada de contexto de teste, o armazenamento dos dois campos privados de
GridGeometry (um diagnóstico) e três helpers não utilizados nos testes.
A contagem não é prova de 17 bugs comportamentais corrigidos.

## Pendências

Os 183 diagnósticos restantes do Clippy, os avisos arquiteturais, as lacunas
funcionais da auditoria original e a matriz remota de release continuam
abertos. Os seis dead_code restantes não foram ocultados com allow nem
removidos sem análise de seus contratos. Não houve commit, push, publicação
ou medição de desempenho nesta etapa.

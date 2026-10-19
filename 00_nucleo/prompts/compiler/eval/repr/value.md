# L0 — compiler/eval/repr/value
Hash do Código: d36ea5bd

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/introspection/content-snapshot.toml sha256:5a0270231de70be1212dbd17298cce34b7161b527d74b4b589e3f4c69d35ce24

## Ownership e medição

Camada L1. Consumer produtivo único: `01_core/src/compiler/eval/repr/value.rs`. No HEAD `6b147bd213e07e1dc896a8e798b84bf4d8dbbdcb` e árvore limpa, `01_core/src/compiler/eval/repr.rs:28-201 e 1516-1573` possui o comportamento separado. SHA-256, horário e estado estão em `lab/results/construction/diagnostics/atomizacao-repr-2026-09-30-before.json.gz`. O objetivo é pesquisa algorítmica por comportamento, sem ganho de desempenho presumido.

## Contrato

Formatar cada variante de Value em morfologia pública determinística. Preservar arrays limitados a 40 itens apenas no repr, dicionário vazio (:), Args causais, floats não finitos, DateTime, closures e snapshots de LocatedContent sem reconstruir campos por estilo.

O contrato detalhado e as evidências históricas de paridade continuam em `00_nucleo/prompts/compiler/eval/repr.md`; a extração não muda saída, assinatura pública ou fase. Não há I/O nem estado global mutável.

## Aceitação

Preservar testes de morfologia de linguagem, compilar e passar `crystalline-lint .` com owner e hash válidos.

## Dicionários extensos — correção interna medida

`transformacoes-2026-10-14/after-second/results.json.gz` registra rotate-fields-explicit
e skew-fields: valores iguais, mas o repr Tekt junta o dicionário numa linha; o
pin usa layout vertical acima de 50 colunas. `repr/value.rs:52` usa join direto,
embora o owner de format já implemente a regra medida em pretty_array_like.
Usar esse formatador também para Dict não vazio, preservando ordem e (:).
Morfologia de repr é observável; o helper e alocação são mecânica. Não alterar
quoting de keys nesta correção. Validar regressão extensa/nested e controles curtos.

## Uso do Display decimal — correção interna medida 2026-10-16

### Medição anterior à decisão

first-validation/validation-clippy.log em lab/results/construction/diagnostics/clippy-enums-2026-10-16 registra to_string_in_format_args em repr/value.rs no braço Value::Decimal após a implementação Display ratificada. O texto vigente é format!("decimal(\"{}\")", d.to_string()); o Display padrão do novo contrato delega ao mesmo InnerDecimal. Hashes, HEAD, working tree, UTC e diffstat estão em first-validation/validation-details.json. É construção de string interna, não nova medição de paridade.

### Responsabilidade e aceitação

Passar d diretamente ao format!, preservando decimal("...") e o texto decimal/escala/zeros/sinal. Não alterar nenhuma outra variante, regra de quoting, assinatura ou fase. Correção mecânica em fluxo contínuo ADR-0127, sem supressão ou contrato novo; preservar todos os testes de repr e rodar suíte, Clippy, build, rustdoc e linhagem com teto de 12 GiB. O diagnóstico anterior constitui RED do lint; GREEN exige ausência de novos avisos e saídas existentes preservadas.

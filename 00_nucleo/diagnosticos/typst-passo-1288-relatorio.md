# Passo 1288 — materialização de perfis e semântica acessível de tabelas

## Resultado executivo

A capacidade P1288 foi implementada e os testes funcionais do candidato estão
verdes: features `html` e `a11y-extras` permanecem opt-in e ortogonais; o trio
`pdf.table-summary`/`pdf.header-cell`/`pdf.data-cell` é gated em conjunto; a
classificação semântica de tabelas sobrevive até o PDF tagueado; tags disabled
omitem a estrutura sem alterar os observáveis visuais congelados; os três
perfis da matriz fecham com ordem forward/reverse idêntica.

O passo, contudo, **não pode ser certificado como refinado** neste checkout.
O selo pré-candidato pinou por bytes 15 Prompts L0 e declarou qualquer drift
como invalidação. Ao fim da implementação, os 15 hashes diferem: 8 diferenças
são explicadas somente pelo resselo obrigatório de `Hash do Código`, enquanto
7 também contêm alterações concorrentes de outros passos. O controle A22 do
gate adversarial detecta corretamente essa deriva e termina com exit 1. Não se
alterou retrospectivamente contrato, manifesto, oráculos ou selo para esconder
o resultado.

**Veredito do autor do relatório: NOT REFINED**

## Regime e segregação

Foi usado o protocolo completo da skill `tekt-materializacao-segregada`, com
segregação por papel, ordem, capacidades e hashes, mas sem alegação de
isolamento ambiental forte no checkout compartilhado.

| Papel | Executor | Limite material |
|---|---|---|
| Fase A / harness | `/root/p1288_harness` | matriz, runner, testes e receipt do harness |
| Núcleo + L0 / adversário | `/root/p1288_l0` | L0 pré-gate e campanha de mutação; não implementou o candidato |
| contrato + oráculos | `/root/p1288_contract2` | contrato/oráculos congelados antes do candidato |
| implementador + relatório | `/root` | código produtivo, integração, gates e este relatório |
| verificador final | independente, posterior a este relatório | somente `p1288-verification-receipt.md` |

O humano confirmou explicitamente o gate ADR-0127 antes da criação dos
consumers produtivos e da implementação. O checkout permaneceu não commitado e
recebeu trabalho concorrente P1285/P1286/P1289/P1290/P1291, portanto não é
possível atribuir todo o diff global ao P1288. Nenhuma alteração alheia foi
descartada.

## RED, implementação e GREEN

O contrato independente do candidato foi congelado em
`lab/parity/matrix/test_p1288_candidate_contract.py`. Antes da implementação,
10 testes produziram 7 falhas e 3 sucessos; o controle anti-stub rejeitou 43
dos 46 observáveis funcionais. Depois da implementação, os mesmos 10 testes
passam.

A materialização adicionou:

- owner puro `entities::compiler_features` com `Html` e `A11yExtras`, ambos
  false por default, e re-export de compatibilidade no módulo HTML;
- composição CLI repetida, separada e por vírgula, com transporte do set por
  wiring, pipeline e eval; `bundle` é aceito sem ativar feature, conforme o
  caso opaco congelado, embora continue fora do objetivo de implementação;
- filtragem uniforme do scope: o trio PDF fica ausente sem `A11yExtras` e é
  instalado integralmente quando ativo;
- casts, defaults e diagnósticos medidos para summary/header/data; conteúdo cru
  é normalizado como célula default e célula existente preserva os demais
  campos;
- `TableElem.summary` e classificação fechada `Auto`, `Header` e `Data`, com
  level/scope/spans preservados; `table.header(level:)` foi corrigido no owner
  vigente `compiler/stdlib/structural/table_grid.rs`, após atualizar seu L0;
- carriers semânticos fechados no layout e emissão PDF de Table, THead, TBody,
  TR, TH, TD, IDs, Headers, spans, MCIDs, ParentTree e IDTree; multipágina
  conserva uma árvore lógica e índices físicos por página;
- `/Summary` somente quando presente, `/MarkInfo` com `Marked true` e
  `Suspects false`, e ausência de estrutura quando `PdfTags::Disabled`;
- export HTML de tabela independente dos metadados PDF e fallback
  `SOURCE_DATE_EPOCH` para tornar a comparação geométrica reproduzível.

Não se alega PDF/UA, tecnologia assistiva real, reflow, certificação nem
acessibilidade geral. Esses quatro observáveis permanecem `Unknown` deliberado.

## Harness final por perfil

O adendo da Fase A começou com RED de 41 testes, 2 falhas e 2 erros. A correção
deixou de injetar `--features` em comandos que não declaram features e atualizou
a expectativa pós-implementação do caso P1288-B-001. GREEN: 41/41.

| Perfil | Resultado final, total 20 | Ordem | SHA-256 do JSON |
|---|---|---|---|
| default | MATCH 15; DIFFERENCE 2; DISABLED 3; demais 0 | idêntica | `b7fc4fa6bc47b07781f0192260cd0a36ce4f63c8c44432712a3959cfe95e0221` |
| html | MATCH 17; DIFFERENCE 2; DISABLED 1; demais 0 | idêntica | `1db96db4077171caf46a5b63acb8de09363647880db7a885076eddd8228250d8` |
| a11y-extras | MATCH 16; DIFFERENCE 2; DISABLED 2; demais 0 | idêntica | `7a8a41b6b5b93c5f8abaae2e24edc0093bb718f0fb84be1ac536f91d965d82c4` |

`DISABLED_BY_PROFILE` nunca foi contado como MATCH. P1288-B-001 fica disabled
em default/html e MATCH somente no perfil ativo. As duas diferenças residuais
por perfil já pertencem à matriz global e não foram renomeadas.

## Contrato, oráculos e ataques

No candidato final, `p1288_oracles.py --order both` produziu 46 `Preserved`, 0
`Violated` e 4 `Unknown` deliberados, com ordem idêntica. O JSON final tem
SHA-256
`9702384222e5acd89d5365ec03ba562b64c170b01e85c3dd5c8a540e49bd426c`.
Os 10 testes do runner de oráculos e seus 10 self-tests passaram.

A campanha discriminatória pré-candidato permanece válida como evidência de
qualidade dos oráculos: 22/22 mutantes válidos mortos, 2 inválidos excluídos, 4
opacos e `mutation_score = 1.0`, com ordem idêntica. Seu log protegido conserva
SHA-256
`049a8fbc6b924eec6c1c07fbbd7c930fc89b27cc2a5223a33dd8891891078aae`.

Ao repetir a campanha no estado pós-candidato, os mutantes continuam 22/22
mortos e as entradas não mudam durante a execução, mas o controle não mutado é
`Violated` por A22. O output
`/tmp/p1288-adversarial-post-candidate-final.json` tem SHA-256
`6727bb1495d120fdda4d064171b7f60dbf296b6eccf6e7a020a1d5c74b2bc6a0`
e o processo termina com exit 1. Esse resultado bloqueia o selo funcional.

## Auditoria do selo L0

O manifesto pré-candidato mantém SHA-256
`0e7282b6b5b445d0532bb5456abfbd765e7878fc44d318f96a0b8b960351ed3a`
e o selo
`22042fef8243a7bf9ce0f8e4ce1b5300dbc0bd1a13da97e97059bf59983b397e`.
Núcleo, manifesto, contrato, baseline vanilla, fixtures, runner de oráculos,
testes de oráculos e baseline de oráculos continuam byte-idênticos aos pins.

A comparação dos 15 Prompts L0 protegidos encontrou:

- drift somente na linha `Hash do Código` em 8:
  `entities/compiler_features.md`, `entities/html.md`, `wiring.md`,
  `entities/elements/table.md`, `entities/elements/table_cell.md`,
  `compiler/eval/table.md`, `compiler/layout/table.md` e
  `compiler/layout/table_cell.md`;
- drift adicional em 7: `shell/cli.md`, `infra/pipeline.md`,
  `compiler/eval.md`, `compiler/stdlib/pdf.md`, `entities/layout_types.md`,
  `infra/export/stream.md` e `infra/export/builder.md`.

Para a primeira categoria, substituir a linha corrente pelo valor pré-gate
reconstrói exatamente o SHA-256 pinado. Na segunda, isso não reconstrói o pin;
o diff contém, entre outras, cláusulas concorrentes P1285/P1286. Mesmo o drift
explicável de hash é drift de bytes sob a formulação rígida do selo. Por isso
este relatório não faz uma exceção retroativa.

Ownership e estrutura corrente permanecem válidos: `crystalline-lint` e o gate
V5/V15/V26 fecharam com zero violações.

## Gates finais reproduzidos

Estado da medição: `2026-08-31T13:32:38-03:00`, HEAD
`53d21c5a602f4045a769a0ab0c935baa5ecd3b88`, working tree não commitada.
`git diff HEAD --stat` registrou o estado compartilhado exato: 155 ficheiros,
650686 inserções e 2097 remoções; esse total inclui artefatos e mudanças de
outros passos.

| Comando | Resultado |
|---|---|
| `cargo test --workspace -q` | exit 0; 5343 + 911 + 1 + 61 + 2 + 71 + 2 + 6 + 1 passaram; 3 ignorados |
| `python3 -m unittest discover -s lab/surface-inventory -p 'test_*.py'` | 22/22 |
| `python3 -m unittest lab/parity/matrix/test_runner.py` | 41/41 |
| `python3 -m unittest lab/parity/matrix/test_p1288_candidate_contract.py` | 10/10 |
| `python3 -m unittest lab/parity/matrix/test_p1288_oracles.py` | 10/10 + self-test |
| três runs de `runner.py --profile ...` | exit 0; totais fechados; ordem idêntica |
| `cargo build --workspace --bin typst -q` | exit 0 |
| `cargo fmt --all -- --check` | exit 0 |
| `crystalline-lint --quiet .` | exit 0 |
| `crystalline-lint --checks v5,v15,v26 --fail-on warning .` | exit 0; zero violações |
| `git diff --check` | exit 0 |

Binário candidato `target/release/typst`: SHA-256
`5f2841b03cc776dfdc55b43211a8f1f90307f8ba8b5b1f0f448a6adb0398553d`.
Vanilla ratificado `/usr/local/bin/typst`: SHA-256
`7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8`.

O gate global P1287 permanece visível e terminou com exit 1: 13 `Violated` e
10 `Unknown`. Conforme o Passo 1288, isso não invalida automaticamente este
recorte, mas também não é apresentado como sucesso nem como 72/77 reproduzido.

## Condição para nova certificação

Para obter um veredito `REFINED`, é necessário repetir a cadeia causal em um
estado estável: consolidar primeiro as alterações concorrentes dos L0s, gerar
novos pins/contrato/oráculos antes do candidato, executar novamente o gate
discriminatório e então validar o mesmo candidato. Resselar apenas os hashes no
fim não é suficiente, porque eliminaria a independência temporal que A22 foi
criado para proteger.

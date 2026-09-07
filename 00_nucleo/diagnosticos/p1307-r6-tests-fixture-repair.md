# P1307-R6 — sucessor focal de duas fixtures de teste

Estado: **reparo independente proposto, aguarda selo focal e integração**.
Não é veredito do candidato. Executado sem atestação de isolamento técnico.
Autor `/root/p1307_contract`; nenhuma fonte produtiva atual, diff de candidato,
recibo com patch candidato ou binário candidato foi lido/executado nesta tarefa.

## Entradas protegidas e escopo

Predecessor `p1307-r6-tests.patch`, SHA-256
`d4232b6144b7def77498e2fd1353b5dbad3786276fb8ccd5f8c56175a08fa19b`,
permanece inalterado. Baseline documental R6 SHA-256
`8cc0eae00457d2e7d54b420024eae49344032b34b295b4eda536faeb1ec3c4a3`
registra HEAD `b303f1f15b610e09872b567027e0d806387fde8c`, working tree
P1306 e os diff/stat completos antes do candidato. Para o diagnóstico de
trace, leu-se somente `git show b303f1f15b610e09872b567027e0d806387fde8c:...`
dos consumers baseline de tests, call_dispatch, eval/mod e Source; não o
working tree atual. Esses trechos são anteriores ao candidato.

A skill de materialização segregada e seus dois documentos de papéis/gates
foram relidos. Loading congelado R6, já lido pelo autor antes do candidato,
SHA-256 `c800187d07bf3acb84eb620ba4305001588001d088dabff1941ca8fe4fb0a3db`,
mantém a obrigação JSON não finito→null e -0.0 preservado. O L0 float congelado,
SHA-256 `68e877cfdaf7feaf7fee344c268244d04df13707f3aa2beeba83bf8513f49794`,
foi reconstruído em memória de `git show` do HEAD pinado mais o seu amendment
preservado em `p1307-r6-baseline.json.state.diff`; o hash reconstruído coincidiu.
Esse L0 já documentava `float.inf`/`float.nan` ausentes no cristalino e
`float("1e999")`/`float("NaN")` como carriers bilaterais. Foi erro deste autor
usar a superfície ausente como pré-condição do teste de encode.

Os hashes dos arquivos L0 atuais foram consultados apenas para checar se
ainda eram os congelados; divergiam, portanto **seus conteúdos atuais não
foram abertos**. Não se inferiu nenhuma correção a partir do produto atual.

Únicas escritas: este relatório e
`p1307-r6-tests-fixture-repair.patch`, SHA-256
`90ca08bc474f8012f87f7aa1573d2e723a7817f3cca08ac3fa5a93e21818974b`.
O sucessor deve ser aplicado sobre os testes integrados do predecessor;
nunca altera o patch predecessor, seus expected, L0 ou oracle.

## Medição literal anterior à decisão

Somente executáveis imutáveis, hashes verificados antes das execuções:

- vanilla `/usr/local/bin/typst`, SHA-256
  `7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8`,
  upstream ratificado `a51e02804`;
- baseline `/dev/shm/p1307-baseline-target.hqbv0ooo/release/typst`, SHA-256
  `945e488c53246b33fd11c56ff670c697aeaffa5a92d5c15cc8e09a76cf7cf1f3`.

Host/RAM acessado pela execução escalada autorizada. Cada comando é
`[binário, "eval", expressão_literal, "--format", "json"]`; cwd `/tmp`,
stdin ausente, timeout 20 segundos. Ambiente herdado sem chaves `TYPST_*`
ou `CRYSTALLINE_*`, com `NO_COLOR=1` e `LC_ALL=C.UTF-8`. Não se passou
`--color` ao vanilla. Foram dez processos focais, nenhuma matriz ou build.

Expressões literais:

```text
A = (repr(float.inf), repr(-float.inf), repr(float.nan), repr(-0.0))
B = (repr(float("1e999")), repr(-float("1e999")), repr(float("NaN")), repr(-0.0))
C = json.encode((none, float.inf, -float.inf, float.nan, -0.0), pretty: false)
D = json.encode((none, float("1e999"), -float("1e999"), float("NaN"), -0.0), pretty: false)
E = { let fail() = panic("fixture-control"); fail() }
```

Cada linha abaixo preserva saída literal em notação JSON; as chaves stderr
referem-se aos blocos literais seguintes quando não vazias. Horários UTC:

| Caso | Binário | Início UTC | Exit | stdout literal JSON | stderr |
|---|---|---|---:|---|---|
| A | vanilla | 2026-09-07T19:27:33.858887+00:00 | 0 | `"[\"float.inf\",\"-float.inf\",\"float.nan\",\"-0.0\"]\n"` | vazio |
| B | vanilla | 2026-09-07T19:27:33.865476+00:00 | 0 | `"[\"float.inf\",\"-float.inf\",\"float.nan\",\"-0.0\"]\n"` | vazio |
| C | vanilla | 2026-09-07T19:27:33.871977+00:00 | 0 | `"\"[null,null,null,null,-0.0]\"\n"` | vazio |
| D | vanilla | 2026-09-07T19:27:33.878095+00:00 | 0 | `"\"[null,null,null,null,-0.0]\"\n"` | vazio |
| A | baseline | 2026-09-07T19:27:33.941665+00:00 | 1 | `""` | A-baseline |
| B | baseline | 2026-09-07T19:27:34.031107+00:00 | 0 | `"[\"float.inf\",\"-float.inf\",\"float.nan\",\"-0.0\"]\n"` | vazio |
| C | baseline | 2026-09-07T19:27:34.124584+00:00 | 1 | `""` | C-baseline |
| D | baseline | 2026-09-07T19:27:34.215388+00:00 | 1 | `""` | D-baseline |
| E | vanilla | 2026-09-07T19:29:12.077285+00:00 | 1 | `""` | E-vanilla |
| E | baseline | 2026-09-07T19:29:12.120111+00:00 | 1 | `""` | E-baseline |

```json
{
  "A-baseline": "error: type float does not contain field \"inf\"\n  ┌─ <input-expression>:1:6\n  │\n1 │ (repr(float.inf), repr(-float.inf), repr(float.nan), repr(-0.0))\n  │       ^^^^^^^^^\n\n",
  "C-baseline": "error: type float does not contain field \"inf\"\n  ┌─ <input-expression>:1:19\n  │\n1 │ json.encode((none, float.inf, -float.inf, float.nan, -0.0), pretty: false)\n  │                    ^^^^^^^^^\n\n",
  "D-baseline": "error: cannot access fields on type function\n  ┌─ <input-expression>:1:0\n  │\n1 │ json.encode((none, float(\"1e999\"), -float(\"1e999\"), float(\"NaN\"), -0.0), pretty: false)\n  │ ^^^^^^^^^^^\n\n",
  "E-vanilla": "error: panicked with: fixture-control\n  ┌─ <input-expression>:1:15\n  │\n1 │ { let fail() = panic(\"fixture-control\"); fail() }\n  │                ^^^^^^^^^^^^^^^^^^^^^^^^\n\n  while calling `fail` at <input-expression>:1:41\n    fail()\n\n",
  "E-baseline": "error: panicked with: fixture-control\n  ┌─ <input-expression>:1:20\n  │\n1 │ { let fail() = panic(\"fixture-control\"); fail() }\n  │                     ^^^^^^^^^^^^^^^^^^^\n\n"
}
```

Conclusão delimitada: B prova a pré-condição bilateral; C e D têm no vanilla
a **mesma string inteira esperada**, `[null,null,null,null,-0.0]`. D no
baseline ultrapassa a falha de construção de infinito e encontra a ausência
de encode. Não há opacidade nem expected enfraquecido. E não é GREEN de trace:
o baseline CLI também não publica o trace e possui sua âncora legacy distinta.
Essa observação fica preservada, não autoriza correção de CLI/produto aqui.

## Source do harness e trace

Fonte baseline anterior à decisão:

- `01_core/src/compiler/eval/tests.rs:187–204`: MockWorld guarda `source`,
  e `new("")` cria Source markup vazia.
- Mesmo arquivo `:219–221`: `World::source` devolve exatamente esse clone.
- `01_core/src/compiler/eval/mod.rs:343–347`: eval de expressão cria
  separadamente uma Source `parse_code` com o FileId de `world.main()`.
- `01_core/src/compiler/eval/call_dispatch.rs:980–985`: trace_call tenta
  resolver call_span por `engine.world.source(id)` e retorna sem trace quando
  não há range. `:990–999` usa o mesmo World para a contenção do erro.

O helper independente `error_all` conservava uma Source correta para a
asserção final de ranges, mas fornecia ao Engine um World cuja Source era
vazia. Assim ele não fornecia a pré-condição requerida pela própria API para
observar os traces esperados. A inferência é causal e refutável: se um World
com clone idêntico de Source/code/FileId ainda não permitir observar os traces
contratados, o reparo de harness é insuficiente e a revisão volta ao autor,
sem apagar trace esperado nem adaptar produto para satisfazer fixture vazia.

O sucessor faz `let mut world` e, logo após construir a Source `parse_code`,
atribui `world.source = source.clone()`. Não basta mudar `MockWorld::new("")`
para `MockWorld::new(expression)`: esse constructor usa parsing markup.
Nenhum Source é inferido por igualdade de Value ou texto de erro; é a própria
Source integral da fixture. Conservam-se todas as expectativas de trace
`encode`, ranges f/g, mensagens, hints, cardinalidade e perfis.

## Delta, checks e gate

O patch contém somente:

1. a grafia de construção dos três não finitos dentro da expressão de teste;
2. a disponibilidade da Source de código no World do helper diagnóstico.

Não muda `[null,null,null,null,-0.0]`, nem cria float.inf/nan, nem reduz casos,
converte erro em sucesso ou altera obrigatoriedade de trace. A primeira
revisão focal tem ganho discriminatório de pré-condição: input inválido no
baseline vira input construível, chegando à ausência semântica de encoder;
a segunda remove inconsistência comprovada no adaptador de Source. Nenhuma
contagem de mutantes ou resultado candidato é reivindicada.

Checks sem acesso ao produto atual: ambos os contextos de hunk ocorreram
exatamente uma vez no fragmento do predecessor; substituição em memória e
`rustfmt --edition 2024 --emit stdout` via stdin passaram, exit 0. Nenhum
Rust foi escrito e **não** se executou git apply --check contra o working
tree candidato. Compilação e execução dos testes reparados ficam pendentes.

O verificador deve criar selo sucessor focal antes da integração. O selo
anterior não é reinterpretado como se já contivesse estas fixtures. Todos
os demais artefatos e obrigações continuam protegidos; este autor não emite
GREEN de implementação nem autoriza mudanças fora do reparo descrito.

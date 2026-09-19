# P1462 R2 — oráculos independentes pré-candidato

**Estado:** CONCLUSIVO COMO CALIBRAÇÃO DE ORÁCULOS; BASELINE RED; SEM SELO; SEM TESTE RUST; SEM CANDIDATO

**Contrato congelado R2:** `00_nucleo/diagnosticos/p1462-r2-contrato-congelado.md`

**Manifesto causal R2:** `00_nucleo/diagnosticos/p1462-r2-manifesto.json`

**Papel:** autor de oráculos e A/B tester R2, anterior ao adversário, ao selo e ao implementador

**Atestação proporcional:** executado sem atestação de isolamento

O checkout e o filesystem eram compartilhados. A independência alegada neste
relatório é somente de papel, entrada, capacidade, ordem causal e artefato. Os
hashes provam identidade e sucessão das entradas observadas; não provam
isolamento técnico. Nenhum patch candidato existia ou foi lido. O argumento
`--candidate` do runner temporário recebeu o **baseline R2**, não um candidato
futuro.

Este documento é a única escrita persistente desta autoridade. O runner e o
probe Rust ficaram exclusivamente em `/tmp`. Não foi criado
`04_wiring/tests/p1462_format_svg.rs`: pelo contrato §9, esse consumer RED só
pode nascer depois do selo discriminatório R2.

## 1. Entradas protegidas e estado medido

| Entrada | Identidade observada |
|---|---|
| baseline commit | `61796fa6e183c08207e3828cec6d2e43da1af846` |
| baseline tree | `4890fb690d222c093e858130da00d2062ae514dd` |
| manifesto R2 | `9de72872a6113b35464ae17da0f3f7dd59a70acbd5956426108e233e3bd66e8f` |
| contrato R2 | `69cfdc0de39564cf542e2ca7fc99bcb7b80752d10bc8cdce3c809c1d9cf02fa2` |
| Prompt owner do futuro teste | `c4e3d1a34b728c719a091c5f3c6ef62aa94e79bfd12b457eceb75f8092b04738` |
| vanilla ratificado | commit `586e1bd43fae6c9a973218163d3165c53ab8d16d`, binário `eb60986b522d9843172cdf318dd46c81f5922109f503ab1733cfe8baaeb1468f` |
| baseline recompilado | binário `target/debug/typst`, SHA-256 `58d503b4157a53b0d01ba080053db7bf515fbb6b30ffd3fc86493b2bb052bc5d` |
| skill do protocolo | `66990d349a9e89851686cd94590a84711c69364f76b6df501230f64daf3b0c48` |
| referência de papéis | `f59f44c4e53e89651963115c582872b4d3cd59d89689d103baa9ef8b464d2417` |
| referência de gates | `16db4af3a8a21a27e1bfc4a5dd00c976f0fc946ba46dc8df1a663171d823f72d` |

Foi feita conferência mecânica de todos os hashes de Passo, ADRs, dez Prompts
L0 e Núcleo declarados no manifesto; não houve divergência.

Às `2026-09-19T13:13:03-03:00`, imediatamente antes dos recortes, `HEAD` e
tree eram os valores acima. `git diff HEAD --stat` era vazio, porque as únicas
entradas visíveis eram não rastreadas:

```text
?? 00_nucleo/diagnosticos/p1462-r2-contrato-congelado.md
?? 00_nucleo/diagnosticos/p1462-r2-manifesto.json
?? 00_nucleo/diagnosticos/p1462-r2-mutantes.md
```

O terceiro ficheiro apareceu concorrentemente no checkout compartilhado e não
foi lido nem usado por esta autoridade. Manifesto e contrato eram as entradas
autorizadas. O mesmo HEAD/tree permanecia às
`2026-09-19T13:24:07-03:00`, depois das execuções e antes de escrever este
relatório.

## 2. Instrumento independente em `/tmp`

| Artefato efêmero | SHA-256 |
|---|---|
| `/tmp/p1462_r2_oracles.py` | `f6715f146d65134c352983a189005d0aeea096da0bcc57a52161cbd5b21104ba` |
| `/tmp/p1462-r2-carrier-probe/Cargo.toml` | `a4070b42973f98cecdb01382a76d8692f0dcf3fa3debee6de8afe1230432975e` |
| `/tmp/p1462-r2-carrier-probe/src/main.rs` | `876077cfc49cba78db6589a5df183fa675f22a13772faee6f870b8f291bc9c71` |

O runner protege os dois hashes de binário antes de qualquer classificação,
cria diretório temporário novo por caso e persiste uma linha JSONL com
`product`, `order`, `case_id`, `subcase_id`, target, SHA-256 da fonte, flags,
features, SHA-256 do binário, timeout e estado `started` antes de cada
`subprocess.run`. O terminal normal usa o evento `process_completed`; spawn
falho usa `failed` e timeout usa `timed_out`. O wrapper de compilação acrescenta
o evento `completed` com exit, canais e identidade do artefato. Cada append faz
`flush` e `fsync`.

Uma auditoria sequencial do recibo completo encontrou **324/324** pares
atribuíveis `started → process_completed`, sem `Timeout` e sem `Unknown`. A
diferença entre 324 processos e 198 eventos `completed` decorre de probes
`eval` e observadores auxiliares não publicarem artefato; não é processo sem
terminal. O recibo completo tem 894 eventos.

As correções R2 incorporadas antes da primeira execução foram:

1. `#set format.svg()` isolado espera compacto por ausência de decisão e
   `true → #set format.svg()` espera pretty; vazio nunca é modelado como
   escrita de `false`.
2. O8 é quatro casos independentes. A identidade e os inputs de cada
   subprocesso são duráveis antes do spawn, portanto um timeout seria
   atribuível somente à linha correspondente.
3. `styled-root` compila o mesmo conteúdo byte-idêntico duas vezes: decisão
   documental `true` versus override CLI `false`; somente esse par tem a
   morfologia XML normalizada comparada.
4. XML inválido, fonte cruzada, hash divergente, binário ausente e timeout
   produzem `Unknown`; um controle deliberado confirma a opacidade sem a
   promover a sucesso de produto.

O source SHA-256 do par `styled-root` foi
`2dc37e13ed04dcb25b70d77bb0f2081925872fe830209bc4e1e24661ea0885be`
nas duas execuções de cada produto.

### 2.1 Atribuição focal O8

Cada linha abaixo foi executada para vanilla e baseline; os hashes de binário
são os da §1. Compilações tiveram timeout de 90 s. `pdftotext`, `pdfinfo` e
`identify`, quando aplicáveis, tiveram timeout próprio de 30 s.

| ID | target/flags | fonte `false` | fonte `true` | observável público |
|---|---|---|---|---|
| `O8-PDF` | `pdf`; `--format pdf` | `d8609543935d1534a9d7071facd51eb1bd8295bbc7e87c478c8859c2e862b416` | `0dfc216a2b517fbde5f82a2a1df1c52f828d19342a2c25447d6c2b7cb841b045` | texto extraído e metadata selecionada |
| `O8-PNG` | `png`; `--format png` | `d8609543935d1534a9d7071facd51eb1bd8295bbc7e87c478c8859c2e862b416` | `0dfc216a2b517fbde5f82a2a1df1c52f828d19342a2c25447d6c2b7cb841b045` | dimensões e pixels |
| `O8-HTML` | `html`; `--format html --features html` | `efdebc73b7af4536ed883cfd9921418dcf6989fd57ac5e867d5ddd5416c488ef` | `61e820f7078a80a08e5e49482215875f45a1ba0e95c9785ccf07ebf3aad23916` | DOM normalizado |
| `O8-HTML-FRAME` | `html`; `--format html --features html` | `113d1d5c7d87e1bec5a4e6289d94403427a1621b5f037fc12a8cefe415045c73` | `39cd51ae5c5e8f24ab719311bb87a8c17e42e3b30dea79d6faa3ac30bf5dae11` | DOM e SVG interno normalizados |

## 3. Ordem executada, comandos, custos e exits

O baseline foi reconstruído antes dos oráculos com:

```console
cargo build -p typst-wiring --bin typst
```

Exit `0`; o Cargo reportou 2,56 s. A hora individual desse build não foi usada
como evidência decisória; a identidade decisória é o SHA-256 do binário
recompilado e foi revalidada pelo runner antes de cada corpus.

Primeiro foram executados somente `true→empty`, regra vazia isolada,
`true→false`, `false→true`, o par `styled-root` da mesma fonte e cada subcaso
O8:

```console
python3 /tmp/p1462_r2_oracles.py --mode focal --vanilla /usr/local/bin/typst --candidate /repos/Antigravity/typst-crystalline/target/debug/typst --carrier-manifest /tmp/p1462-r2-carrier-probe/Cargo.toml --ledger /tmp/p1462-r2-focal-ledger.jsonl --output /tmp/p1462-r2-focal.json
```

- intervalo do recibo: `2026-09-19T16:13:42.171860+00:00` a
  `2026-09-19T16:14:32.533533+00:00`;
- exit: `0`;
- 40 subprocessos, 124 eventos, zero timeout, zero `Unknown` acidental;
- custo medido pelos grupos: vanilla 2,224 s; baseline 48,202 s;
- recibo: `/tmp/p1462-r2-focal-ledger.jsonl`, SHA-256
  `cab3795a92cd14ad2b59f221226d918bc3649e606365779f305efd0170ae09b6`;
- relatório: `/tmp/p1462-r2-focal.json`, SHA-256
  `f65836722e8dafa8f044be4da64a84b34c2ac32d0e76a7adbcad0435ede3fef1`.

Todos os focos vanilla foram `Preserved`. No baseline, vazio/neutro e
`styled-root` foram `Violated`; os quatro O8 foram `Preserved`. Como o gate
focal ficou atribuível e sem `Unknown`, e somente então, foi executado uma vez
o corpus completo em `forward`, `reverse` e `forward-repeat`:

```console
python3 /tmp/p1462_r2_oracles.py --mode full --vanilla /usr/local/bin/typst --candidate /repos/Antigravity/typst-crystalline/target/debug/typst --carrier-manifest /tmp/p1462-r2-carrier-probe/Cargo.toml --ledger /tmp/p1462-r2-full-ledger.jsonl --output /tmp/p1462-r2-full.json
```

- intervalo do recibo: `2026-09-19T16:15:08.069270+00:00` a
  `2026-09-19T16:23:29.708631+00:00`;
- exit: `0`; veredito do instrumento: `Conclusive`;
- 324 subprocessos, 894 eventos, zero timeout, zero `Unknown` acidental;
- vanilla: forward 4,449 s; reverse 4,609 s; forward-repeat 4,482 s;
- baseline: forward 185,218 s; reverse 156,713 s; forward-repeat 146,230 s;
- os seis vetores foram estáveis entre as três ordens de cada produto;
- recibo: `/tmp/p1462-r2-full-ledger.jsonl`, SHA-256
  `25c2fb83d46784d43f512a8e56ec748e5060ba267f46156b8874cce2a8e8b9d2`;
- relatório: `/tmp/p1462-r2-full.json`, SHA-256
  `87cb24a3de68d7402c79a05fe225e149ff14c275bd0b62fbb6366517b789c8d0`.

Foram consumidas uma execução focal e uma invocação `full`, composta pelas
três execuções de ordem exigidas (`forward`, `reverse`, `forward-repeat`);
não houve segunda invocação integral nem adaptação ao baseline. O custo somado
dos grupos foi 552,127 s, dentro do teto de 120 minutos.

## 4. Vetor estável e baseline RED

Esta tabela vale igualmente para `forward`, `reverse` e `forward-repeat`:

| Caso | Vanilla ratificado | Baseline R2 | Discriminador |
|---|---|---|---|
| O1 identidade/repr/default, quatro perfis, direto/std/alias | `Preserved` | `Violated` | `(function, svg, false)` exato |
| O2 construção manual proibida | `Preserved` | `Violated` | diagnóstico `cannot be constructed manually` |
| O3 validação de set | `Preserved` | `Violated` | unknown named, inteiro, posicional e duplicado falham pelo canal normal |
| O3/O5 vazio neutro | `Preserved` | `Violated` | isolado compacto; `true→empty` continua pretty |
| O4 root + dois containers | `Preserved` | `Violated` | root aceita; `block` e `box` falham com mensagem exata e sem artefato |
| O5/O6/O7 matriz | `Preserved` | `Violated` | LWW, vazio transparente, `Some` vence, `None` delega, XML válido |
| O7 `styled-root` mesma fonte | `Preserved` | `Violated` | pretty/compact distintos, mesma morfologia XML normalizada |
| O8-PDF | `Preserved` | `Preserved` | neutralidade pública PDF |
| O8-PNG | `Preserved` | `Preserved` | neutralidade de dimensões/pixels |
| O8-HTML | `Preserved` | `Preserved` | neutralidade do DOM |
| O8-HTML-FRAME | `Preserved` | `Preserved` | neutralidade do DOM e SVG interno |
| O9 ausência de filhos irmãos | `NotApplicable` | `Preserved` | `format.png/pdf/html` ausentes no cristalino |
| O10 carriers públicos/fail-closed | `NotApplicable` | `Violated` | probe Rust externo não encontra a superfície congelada no baseline |
| controles deliberados de opacidade | `Preserved` | `Preserved` | somente os cinco controles esperam `Unknown` |

Logo, o baseline é RED de forma discriminatória e reproduzível: viola O1–O7
e O10, enquanto os controles de fronteira O8/O9 permanecem verdes. Isso evita
um falso RED causado por target alheio. O exit `0` do runner significa que a
**classificação** foi conclusiva e o vanilla foi calibrado; não significa que
o baseline satisfez o contrato.

O probe O10 importa a futura superfície pública, verifica
default/clone/igualdade, o campo `PagedDocument.format_options`, aceitação do
booleano canônico e fechamento para valor inteiro. No baseline ele falha com
as três testemunhas congeladas: símbolos `collect_document_format_options` e
`document_format_options` ausentes e inexistência do campo `format_options`.
Qualquer falha alheia a essas testemunhas teria sido `Unknown`.

## 5. Cobertura O1–O10 e limites do observador

- O1 cobre identidade nominal, `repr`, default e estabilidade de namespace
  nos quatro perfis e por acesso direto, `std` e alias.
- O2 observa a chamada manual proibida pelo diagnóstico público exato.
- O3 separa aceitação de booleanos, rejeições e neutralidade da regra vazia.
- O4 usa controle root e dois containers estruturalmente distintos (`block` e
  `box`), exige a mensagem exata e ausência de artefato no erro.
- O5 cobre as duas ordens explícitas, vazio isolado e vazio após `true` e
  `false`.
- O6 cobre todas as sete linhas da matriz, inclusive conflitos
  `Some(false)/documento true` e `Some(true)/documento false`.
- O7 parseia XML compacto e pretty e compara morfologia somente em pares da
  mesma fonte; bytes, LF, tamanho e SHA do SVG não são expected.
- O8 separa os quatro targets e usa observáveis públicos apropriados, sem
  igualdade cega de bytes para PDF/HTML.
- O9 comprova a ausência pública dos três filhos irmãos no cristalino.
- O10 é probe público externo de carriers; não inspeciona layout interno.

As contagens exatas P1357 pertencem ao diagnóstico/inventário P1357 e não
foram duplicadas como dependência frágil deste runner. Portanto O9 aqui prova
somente ausência pública dos irmãos; os deltas `BOTH +1`, `VANILLA_ONLY -1` e
`CRYSTALLINE_ONLY` invariável continuam gate separado antes do certificado
final.

## 6. Mapeamento pré-candidato M01–M18

Esta secção mapeia discriminadores já calibrados; **não** alega que mutantes
foram executados ou mortos e **não** calcula mutation score.

| Mutante | Discriminador independente preparado | Situação desta autoridade |
|---|---|---|
| M01 omitir/renomear filho | O1, quatro perfis × direto/std/alias | calibrado no vanilla; baseline rejeitado |
| M02 default `true` | O1 + linha ausência/ausência de O6 | calibrado |
| M03 permitir construção manual | O2, diagnóstico exato | calibrado |
| M04 aceitar/ignorar argumentos inválidos | O3, quatro classes e canal normal | calibrado |
| M05 perder named/chave canônica | documento `true`, CLI ausente em O6 | calibrado |
| M06 vazio escrever `false` | `true→empty` permanece pretty | corrigido R2 e calibrado |
| M07 primeira decisão vencer/inverter ordem | `true→false` e `false→true` | calibrado |
| M08 fronteira root/container errada | root + `block` + `box`, mensagem exata | calibrado |
| M09 valor interno não booleano | probe O10 fail-closed | preparado; baseline RED específico |
| M10 colapsar `None` cedo | documento `true` + CLI ausente | calibrado |
| M11 documento vencer `Some` | ambos os conflitos explícitos CLI/documento | calibrado |
| M12 vazamento de target | quatro O8 separados e pré-rotulados | calibrado sem `Unknown` |
| M13 antecipar filhos irmãos | O9 público; inventário P1357 separado | runtime calibrado; contagens pendem do gate próprio |
| M14 publicar após erro nested | O4 exige exit falho e ausência de artefato | calibrado |
| M15 precedência no exporter | ownership/invariantes 10–11 | não fingido como black-box; requer ataque estático/preseal |
| M16 L3 percorre `Content`/style key | ownership/invariante 6 | não fingido como black-box; requer ataque estático/preseal |
| M17 quebrar owner/núcleo | V15/V26 e tabela de ownership | não fingido como black-box; requer gates arquiteturais |
| M18 alterar morfologia | O7 pares da mesma fonte, inclusive `styled-root` | calibrado |

M15–M17 são deliberadamente separados do runner público: atribuir-lhes kill
por mero output correto seria crédito falso. O adversário deve fornecer ao
menos uma instância válida por M01–M18, excluir equivalentes/não compiláveis
com evidência e obter 1,0 sobre o denominador válido. Esta autoridade não leu
o catálogo adversarial concorrente e não antecipa seu veredito.

## 7. Veredito limitado

Os oráculos R2 são conclusivos para seguirem ao ataque adversarial e ao
preseal: vanilla é positivo, baseline é RED nas obrigações novas, os quatro
O8 são atribuíveis, `styled-root` usa a mesma fonte, `true→empty` implementa a
regra R2 correta, não houve `Unknown` acidental e os vetores resistiram às três
ordens.

Isto **não é selo**, **não é mutation score**, **não autoriza ainda o teste
Rust**, **não é certificado final** e não prova isolamento técnico. Selo R2
depende da execução adversarial M01–M18, score 1,0, verificador preseal distinto
e integridade das entradas protegidas. Até lá, permanecem proibidos o consumer
RED e o candidato de produção.

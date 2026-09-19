# P1462 R2 — catálogo adversarial pré-candidato de `format.svg.pretty`

**Estado:** CATÁLOGO R2 PRÉ-CANDIDATO; mutantes não aplicados; gate não
executado; nenhum selo ou mutation score emitido

**Papel:** adversário R2 `/root/p1462_mutantes`

**Regime:** protocolo completo de materialização Tekt segregada

**Atestação proporcional:** executado sem atestação de isolamento

**Manifesto causal:** `00_nucleo/diagnosticos/p1462-r2-manifesto.json`

**SHA-256 do manifesto:**
`9de72872a6113b35464ae17da0f3f7dd59a70acbd5956426108e233e3bd66e8f`

**Contrato congelado:**
`00_nucleo/diagnosticos/p1462-r2-contrato-congelado.md`

**SHA-256 do contrato:**
`69cfdc0de39564cf542e2ca7fc99bcb7b80752d10bc8cdce3c809c1d9cf02fa2`

Este catálogo foi derivado somente das obrigações públicas R2 antes de
existir ou ser lido um patch candidato. Não seleciona nem corrige a solução.
Nenhum oráculo/teste R2 foi aberto, reproduzido ou usado para adaptar as
testemunhas. A R1 permanece história inconclusiva conforme descrita pelas
entradas públicas R2; nenhum artefato, kill ou score R1 é promovido.

## 1. Proveniência, ambiente e capacidades

Medição inicial em `2026-09-19T16:09:56Z`:

- baseline/HEAD R2: `61796fa6e183c08207e3828cec6d2e43da1af846`;
- tree R2: `4890fb690d222c093e858130da00d2062ae514dd`;
- checkout compartilhado: `/repos/Antigravity/typst-crystalline`;
- estado: working tree não commitado;
- `git status --short` continha exatamente, antes desta saída,
  `?? 00_nucleo/diagnosticos/p1462-r2-contrato-congelado.md` e
  `?? 00_nucleo/diagnosticos/p1462-r2-manifesto.json`;
- `git diff HEAD --stat` não representa esses dois paths por serem untracked;
  a lista exata acima registra o dirty state;
- candidato P1462 R2: inexistente e não lido;
- oráculos/testes R2: não lidos;
- R1: não editada e não reutilizada como score/selo;
- única capacidade de escrita deste papel:
  `00_nucleo/diagnosticos/p1462-r2-mutantes.md`.

O filesystem e o contexto do checkout são compartilhados; as allowlists são
procedurais, não tecnicamente impostas. Os hashes provam identidade e ordem
causal, não isolamento. Por isso a alegação é somente “executado sem atestação
de isolamento”.

Entradas protegidas verificadas por SHA-256 bruto:

| Entrada | SHA-256 |
|---|---|
| skill `tekt-materializacao-segregada` | `66990d349a9e89851686cd94590a84711c69364f76b6df501230f64daf3b0c48` |
| `references/papeis-e-capacidades.md` | `f59f44c4e53e89651963115c582872b4d3cd59d89689d103baa9ef8b464d2417` |
| `references/artefatos-e-gates.md` | `16db4af3a8a21a27e1bfc4a5dd00c976f0fc946ba46dc8df1a663171d823f72d` |
| `00_nucleo/materialization/typst-passo-1462.md` | `289b6ead93cf3f7457c165fcd4a4cad39a52bd93c56a17e7e20ad5fcf6f816cd` |
| `00_nucleo/prompts/compiler/stdlib/format.md` | `74ab8a35c81367d351670c2cc2e01c07ba1d6758114499874527832677c7d91d` |
| `00_nucleo/prompts/compiler/eval/rules.md` | `0841c74c0d317b5510b11aa583b31f93cb1a15ea8177f5fd762f0fa3d21af3b1` |
| `00_nucleo/prompts/compiler/layout.md` | `c4f8fcf4a0d82edfff50646fe64b6ec0fd48cd5c3680d75b8adc09a6e9764de9` |
| `00_nucleo/prompts/compiler/layout/format_options.md` | `1e58b389a25615e3bf2eb7d453418fdc7011741f2898a704c17882025c1e4b23` |
| `00_nucleo/prompts/entities/document_format_options.md` | `da77427107583126e778a61de20e7b63cdf8f8b7613c11c034a37e7624ad2716` |
| `00_nucleo/prompts/entities/mod.md` | `e8d3703c5fbf2315d590cc31449f156ad06964b907a2c7a95bae9f9043993356` |
| `00_nucleo/prompts/entities/layout_types.md` | `b41932c1721c914ea57b760f16b2a95222f8e1509176b5ec53f898d94cdb51dc` |
| `00_nucleo/prompts/infra/pipeline.md` | `970a7f725b5b687acd3074e694c3d7c297ed2b1a437e884e8dd2ad27b04c0e47` |
| `00_nucleo/prompts/wiring.md` | `a7dd4c0ea9c0017bcf69e65aa27abe6897a4b737ecd64c41ec4306194d48ad77` |
| `00_nucleo/prompts/wiring/tests/p1462-format-svg.md` | `c4e3d1a34b728c719a091c5f3c6ef62aa94e79bfd12b457eceb75f8092b04738` |
| Núcleo `format-option-precedence.toml` (raw) | `ac1500ecb978576471139f7525c6b824b273fa2fc3a02ff3ca304c6880598295` |

O pin efetivo V26 do Núcleo é
`51f5e11108498faec8be851c97ce6ddc642cabf580eaada021b64369dd04cb9f`.

## 2. Hipótese de correção R2 e três controles obrigatórios

R2 corrige três pontos antes de qualquer candidato:

1. **M06/empty:** `#set format.svg()` é neutro e não escreve chave. O mutante
   válido agora fabrica `format.svg.pretty = false`; a testemunha causal é
   `pretty: true → regra vazia`, que deve continuar pretty. O M06 histórico
   inverso é inválido para R2 e não entra no denominador.
2. **O8 atribuível:** cada recorte `O8-PDF`, `O8-PNG`, `O8-HTML` e
   `O8-HTML-FRAME` persiste identidade e estado `started` antes de iniciar o
   subprocesso. Timeout afeta somente o id registrado, mas impede O8 agregado
   de ser `Preserved`.
3. **`styled-root` mesma fonte:** compacto e pretty são produzidos de bytes
   Typst idênticos. A comparação morfológica ocorre dentro desse par; nunca se
   compara `styled-root` com página vazia ou outra fonte.

Esses controles não são detalhes de harness descartáveis: ausência de id
pré-spawn, XML inválido ou comparação cross-source produz `Unknown` e não pode
matar M12/M18.

## 3. Validade e estados das instâncias

Cada M01–M18 abaixo é um operador de primeira ordem a aplicar posteriormente
sobre uma cópia limpa do candidato R2 selado. A receita é construtivamente
tipável com as interfaces congeladas, mas compilação só pode ser alegada após
existir candidato. Uma instância entra no conjunto válido `V` somente se:

1. o patch mutante parte do hash exato do candidato selado;
2. aplica um desvio e não altera L0, contrato, manifesto ou oráculos;
3. compila/liga no recorte necessário e alcança a testemunha;
4. é não equivalente frente a obrigação R2 explícita;
5. preserva os controles de fronteira não afetados.

Estados futuros:

- `Killed`: `Violated` com witness e `reason_code` específicos;
- `Survived`: instância válida não rejeitada pelas testemunhas relevantes;
- `Unknown`: timeout, crash, identidade/hash ambíguo, subcaso não persistido,
  parser opaco, XML inválido, comparação cross-source ou budget esgotado;
- `Excluded-Equivalent`: não altera obrigação observável R2;
- `Excluded-NonCompilable`: não compila/liga/alcança a witness por defeito do
  próprio mutante;
- `Pending`: operador ainda não aplicado; não é resultado e não entra no
  score.

`Unknown` nunca é `Killed` ou `Preserved`, nunca aumenta score e nunca
autoriza selo. Instância equivalente ou não compilável é evidenciada e
excluída do numerador e denominador, mas não satisfaz a classe: deve ser
substituída, dentro do budget, por instância válida da mesma Mxx.

## 4. Catálogo discriminatório R2 — 18/18

Todas as linhas têm estado atual `Pending`. A coluna de contraste explicita o
que deve ser excluído para não obter crédito por equivalência ou build
quebrado.

| ID | Receita do mutante válido/compilável a materializar depois | Obrigação e testemunha que deve matá-lo | Exclusão de contraste |
|---|---|---|---|
| M01 | No owner da factory `format`, registrar a `Func` nominal `svg` sob binding público `svx`, mantendo tipos e chamadas internas. | O1 nos quatro perfis deve encontrar `format.svg` e observar `(function, svg, false)`, inclusive via `std` e alias. | Renomear apenas identificador Rust privado é equivalente; remover símbolos/imports até quebrar o build é não compilável. |
| M02 | Na namespace pública de `format.svg`, trocar somente `pretty: Value::Bool(false)` por `true`. | O1 exige default público `false`; ausência de caller e decisão em O6 deve produzir compacto. | Mudar literal não alcançado/helper privado é equivalente; mudar o tipo público para valor que quebra callers é não compilável. |
| M03 | No constructor nativo de `format.svg`, retornar `Ok(Value::None)` em vez do erro, conservando a assinatura. | O2 exige falha e texto `cannot be constructed manually`; exit de sucesso mata o mutante. | Reconstruir o mesmo erro por outra mecânica é equivalente; retornar tipo Rust incompatível é não compilável. |
| M04 | No braço estrutural `#set format.svg`, aceitar named desconhecido descartando-o e manter o caminho de `pretty` válido intacto. | O3: `other` deve falhar com `unexpected argument: other`, não warning/aceite. Tipo, posicional e duplicado permanecem controles. | Reordenar branches sem mudar diagnósticos é equivalente; introduzir borrow/move inválido é não compilável. |
| M05 | Quando `pretty` explícito existe, transportar o booleano sob `format.svg.prettx` em vez da chave canônica. A regra vazia continua sem chave. | O3/invariante 5 e O6: documento `true` com CLI ausente deve ficar pretty; chave alternativa não é equivalente. | Renomear constante local preservando bytes canônicos é equivalente; chave de tipo incompatível é não compilável. |
| M06 | No braço eval de `#set format.svg`, quando nenhum named foi fornecido, empurrar artificialmente `format.svg.pretty = Value::Bool(false)` em `StyleDelta.custom`. | O3/O5: `pretty: true → #set format.svg()` deve preservar `true` e continuar indentado. O controle unitário também verifica que empty isolado não contém a chave. | Retornar neutralidade sem chave é a implementação correta/equivalente; o antigo M06 que omitia uma escrita `false` é inválido em R2; patch sem tipos corretos é não compilável. |
| M07 | No coletor root, após a primeira decisão explícita, ignorar decisões explícitas posteriores; empty continua neutro. | O5: `true → false` deve ficar compacto e `false → true` pretty; `true/false → empty` confirma que vazio não participa da ordem. | Mudar ordem de coleção com uma decisão só é equivalente; consumir conteúdo de forma que não compile é exclusão. |
| M08 | Remover a comparação entre ocorrências explícitas exaustivas e root, aceitando `pretty` explícito sob container; não contar `Styled` vazio como ocorrência. | O4: root explícito passa; dois containers estruturalmente distintos falham com a mensagem exata e não publicam documento/artefato. | Rejeitar um `Styled` root transparente sem chave seria outro defeito; match não exaustivo/build quebrado é não compilável. |
| M09 | Ao encontrar valor não booleano sob a chave canônica, ignorá-lo ou aplicar `false` em vez de devolver erro. | O10 exige fail-closed; estado interno impossível não pode virar ausência/default/sucesso. | Alterar apenas span mantendo a falha é equivalente; construir variante `Value` inexistente é não compilável. |
| M10 | Em L4, transformar a ausência em `Some(pretty.unwrap_or(false))` antes de chamar a entry point SVG L3. | O6: CLI `None` + documento `true` deve ser pretty; Núcleo proíbe colapso antes de consultar documento. | Colapsar depois da resolução documental é equivalente; aridade/tipo errado na API é não compilável. |
| M11 | Em L3, resolver o booleano final somente do documento e ignorar `Some(false)`/`Some(true)` do caller. | Os dois conflitos explícitos O6: `Some(false)+doc true` fica compacto; `Some(true)+doc false` fica pretty. | `match` equivalente a `override.unwrap_or(doc)` não é mutante; devolver tri-state ao exporter é não compilável. |
| M12 | No caminho PDF após existir `PagedDocument`, forçar `PdfSerializationMode::Pretty` quando `document.format_options.svg.pretty` for `true`, sem alterar SVG. | `O8-PDF` deve observar texto/metadata selecionada iguais entre decisão false/true. `O8-PNG`, `O8-HTML` e `O8-HTML-FRAME` rodam como controles atribuíveis. Só há kill se `O8-PDF` foi persistido antes do spawn e concluiu. | Vazamento apenas no SVG é equivalente para esta classe; timeout/execução sem id é `Unknown`; referência antes do documento que não compila é exclusão. |
| M13 | Clonar/reutilizar o valor público `svg` na factory e ligá-lo também como `format.png`. | O9 e inventário P1357: `format.png` continua ausente e o único delta é `BOTH +1`, `VANILLA_ONLY -1`, `CRYSTALLINE_ONLY` invariável. | Variável privada chamada `png` sem binding é equivalente; mover valor sem clone e quebrar ownership/build é não compilável. |
| M14 | Na associação das opções, capturar erro nested do coletor com fallback default e continuar até devolver/exportar o `PagedDocument`. | O4/invariante 8: regra explícita nested falha, com ausência real do SVG e de documento parcial. | Adicionar contexto e continuar devolvendo o erro é equivalente; branches de retorno incompatíveis são não compiláveis. |
| M15 | Mutação coordenada: adicionar no exporter SVG uma entrada que receba `Option<bool>` e o booleano documental, resolva ali a precedência e fazer a pipeline repassar ambos. | Ownership e invariantes 10–11: L3 pipeline resolve; exporter recebe somente o booleano final. Gate estático mata mesmo se bytes coincidirem. | Extrair helper privado dentro de `pipeline.rs` é equivalente arquitetural; assinatura não atualizada em todos callers é não compilável. |
| M16 | Substituir a chamada ao coletor L1 por helper em `infra/pipeline.rs` que percorra `Content` e reconheça literalmente `format.svg.pretty`. | Ownership/invariante 6: L3 não percorre `Content`, não interpreta style key e não duplica mensagem. Inspeção estática/linhagem deve rejeitar. | Renomear apenas o import L1 é equivalente; cópia incompleta que não compila é exclusão. |
| M17 | Num consumer produtivo compilável, trocar somente o header `@prompt` pelo path do Núcleo `format-option-precedence.toml`. | Invariante 13, ownership e V26: código nunca aponta para Núcleo; `crystalline-lint .` deve rejeitar. | Comentário fora do header reconhecido é equivalente; corromper sintaxe Rust é não compilável, não kill. |
| M18 | Após gerar SVG XML válido, inserir apenas no ramo pretty, antes de `</svg>`, `<g data-p1462-r2-mutant="1"/>`; compacto da mesma fonte fica intacto. | O7 compara compacto/pretty da mesma fonte e deve detectar morfologia normalizada diferente. Repetir também no par `styled-root` byte-idêntico. | Mudar só whitespace/LF/tamanho/SHA mantendo morfologia é equivalente; XML inválido ou comparação `styled-root`↔outra fonte é `Unknown`, não kill. |

M15–M17 são não equivalentes porque as obrigações arquiteturais e de linhagem
são parte explícita do contrato, embora o produto possa emitir os mesmos bytes.
M06 é deliberadamente a inversão R2: inserir `false` onde a ausência deveria
ser neutra. M08 considera somente decisões explícitas; envelope `Styled` sem a
chave não é ocorrência. M18 mantém XML válido para impedir falso kill por
parser e usa sempre pares da mesma fonte.

## 5. Denominador, cobertura e score

Seja `V` o conjunto de **instâncias** que compilam, alcançam a testemunha, são
não equivalentes e partem das entradas protegidas. Seja `K ⊆ V` o conjunto
classificado `Killed` por `Violated` com witness atribuível:

```text
mutation_score = |K| / |V|
```

O único score selável é `1.0`. Há ainda gate separado de cobertura: cada classe
M01–M18 precisa de ao menos uma instância válida. Com o conjunto mínimo deste
catálogo, o alvo é `|V| = 18`, `|K| = 18`, score `18/18 = 1.0` e cobertura
`18/18`. Instâncias válidas adicionais entram todas no denominador; não se
seleciona retrospectivamente apenas uma instância morta por classe.

`Pending`, `Excluded-Equivalent` e `Excluded-NonCompilable` ficam fora do
numerador/denominador e não satisfazem cobertura. `Unknown` nunca entra no
numerador nem mata; uma classe só `Unknown` permanece descoberta. `Survived`
entra em `V`, não em `K`, reduz o score e bloqueia o selo.

M12 possui uma instância mínima de vazamento PDF, mas executa os quatro cortes
O8: o kill é atribuído somente a `O8-PDF`; os outros três são controles. Se
forem criadas depois instâncias M12-PNG/HTML/HTML-FRAME, cada instância válida
aumenta `|V|` e precisa morrer individualmente. M06 histórico R1 é equivalente
à semântica R2 correta e nunca entra em `V`.

## 6. Protocolo atribuível de O8

Antes de cada subprocesso O8, o runner persiste em recibo durável:

- execution/mutant id;
- um dos ids exatos `O8-PDF`, `O8-PNG`, `O8-HTML`, `O8-HTML-FRAME`;
- target, hash da fonte, flags e argumentos;
- hash do binário, timeout e diretório temporário;
- estado `started` e timestamp.

Depois do processo, o mesmo registro recebe `completed`, `failed` ou
`timed_out`, exit, stdout/stderr, hash/presença do artefato e resultado do
comparador público. Cada subcaso usa fonte false/true controlada, diretório e
timeout próprios. A classificação é por id, não por “último processo”.

Timeout após persistência torna somente aquele id `Unknown`; sem persistência
pré-spawn, a execução inteira do recorte é opaca. O8 agregado não é
`Preserved` enquanto qualquer uma das quatro linhas estiver `Unknown`. Um
resultado R1, inclusive PDF histórico, não fornece crédito R2.

## 7. Ordem, repetição e isolamento de mutantes

Para cada instância futura:

1. conferir hashes de baseline, manifesto, contrato, L0, Núcleo, candidato e
   instrumentos R2;
2. partir de cópia limpa do candidato selado; nunca empilhar mutantes;
3. aplicar um operador, registrar diff/hash e compilar antes de testar;
4. rodar a witness focal e controles de fronteira primeiro;
5. para M06, executar empty isolado, `true → empty` e `false → empty`;
6. para M08, executar root e os dois containers, sem contar empty;
7. para M12, executar os quatro ids O8 com persistência pré-spawn;
8. para M18, verificar XML válido e pares byte-idênticos por fonte, inclusive
   `styled-root`;
9. somente quando todos os recortes forem atribuíveis e conclusivos, executar
   o corpus completo em `forward`, `reverse` e `forward-repeat`;
10. remover o mutante e reconferir o hash do candidato antes do próximo.

O vetor de classificação por case id deve ser idêntico nas três ordens;
timestamps, duração e paths temporários podem diferir. Cada caso usa diretório
novo e verifica exit, canais, presença/ausência de artefato, validade XML e o
observável próprio. Dependência de ordem, estado residual ou flake é `Unknown`;
nunca se escolhe o run conveniente.

A inversão semântica `true → false`/`false → true` de M07 não substitui a
reordenação do corpus. Igualmente, o par pretty/compacto de M18 não substitui
`forward-repeat`.

## 8. Budget e rendimento observável

O budget congelado é:

- `120` minutos de wall clock;
- até `4` rodadas de revisão;
- até `3` execuções focais por revisão;
- até `3` execuções completas antes do selo;
- focais afetados e controles antes de qualquer corpus completo.

A primeira rodada tenta uma instância por M01–M18. Substituir receita
inaplicável/não compilável consome revisão e execução focal; não permite
retirar a classe. Cada revisão registra negativos `Violated`, positivos
`Preserved`, opacos `Unknown`, casos corrigidos/regredidos, causa pública,
tempo e processos.

Duas revisões consecutivas com o mesmo vetor e mesma causa obrigam reabrir o
desenho antes de outra execução completa. Também se para se corrigir opacos
regredir positivos sem avanço monotônico, se o custo marginal exceder o
budget sem nova witness/hipótese, se informação pública for insuficiente ou se
o score já for `1.0` e revisões apenas deslocarem erros entre controles. Duas
tentativas malsucedidas na mesma classe acionam revisão arquitetural antes de
uma terceira.

Budget esgotado é diagnóstico inconclusivo: não afrouxa expected, não converte
`Unknown`, não remove sobrevivente do denominador e não autoriza selo.

## 9. Matriz futura de kills

| ID | Classe R2 | Estado atual | Expected |
|---|---|---|---|
| M01 | identidade/registro | `Pending` | `Killed` por O1 |
| M02 | default público | `Pending` | `Killed` por O1/O6 |
| M03 | construção manual | `Pending` | `Killed` por O2 |
| M04 | argumentos inválidos | `Pending` | `Killed` por O3 |
| M05 | transporte de named/chave | `Pending` | `Killed` por O3/O6 |
| M06 | empty fabrica `false` | `Pending` | `Killed` por `true → empty` em O3/O5 |
| M07 | ordem das decisões explícitas | `Pending` | `Killed` por O5 |
| M08 | root/container explícito | `Pending` | `Killed` por O4 |
| M09 | valor interno não booleano | `Pending` | `Killed` por O10 |
| M10 | colapso antecipado de `None` | `Pending` | `Killed` por O6/Núcleo |
| M11 | documento vence caller | `Pending` | `Killed` por conflitos O6 |
| M12 | vazamento target PDF | `Pending` | `Killed` por `O8-PDF` atribuível |
| M13 | filho antecipado | `Pending` | `Killed` por O9/P1357 |
| M14 | artefato após erro | `Pending` | `Killed` por O4/invariante 8 |
| M15 | precedência no exporter | `Pending` | `Killed` por ownership/invariantes 10–11 |
| M16 | traversal/key em L3 | `Pending` | `Killed` por ownership/invariante 6 |
| M17 | código aponta para Núcleo | `Pending` | `Killed` por V26/ownership |
| M18 | morfologia difere entre formas | `Pending` | `Killed` por O7, mesma fonte |

Não existe kill matrix observada nesta fase. `Pending` não é `Unknown`, kill ou
sobrevivência. Nenhum mutante/candidato/binário foi aplicado, compilado ou
executado. O recibo futuro registra comando, hashes de candidato/patch/binário,
case/subcase id, ordem, custo, vetor, witness, `reason_code` e classificação.

## 10. Limite da alegação

Mesmo um futuro `18/18 = 1.0` prova somente o poder discriminatório do
contrato R2 para identidade e set rule de `format.svg`, neutralidade de empty,
extração de `pretty`, precedência no export SVG completo, neutralidade dos
targets listados e ownership Tekt. Não prova paridade global de `format.*`,
SVG, layout, exporters, Typst ou isolamento técnico.

## 11. Recibo de validação de forma

Validação executada em `2026-09-19T16:11:54Z`, ainda no HEAD
`61796fa6e183c08207e3828cec6d2e43da1af846` e tree
`4890fb690d222c093e858130da00d2062ae514dd`:

- IDs Mxx distintos: `18`;
- linhas M01–M18 no catálogo: `18`;
- linhas M01–M18 na matriz futura: `18`;
- M06 que insere `false`, witness `true → empty`, quatro ids O8, controle
  `styled-root` da mesma fonte, política de `Unknown`, score `1.0`, atestação e
  declaração de não leitura dos oráculos R2: presentes;
- `git diff --check`: exit `0`, sem output;
- `git diff --no-index --check /dev/null
  00_nucleo/diagnosticos/p1462-r2-mutantes.md`: nenhum diagnóstico de
  whitespace; exit `1` esperado porque o arquivo novo difere de `/dev/null`;
- `git status --short`: somente manifesto R2, contrato R2 e este catálogo
  estavam sujos, todos untracked; este papel criou somente o catálogo.

Não foram executados build, teste, candidato, mutante, exporter ou binário do
produto. O recibo atesta apenas forma e whitespace do catálogo pré-candidato.

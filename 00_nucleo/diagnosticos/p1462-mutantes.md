# P1462 — catálogo adversarial pré-implementação de `format.svg.pretty`

**Estado:** CATÁLOGO PRÉ-CANDIDATO; mutantes não aplicados; gate
discriminatório não executado; nenhum selo emitido

**Papel:** adversário `/root/p1462_mutantes`

**Regime:** protocolo completo de materialização Tekt segregada

**Atestação proporcional:** executado sem atestação de isolamento

**Manifesto causal:** `00_nucleo/diagnosticos/p1462-manifesto.json`

**SHA-256 do manifesto:**
`de5d15c28576da4436fdaf05ee1c92416e001577ca07d4d5761f7a6bfb7885b4`

**Contrato congelado:**
`00_nucleo/diagnosticos/p1462-contrato-congelado.md`

**SHA-256 do contrato congelado:**
`96052223adf9ed4ff49332ee72ee2c4096f1a9e04cb653077f4499b42fbd8503`

Este artefato foi escrito antes de existir ou ser lido um patch candidato. Ele
não escolhe nem corrige a implementação. Também não lê nem reproduz a redação
de testes/oráculos independentes: as testemunhas abaixo derivam apenas do
contrato público congelado e dos owners L0. O checkout e o filesystem são
compartilhados, as allowlists são procedurais e hashes provam identidade e
ordem, não isolamento técnico.

## 1. Proveniência, entradas e capacidades

Medição de entrada em `2026-09-19T15:46:51Z`:

- HEAD: `514b2ae8f35b661127e38620dcbdaefa9e158624`;
- tree de HEAD: `c12c3dac7ba85e0bf9f89b21c5e88203ff84b48e`;
- checkout: `/repos/Antigravity/typst-crystalline`, compartilhado;
- estado: working tree não commitado;
- `git status --short` antes desta saída continha exatamente
  `?? 00_nucleo/diagnosticos/p1462-contrato-congelado.md` e
  `?? 00_nucleo/diagnosticos/p1462-manifesto.json`;
- `git diff HEAD --stat` não representava esses dois arquivos porque eram
  untracked; a lista exata acima é a proveniência do dirty state;
- baseline imutável recebido: o mesmo HEAD/tree acima;
- candidato P1462: inexistente e não lido;
- teste/oráculo independente P1462: não lido;
- única escrita autorizada e realizada por este papel:
  `00_nucleo/diagnosticos/p1462-mutantes.md`.

Entradas causais verificadas por SHA-256 bruto:

| Entrada | SHA-256 |
|---|---|
| `00_nucleo/materialization/typst-passo-1462.md` | `1e473512724681c168ffe514e37c1b7eea85a617ce4a2ea194c4fed8a56c8a65` |
| `00_nucleo/prompts/compiler/stdlib/format.md` | `6bf2a41d8852644bc3a3960efad435777f9edfe525a3437aa7c82401fd65b797` |
| `00_nucleo/prompts/compiler/eval/rules.md` | `448431c67653248b8b35320a47252be3d3e409aec3a3c8bbc73214e13f2c4189` |
| `00_nucleo/prompts/compiler/layout.md` | `3002834d82569ed92048da5928afeaef0c8407bc9c8a8840d0525924b6548cb7` |
| `00_nucleo/prompts/compiler/layout/format_options.md` | `2aff71827ad90bf5c9f5d96067d1cf9761ef34ca58db6a12432114f7b41b269b` |
| `00_nucleo/prompts/entities/document_format_options.md` | `df6dd629b780e61150e1f2475ef84495fc307564d011806e1fe9dd1b9cea5586` |
| `00_nucleo/prompts/entities/mod.md` | `0a9c2b56c55ccdd251e70b981ecb2155c6d20e91345a59398812347a27ddbedd` |
| `00_nucleo/prompts/entities/layout_types.md` | `ed9d3c3b72314a70f881b6125cd6df5c907bc725eb424c93a62a653138e18995` |
| `00_nucleo/prompts/infra/pipeline.md` | `c7a32dc056e8f19967edc21d91e71b61415e3419478b7bd8af26fcc427e10937` |
| `00_nucleo/prompts/wiring.md` | `8cb5efb6713cadb2d53f526c8fd952d1a84de7016bc1952bb4ed9b22cf3e9dc8` |
| `00_nucleo/prompts/wiring/tests/p1462-format-svg.md` | `c37dc1f71f8e383de33abb667c7c36656e652f1f149d584d51e1b597b89a8e57` |
| `00_nucleo/prompts/_nuclei/export/format-option-precedence.toml` (raw) | `ac1500ecb978576471139f7525c6b824b273fa2fc3a02ff3ca304c6880598295` |

O pin efetivo V26 do Núcleo recebido pelo manifesto é
`51f5e11108498faec8be851c97ce6ddc642cabf580eaada021b64369dd04cb9f`.
Também foram lidos somente os trechos produtivos do baseline necessários para
garantir que as formas de mutação propostas têm pontos de aplicação reais:
factory `format`, avaliação de set rules, `Content`/`Styles`, `PagedDocument`,
pipeline SVG e dispatch L4. Nenhum desses arquivos foi alterado ou executado.

## 2. Validade, equivalência e estados do gate

Cada linha M01–M18 abaixo define um operador de primeira ordem e um ponto de
aplicação posterior sobre o candidato já materializado e selado. A receita é
construtivamente tipável com as APIs congeladas, mas, por não existir candidato
nesta fase, **não há alegação prematura de compilação observada**. Na
calibração, uma instância só entra no conjunto válido `V` depois de:

1. partir novamente do candidato selado limpo, sem acumular outro mutante;
2. aplicar exatamente o desvio sem alterar contrato, L0 ou oráculos;
3. compilar o workspace/recorte suficiente e alcançar a testemunha indicada;
4. demonstrar violação não equivalente de pelo menos uma obrigação congelada;
5. conservar os controles de fronteira não afetados.

Os estados canônicos de uma instância aplicada são:

- `Killed`: resultado `Violated`, com testemunha pública específica;
- `Survived`: compila e executa, mas nenhuma testemunha relevante o rejeita;
- `Unknown`: timeout, crash, interrupção, baseline/hash divergente, parser
  opaco, XML inválido antes da observação ou outra identidade ambígua;
- `Excluded-Equivalent`: o patch não muda obrigação observável do contrato;
- `Excluded-NonCompilable`: o patch não compila, não liga ou não alcança a
  testemunha por defeito de construção do mutante;
- `Pending`: operador catalogado mas ainda não aplicado; não é resultado e não
  entra em score.

`Unknown` nunca é `Killed`, nunca é `Preserved`, nunca aumenta score e nunca
autoriza selo. Uma receita que não casar com a forma final do candidato é
`Excluded-NonCompilable/Unapplicable`; o adversário deve construir uma nova
instância da mesma classe, dentro do budget, em vez de diminuir a cobertura ou
contar a exclusão como kill.

## 3. Catálogo das 18 classes congeladas

Todas as linhas têm estado atual `Pending`. “Exclusão de contraste” registra
uma alteração parecida que não seria um mutante válido e impede inflar o
denominador com equivalentes ou cobrar crédito por falha de compilação.

| ID | Mutante válido e forma de aplicação posterior | Obrigação/testemunha que deve matá-lo | Exclusão de contraste |
|---|---|---|---|
| M01 | No owner `compiler/stdlib/format.rs`, conservar a `Func` nominal `svg`, mas registrar o binding do scope sob `svx` em vez de `svg`. É uma troca de literal e permanece compilável. | O1 nos quatro perfis: `format.svg` deve existir, ter tipo `function`, repr `svg` e identidade por alias/`std`. | Renomear só uma variável Rust privada é equivalente; remover a factory ou import exigido até quebrar build é não compilável. |
| M02 | Na construção do namespace público de `format.svg`, trocar somente o valor exposto de `pretty` de `Value::Bool(false)` para `Value::Bool(true)`. | O1 exige `(function, svg, false)`; O6 também conserva o default final `false` quando caller e documento não escolhem outro valor. | Mudar um `false` não alcançado em teste/helper privado é equivalente; trocar tipo do campo por inteiro e quebrar os callers é não compilável. |
| M03 | No native constructor de `format.svg`, substituir o erro medido por `Ok(Value::None)`, mantendo assinatura e tipos. | O2: a chamada deve falhar com `cannot be constructed manually` e nunca ter exit de sucesso. | Alterar apenas a construção interna do mesmo `SourceDiagnostic` preservando texto/erro é equivalente; retornar tipo Rust incompatível é não compilável. |
| M04 | No braço estrutural exato de `#set format.svg`, transformar named desconhecido em `continue`/descarte silencioso, mantendo `pretty` válido inalterado. | O3: `other` deve falhar com `unexpected argument: other`, pelo canal de erro e nunca como warning/aceite. Os controles de tipo, posicional e duplicado continuam a delimitar a fronteira. | Reordenar branches sem mudar nenhum diagnóstico é equivalente; remover a avaliação de modo a introduzir move/borrow inválido é não compilável. |
| M05 | Na produção de `StyleDelta.custom`, trocar a chave canônica por `format.svg.prettx`, sem mudar o `Value::Bool`. | O3/invariante 4 e O6: documento `pretty: true` com CLI ausente deve chegar indentado; a chave não canônica não pode ser tratada como equivalente. | Renomear uma constante local mantendo bytes `format.svg.pretty` é equivalente; usar tipo de chave incompatível é não compilável. |
| M06 | No braço da set rule, quando `args` está vazio, retornar sem empurrar `format.svg.pretty = false`; named explícito continua funcional. | O3 exige que a set rule sem named materialize decisão explícita `false`; a terceira sequência de O5 (`true → sem named`) deve terminar compacta. | Materializar `false` por outra forma é equivalente; remover o retorno exigido pela assinatura é não compilável. |
| M07 | No coletor root, aplicar somente a primeira ocorrência canônica (por exemplo, guardando `seen` e ignorando as seguintes), em vez de last-write-wins lexical/cascaded. | O5: `true → false` deve ser compacta, `false → true` deve ser indentada e wrappers `Sequence`/`Styled` não podem inverter a ordem. | Inverter a ordem de uma coleção que contém uma única regra é equivalente ao corpus relevante; uma iteração que move o conteúdo e não compila é excluída. |
| M08 | No coletor, omitir a comparação `contagem_exaustiva > contagem_root` e devolver as opções root mesmo quando existe ocorrência nested. | O4 com root válido e dois containers estruturalmente distintos: nested deve emitir exatamente `format set rules are not allowed inside of containers`. | Trocar o tipo do contador sem mudar resultados é equivalente; remover a travessia de modo a deixar match não exaustivo/build quebrado é não compilável. |
| M09 | Ao encontrar valor não booleano sob `format.svg.pretty`, ignorá-lo ou usar `false` em vez de devolver `SourceResult::Err`. | O10 fail-closed: estado interno impossível deve falhar; não pode virar ausência/default/sucesso. | Alterar apenas o span mantendo falha observável é equivalente para esta classe; tentar construir variante `Value` inexistente é não compilável. |
| M10 | No braço SVG de L4, substituir o `Option<bool>` preservado por `Some(pretty.unwrap_or(false))` antes da nova entry point L3. | O6, linha `CLI None + documento true`: deve ser indentada. O Núcleo exige `None` preservado até consultar o documento. | Fazer o mesmo colapso depois de a opção documental já ter sido resolvida é equivalente; chamar a API nova com aridade/tipo errados é não compilável. |
| M11 | No ponto L3 de resolução, calcular o booleano final somente de `document.format_options.svg.pretty`, ignorando `Some(true)` e `Some(false)`. | Os dois conflitos explícitos de O6: `Some(false)+doc true` deve ser compacta e `Some(true)+doc false` deve ser indentada. | Reescrever `override.unwrap_or(doc)` com `match` equivalente não é mutante; devolver `Option<bool>` onde exporter requer `bool` é não compilável. |
| M12 | No caminho PDF já de posse de `PagedDocument`, quando `document.format_options.svg.pretty` for `true`, forçar `PdfSerializationMode::Pretty` antes do exporter, mantendo o resto do fluxo compilável. | O8: uma fonte que difere somente por `#set format.svg(pretty: true)` não pode alterar PDF; os controles PNG, HTML e `html.frame` permanecem também neutros. | Mudança apenas na serialização SVG é fora desta classe; referência a `format_options` antes de o documento existir, se não compilar, é exclusão e deve ser refeita no ponto posterior correto. |
| M13 | Na factory `format`, reutilizar/clonar o valor `svg` e publicá-lo também sob o binding `png` (um único filho antecipado basta). | O9: `format.png` deve continuar ausente e o inventário P1357 deve ter exatamente o delta `BOTH +1`, `VANILLA_ONLY -1`, `CRYSTALLINE_ONLY` invariável nos quatro perfis. | Criar variável privada chamada `png` sem binding público é equivalente; mover a `Func` sem clone e quebrar ownership/compilação é não compilável. |
| M14 | Na associação das opções ao documento, substituir a propagação `?` do coletor por fallback `unwrap_or_default()` e continuar até devolver/exportar o `PagedDocument`. | O4/invariante 7: nested deve falhar e não pode publicar SVG de sucesso nem documento parcial/provisório. | Capturar o erro apenas para anexar contexto e devolvê-lo continua equivalente; um branch com tipos de retorno divergentes é não compilável. |
| M15 | Mutação coordenada de L3/L3-export: adicionar uma entrada interna do exporter que receba `pretty_override: Option<bool>` e `document_pretty: bool`, resolva ali `override.unwrap_or(document_pretty)` e fazer o pipeline delegar ambos sem resolver. | Ownership congelado e invariantes 9–10: a precedência pertence a `infra/pipeline.rs`; `infra/export/svg.rs` deve receber somente o booleano final. A inspeção estática deve matar mesmo que bytes coincidam. | Extrair uma função privada ainda em `pipeline.rs` é equivalente arquitetural; mudar assinatura sem atualizar todos os callers é não compilável e não recebe kill. |
| M16 | Substituir em `infra/pipeline.rs` a chamada ao coletor L1 por helper local que percorra `Content` e reconheça literalmente `format.svg.pretty`, preservando externamente o mesmo resultado. | Ownership congelado e invariante 5: L3 não percorre `Content`, não interpreta chave de style nem duplica mensagem. Inspeção estática/linhagem deve localizar traversal/chave em L3. | Apenas renomear o import da função L1 é equivalente; copiar trecho incompleto que não compila é exclusão, não kill. |
| M17 | Em um consumer produtivo P1462 compilável, trocar somente o header `@prompt` para `00_nucleo/prompts/_nuclei/export/format-option-precedence.toml`; não mudar Rust. | O ownership, invariante 12 e gate arquitetural: `crystalline-lint .` deve rejeitar com V26; código nunca aponta diretamente para Núcleo. | Alterar comentário que não seja header reconhecido é equivalente; corromper sintaxe Rust para obter falha de build é não compilável. |
| M18 | Depois de obter SVG válido, somente quando o booleano final for pretty, inserir antes de `</svg>` o nó XML válido `<g data-p1462-mutant="1"/>`; compacto permanece intacto. | O7: ambos devem ser XML válido e sua morfologia normalizada deve ser igual. O nó extra produz divergência sem depender de tamanho, LF ou SHA. | Alterar somente whitespace/indentação pedida, ordem irrelevante normalizada, tamanho ou SHA mantendo a mesma morfologia é `Excluded-Equivalent`; produzir XML inválido é `Unknown` do instrumento, não kill sem observável. |

M15, M16 e M17 são mutantes arquiteturais: a testemunha é o gate estático
congelado, não uma diferença de bytes. Eles não são equivalentes ao contrato
porque ownership, direção da dependência e linhagem são obrigações explícitas.
M18 deliberadamente usa XML ainda válido; assim o oráculo precisa distinguir
morfologia, e não obter crédito apenas porque o parser quebrou.

## 4. Denominador, score e cobertura de classes

Seja `V` o conjunto de **instâncias** aplicadas que compilaram, alcançaram a
testemunha, são não equivalentes e preservaram a identidade das entradas. Seja
`K ⊆ V` o conjunto classificado `Killed` por `Violated` com testemunha
específica. O score é:

```text
mutation_score = |K| / |V|
```

O valor exigido para selo é exatamente `1.0`. Além do score, há um gate
separado de cobertura: cada uma das 18 classes M01–M18 precisa possuir ao menos
uma instância válida em `V`. Com o catálogo mínimo de uma instância por classe,
o resultado esperado é `|V| = 18`, `|K| = 18`, score `18/18 = 1.0` e cobertura
`18/18`. Se forem adicionadas instâncias válidas, **todas** entram no
denominador; não se escolhe retrospectivamente apenas uma instância morta por
classe.

`Excluded-Equivalent`, `Excluded-NonCompilable` e `Pending` ficam fora do
numerador e denominador, mas suas evidências permanecem no recibo. Uma exclusão
não satisfaz a cobertura da classe. `Unknown` também não entra no numerador e
não mata; uma classe cujo único resultado seja `Unknown` continua descoberta e
impede o selo. `Survived` entra em `V`, não em `K`, reduz o score e bloqueia o
selo imediatamente.

Não se agregam subcasos de M04, M08, M12 ou M17 como se um kill parcial
provasse toda a fronteira: o mutante catalogado tem um desvio único, mas a
testemunha executa seus controles positivos e negativos para excluir falso
kill. Uma futura subdivisão em mais instâncias aumenta o denominador real.

## 5. Ordem, repetição e isolamento operacional

Para cada mutante futuro:

1. verificar hashes do manifesto, contrato, L0, Núcleo, candidato selado e
   baseline antes da aplicação;
2. restaurar materialmente uma cópia limpa do candidato selado; nunca empilhar
   Mxx sobre Myy;
3. aplicar o operador e registrar diff/hash próprio;
4. compilar primeiro; falha é `Excluded-NonCompilable`, não `Killed`;
5. executar primeiro a testemunha focal e os controles de fronteira da linha;
6. somente após sucesso focal executar o corpus completo em `forward`,
   `reverse` e `forward-repeat`;
7. comparar a classificação por id de caso entre as três ordens; timestamps,
   nomes de diretório temporário e duração não precisam coincidir;
8. usar diretório temporário novo por caso, conferir exit, stdout/stderr,
   presença/ausência de artefato, XML válido e o observável específico;
9. remover a instância e verificar novamente o hash do candidato selado antes
   do próximo mutante.

O vetor por id deve ser idêntico em `forward`, `reverse` e `forward-repeat`.
Qualquer dependência de ordem, estado residual, artefato anterior ou flake é
`Unknown` até diagnóstico; nunca é escolhido o run conveniente. M07 inclui
adicionalmente as duas inversões semânticas `true → false` e `false → true`,
que não substituem a reordenação do corpus. M08 executa os dois containers
estruturalmente distintos e o controle root. M14 confere ausência real do
arquivo, não apenas exit diferente de zero.

## 6. Budget e regras de parada

O budget congelado do manifesto é integral:

- no máximo `120` minutos de wall clock;
- no máximo `4` rodadas de revisão;
- no máximo `3` execuções focais por revisão;
- no máximo `3` execuções do corpus completo antes do selo;
- casos afetados e controles de fronteira antes do corpus completo;
- corpus completo/repetição/reordenação só depois do recorte focal passar,
  antes do selo e na verificação final.

A primeira rodada tenta uma instância de cada M01–M18. Substituir um operador
não compilável ou inaplicável consome revisão e execução focal; não cria licença
para retirar a classe. Duas revisões consecutivas com o mesmo vetor de
classificação e a mesma causa pública dominante obrigam parar e reabrir o
desenho antes de uma terceira tentativa. Também se para quando correção de
opacos regride positivos (ou o inverso) sem progresso monotônico, quando o
custo marginal excede o budget sem nova testemunha/hipótese, ou quando a
classificação depende de informação ausente das interfaces públicas.

Esgotar tempo, revisões ou execuções produz diagnóstico inconclusivo. Não
afrouxa expected, não converte `Unknown`, não remove sobrevivente do
denominador e não autoriza selo. Duas tentativas malsucedidas na mesma classe
acionam revisão arquitetural conforme a regra prática da skill.

## 7. Matriz de execução futura

| ID | Classe | Estado atual | Expected do gate |
|---|---|---|---|
| M01 | identidade/registro público | `Pending` | `Killed` por O1 |
| M02 | default público | `Pending` | `Killed` por O1/O6 |
| M03 | construção manual | `Pending` | `Killed` por O2 |
| M04 | validação de argumentos | `Pending` | `Killed` por O3 |
| M05 | transporte/chave canônica | `Pending` | `Killed` por O3/O6 |
| M06 | set rule sem named | `Pending` | `Killed` por O3/O5 |
| M07 | ordem lexical/cascaded | `Pending` | `Killed` por O5 |
| M08 | fronteira root/container | `Pending` | `Killed` por O4 |
| M09 | estado interno fail-closed | `Pending` | `Killed` por O10 |
| M10 | colapso antecipado de `None` | `Pending` | `Killed` por O6/Núcleo |
| M11 | documento vence CLI | `Pending` | `Killed` por O6 |
| M12 | vazamento entre targets | `Pending` | `Killed` por O8 |
| M13 | filho futuro antecipado | `Pending` | `Killed` por O9/P1357 |
| M14 | artefato após erro nested | `Pending` | `Killed` por O4/invariante 7 |
| M15 | precedência dentro do exporter | `Pending` | `Killed` por ownership/invariantes 9–10 |
| M16 | traversal/chave em L3 | `Pending` | `Killed` por ownership/invariante 5 |
| M17 | Núcleo como `@prompt` | `Pending` | `Killed` por V26/ownership |
| M18 | divergência morfológica pretty | `Pending` | `Killed` por O7 |

Não há kill matrix observada nesta fase: `Pending` não é `Unknown`, kill nem
sobrevivência. Nenhum candidato foi mutado, compilado ou executado. O futuro
recibo deve registrar para cada instância comando, hash do patch, hash do
binário, tempo, custo, ordem, vetor por caso, testemunha, `reason_code` e
classificação; só então a matriz acima pode receber resultados.

## 8. Limite da alegação

Mesmo uma execução futura `18/18 = 1.0` prova apenas o poder discriminatório
do contrato congelado para o fragmento P1462: identidade/set rule de
`format.svg`, extração documental de `pretty`, precedência do export SVG
completo, neutralidade dos targets listados e ownership Tekt correspondente.
Não prova paridade global de `format.*`, SVG, layout, exporters, Typst nem
isolamento técnico da execução.

## 9. Recibo de validação de forma

Validação executada em `2026-09-19T15:50:12Z`, ainda no HEAD
`514b2ae8f35b661127e38620dcbdaefa9e158624` e tree
`c12c3dac7ba85e0bf9f89b21c5e88203ff84b48e`:

- IDs distintos no documento: `18`;
- linhas do catálogo M01–M18: `18`;
- linhas da matriz futura M01–M18: `18`;
- atestação, política explícita de `Unknown`, score `1.0` e declarações de não
  leitura do candidato/teste independente: presentes;
- `git diff --check`: exit `0`, sem output;
- `git diff --no-index --check /dev/null
  00_nucleo/diagnosticos/p1462-mutantes.md`: nenhum diagnóstico de whitespace;
  exit `1` esperado porque o arquivo novo difere de `/dev/null`;
- `git status --short`: os três únicos caminhos sujos eram
  `p1462-manifesto.json`, `p1462-contrato-congelado.md` e este catálogo, todos
  untracked; este papel criou somente o terceiro.

Não foram executados build, teste, candidato, mutante, exporter ou binário do
produto. Esta validação prova somente a forma do catálogo e a ausência de erro
de whitespace detectável pelo Git.

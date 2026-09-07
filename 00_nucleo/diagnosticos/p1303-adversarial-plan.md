# P1303 — plano adversarial anterior ao patch

## Identidade, regime e limite da alegação

- `step`: `P1303`
- `role`: `ADVERSARIO`
- `artifact`: `00_nucleo/diagnosticos/p1303-adversarial-plan.md`
- `baseline_commit`: `5b4a0d0438a535c54fdb5e74b28903c1313f5bc2`
- `contract_id`: `C-P1303-v1`
- `contract_sha256`: `837cb4bd1f38428d93cdadf9216c0182c69e3d8bfa8eb489ed52a710e2bbdf8a`
- `contract_size_bytes`: `17189`
- `regime`: protocolo completo de materialização segregada, **executado sem
  atestação de isolamento técnico**
- `authored_from_state_at`: `2026-09-04T00:36:34,463447802-03:00`
- `required_mutation_score`: `1.0`

Este documento foi escrito antes de existir patch candidato. Ele especifica
ataques e assassinos, mas não implementa, corrige nem escolhe a solução. O
filesystem e o contexto conversacional são compartilhados, portanto hashes
identificam entradas e ordem causal, não provam isolamento. Mesmo um futuro
score `1.0` provará apenas o poder discriminatório sobre os 14 mutantes e o
fragmento observável de C-P1303-v1; não provará equivalência funcional geral.

## Executor, capacidades e inputs congelados

O executor é uma sessão Codex no papel exclusivo de adversário. A escrita
permitida e efetivamente exercida é somente este arquivo. Não foram editados
produto, L0, testes, oracle, baseline ou contrato; não houve staging nem commit.
Não existia implementação candidata no estado de autoria.

Leituras normativas diretas:

| Input | SHA-256 | Bytes |
|---|---|---:|
| `AGENTS.md` | `bc50c0c6d54c0e301a5fe3c5c5869dbeef8fdf624096b0fa20185122c64d7da0` | 19017 |
| `/home/dikluwe/.codex/skills/tekt-materializacao-segregada/SKILL.md` | `66990d349a9e89851686cd94590a84711c69364f76b6df501230f64daf3b0c48` | 4318 |
| `references/papeis-e-capacidades.md` da skill | `f59f44c4e53e89651963115c582872b4d3cd59d89689d103baa9ef8b464d2417` | 1798 |
| `references/artefatos-e-gates.md` da skill | `16db4af3a8a21a27e1bfc4a5dd00c976f0fc946ba46dc8df1a663171d823f72d` | 4314 |
| `00_nucleo/materialization/typst-passo-1303.md` | `f1db5c02b7e6af5ef461900c213e928a8d16bbed156d8eba7ff5c6b32c584fe9` | 16143 |
| `00_nucleo/diagnosticos/p1303-baseline-status.txt` | `8e06d7deba171604986f5ae3eca4d9c2c964a2aaf6d2a0f1971f7aa27574d9a8` | 2066 |
| `00_nucleo/diagnosticos/p1303-contract.md` | `837cb4bd1f38428d93cdadf9216c0182c69e3d8bfa8eb489ed52a710e2bbdf8a` | 17189 |

Para localizar assassinos e assegurar que as transformações são aplicáveis,
foram lidos diretamente do objeto Git do baseline, e não da working tree:

| Blob no commit `5b4a0d0…` | SHA-256 | Bytes | Uso |
|---|---|---:|---|
| `01_core/src/compiler/eval/bindings/field_access.rs` | `29abd27cc01b347da9fbc12a93f05265882c043d36cd9a03cff5e6487a96e024` | 35871 | localizar ramo, span geral e projeção `std` |
| `01_core/src/compiler/eval/tests.rs` | `5437bd761f48bef79b2eedd5c2e920bcba310e41c0e85346afaf6db6a2e68af9` | 718550 | localizar assassinos P735/P1301 existentes |
| `01_core/src/entities/span.rs` | `e8bb9e3c42e79233b244e174187427e43d19cb0413901e02bcef25d6f5cd21d2` | 7833 | confirmar `Span::from_range` para os mutantes de fronteira |
| `01_core/src/compiler/stdlib/pdf.rs` | `b1d7b325eace8f6d819a16c6ff7c135f1ca42781bacaec71888045dbd977ce4d` | 17833 | localizar guard de feature e bindings sentinela |

Uma busca textual de localização no commit emitiu linhas coincidentes de
diagnósticos históricos; nenhuma conclusão ou expectativa histórica foi
incorporada. Essa exposição de contexto, somada ao workspace compartilhado, é
uma limitação de atestação explicitamente registrada.

No instante acima, `git status --short --untracked-files=all` continha somente
o passo, P0 e o contrato P1303 como arquivos não rastreados; não havia diff
tracked ou staged conforme P0. O executor P7 deve reconferir todos os hashes no
handoff. Qualquer mudança de contrato, passo, baseline ou oracle protegido
invalida este plano e reinicia a cadeia na primeira fase afetada.

## Catálogo nominal de assassinos

Estes nomes são obrigações para os testes/oracle independentes de P5/P7; não
são testes criados por este papel:

- `p1303_negativos_pdf_exatos_nos_perfis_sem_a11y`: para cada field em
  `default` e `html`, exige erro único, zero laterais, mensagem exata, dois
  hints exatos e ordenados, range exato (`14..23`, `14..25`, `14..27`) e zero
  `Unknown`.
- `p1303_positivos_pdf_exatos_nos_perfis_com_a11y`: para cada field em `a11y`
  e `html+a11y`, exige exit `0`, stderr vazio, kind `function`, nome/`repr`,
  chamada representativa e `MATCH_VALUE`.
- `p1303_sentinelas_de_span_module_e_nao_module`: cobre `std.nope`,
  `calc.nope`, `sym.nope`, `color.map.nope`, o dicionário ausente e
  `float("NaN").is-nan` sem chamada.
- `p1303_sentinelas_pdf_ungated_e_html_disabled`: cobre kind, nome/`repr`,
  valor/chamada de `pdf.attach` e `pdf.artifact`, além de
  `EXPECTED_FEATURE_DISABLED` para `html` sem a feature HTML.
- `p1303_ordem_repeticao_e_estado_completo`: compara por chave estável
  `(perfil, probe)` ordem normal, sua repetição e ordem integralmente invertida,
  incluindo todos os campos observáveis e a ausência de `Unknown`.

Os probes bilaterais correspondentes têm IDs estáveis
`P1303-N/<perfil>/<field>`, `P1303-P/<perfil>/<field>`,
`P1303-S/<sentinela>` e `P1303-META/ORDER-REPEAT`. Assassinos já presentes no
baseline e que devem continuar como defesa sobreposta são
`p1301_std_aliases_mensagem_vanilla_e_span_field_only_nos_quatro_perfis`,
`p1301_modulos_independentes_mensagem_vanilla_e_span_field_only`,
`p1301_lookups_de_modulo_existentes_preservam_valor_e_kind`,
`p1301_controle_nao_module_dicionario_preserva_span_total`,
`p735_pdf_fields_sao_funcoes`, `p735_pdf_attach_produz_carrier_invisivel` e
`p735_pdf_artifact_passthrough_do_body`.

## Os 14 mutantes mínimos obrigatórios

Cada transformação abaixo deve ser aplicada sobre o candidato isoladamente.
O hunk descrito é uma especificação de ataque, não autorização de edição neste
papel. Um mutante só conta como morto se compilar, o assassino nomeado falhar
pela testemunha prevista e o candidato restaurado voltar a passar.

### M01 — conservar o span total de `access`

- **Hipótese:** os testes verificam apenas mensagem/hints e deixam sobreviver a
  divergência original `pdf.<field>`.
- **Hunk/transformação aplicável:** no `SourceDiagnostic::error` do ramo
  feature-gated de `field_access.rs`, substituir o span candidato por
  `access.span()`.
- **Assassino nomeado:** `p1303_negativos_pdf_exatos_nos_perfis_sem_a11y` e
  probes `P1303-N/*/*`.
- **Resultado esperado:** `Violated` nos seis casos negativos; nenhum resultado
  pode ser `Unknown`.
- **Testemunha:** ranges observados começam em `10`, cobrindo `pdf.<field>`, em
  vez de `14`; stderr difere do vanilla nas linhas de localização/sublinhado.

### M02 — ancorar somente `pdf`

- **Hipótese:** uma checagem de exclusão do span total, sem range positivo
  exato, aceita a âncora no target errado.
- **Hunk/transformação aplicável:** trocar o span do erro por
  `access.target().span()` no mesmo ramo.
- **Assassino nomeado:** `p1303_negativos_pdf_exatos_nos_perfis_sem_a11y` e
  probes `P1303-N/*/*`.
- **Resultado esperado:** `Violated` nos seis negativos.
- **Testemunha:** range cobre somente `pdf` (o range exato é registrado pelo
  ledger) e diverge de `14..23`, `14..25` ou `14..27`.

### M03 — incluir o ponto na âncora

- **Hipótese:** o oracle aceita ponto mais identificador por comparar apenas o
  texto principal do field.
- **Hunk/transformação aplicável:** shim temporário no construtor do diagnóstico
  cria `Span::from_range(field_span.id().unwrap(), 13..end)`, com `end` igual a
  `23`, `25` ou `27` conforme o field congelado.
- **Assassino nomeado:** `p1303_negativos_pdf_exatos_nos_perfis_sem_a11y` e
  probes `P1303-N/*/*`.
- **Resultado esperado:** `Violated` nos seis negativos.
- **Testemunha:** `13..23`, `13..25` e `13..27`; o byte `.` entra no span e o
  stderr deixa de ser byte-idêntico.

### M04 — deslocar o início um byte à esquerda

- **Hipótese:** o oracle valida comprimento ou fim, mas não a fronteira inicial.
- **Hunk/transformação aplicável:** shim temporário equivalente ao operador
  `start := expected.start - 1`, preservando `end`, materializado com
  `Span::from_range` para cada canário. Este operador é definido pela fronteira,
  independentemente da interpretação lexical usada em M03.
- **Assassino nomeado:** `p1303_negativos_pdf_exatos_nos_perfis_sem_a11y` e
  probes `P1303-N/*/*`.
- **Resultado esperado:** `Violated` nos seis negativos.
- **Testemunha:** início observado `13` em lugar de `14`, com fim correto.

### M05 — deslocar o fim um byte à direita

- **Hipótese:** o oracle valida começo mas tolera um byte posterior ao field.
- **Hunk/transformação aplicável:** shim temporário
  `end := expected.end + 1`, preservando `start = 14`, via `Span::from_range`.
- **Assassino nomeado:** `p1303_negativos_pdf_exatos_nos_perfis_sem_a11y` e
  probes `P1303-N/*/*`.
- **Resultado esperado:** `Violated` nos seis negativos.
- **Testemunha:** `14..24`, `14..26` e `14..28`, em vez dos três ranges
  congelados.

### M06 — corrigir somente `data-cell`

- **Hipótese:** uma amostra única é confundida com cobertura do trio.
- **Hunk/transformação aplicável:** usar `access.field().span()` somente quando
  `field == "data-cell"`; conservar `access.span()` para `header-cell` e
  `table-summary`.
- **Assassino nomeado:** `p1303_negativos_pdf_exatos_nos_perfis_sem_a11y`.
- **Resultado esperado:** `Violated`; quatro casos negativos falham e os dois
  de `data-cell` permanecem controles verdes.
- **Testemunha:** `header-cell` e `table-summary` começam em `10` nos perfis
  `default` e `html`, enquanto `data-cell` começa em `14`.

### M07 — corrigir somente o perfil sem `html`

- **Hipótese:** o teste cobre `default`, mas não a ortogonalidade entre `html`
  e `a11y-extras`.
- **Hunk/transformação aplicável:** escolher `access.field().span()` apenas se
  `Feature::Html` estiver desligada; no ramo negativo com `Html` ligado usar
  `access.span()`.
- **Assassino nomeado:** `p1303_negativos_pdf_exatos_nos_perfis_sem_a11y`, com
  probes `P1303-N/html/*` obrigatórios.
- **Resultado esperado:** `Violated`; exatamente os três negativos `html`
  falham, e os três `default` são controles verdes.
- **Testemunha:** em `html`, os três ranges começam em `10` e o stderr diverge;
  em `default`, começam em `14`.

### M08 — reordenar os dois hints

- **Hipótese:** o oracle compara hints como conjunto, não como sequência.
- **Hunk/transformação aplicável:** transpor as duas chamadas consecutivas a
  `.with_hint(...)`, sem alterar seus textos.
- **Assassino nomeado:** `p1303_negativos_pdf_exatos_nos_perfis_sem_a11y` e
  comparação byte a byte dos probes `P1303-N/*/*`.
- **Resultado esperado:** `Violated` nos seis negativos.
- **Testemunha:** vetor de hints aparece `[help-url, enable-feature]` em vez de
  `[enable-feature, help-url]`; stderr tem as duas linhas invertidas.

### M09 — alterar o nome da feature na mensagem

- **Hipótese:** o oracle usa `contains` ou normalização de pontuação em vez de
  igualdade exata.
- **Hunk/transformação aplicável:** somente no literal da mensagem, trocar uma
  ocorrência de ``a11y-extras`` por ``a11y_extras``; os hints ficam intactos.
- **Assassino nomeado:** `p1303_negativos_pdf_exatos_nos_perfis_sem_a11y` e
  comparação byte a byte dos probes `P1303-N/*/*`.
- **Resultado esperado:** `Violated` nos seis negativos.
- **Testemunha:** mensagem observada contém underscore no nome da feature e o
  SHA-256 do stderr difere do vanilla, embora span e hints coincidam.

### M10 — expor o trio sem `a11y-extras`

- **Hipótese:** os testes positivos passam, mas não há negação de exposição nos
  dois perfis sem a feature.
- **Hunk/transformação aplicável:** mutação temporária no construtor baseline do
  módulo `pdf`: tornar incondicional o bloco que registra `table-summary`,
  `header-cell` e `data-cell` (por exemplo, `if true`). É um ataque efêmero de
  P7, não mudança autorizada do patch final nem ampliação da allowlist.
- **Assassino nomeado:** `p1303_negativos_pdf_exatos_nos_perfis_sem_a11y` e
  probes `P1303-N/default/*`, `P1303-N/html/*`.
- **Resultado esperado:** `Violated`; os seis negativos tornam-se sucesso e
  não emitem o diagnóstico obrigatório.
- **Testemunha:** exit `0`/kind `function` onde se exigia um erro único e stderr
  vanilla; cardinalidade negativa cai de seis erros para zero.

### M11 — esconder o trio com `a11y-extras`

- **Hipótese:** os testes negativos passam, mas a superfície positiva nos dois
  perfis com a feature não é protegida.
- **Hunk/transformação aplicável:** mutação temporária no guard de registro do
  trio em `pdf.rs`, tornando-o sempre falso (`if false && ...`).
- **Assassino nomeado:** `p1303_positivos_pdf_exatos_nos_perfis_com_a11y` e
  probes `P1303-P/a11y/*`, `P1303-P/html+a11y/*`.
- **Resultado esperado:** `Violated`; os seis positivos deixam de ser
  `MATCH_VALUE`.
- **Testemunha:** acesso ao field falha ou deixa de produzir kind `function` e
  repr público, com exit não zero/stderr não vazio.

### M12 — generalizar dicionário ausente para field-only

- **Hipótese:** a correção local foi generalizada a todo acesso a campo e
  regrediu targets não-`Module`.
- **Hunk/transformação aplicável:** no cálculo geral de `span`, acrescentar
  `matches!(&target, Value::Dict(_))` aos casos que escolhem
  `access.field().span()`.
- **Assassino nomeado:**
  `p1303_sentinelas_de_span_module_e_nao_module` e o existente
  `p1301_controle_nao_module_dicionario_preserva_span_total`.
- **Resultado esperado:** `Violated`; a sentinela de dicionário falha enquanto
  os negativos `pdf` podem continuar verdes.
- **Testemunha:** para `repr(type((:).missing))`, o range deixa de ser o total
  `10..21` e passa a cobrir somente `missing`; a mensagem permanece
  `dictionary does not contain key "missing"`.

### M13 — regredir a projeção pública `std` para `global`

- **Hipótese:** o patch preserva spans, mas remove silenciosamente a projeção
  de nome selada por P1301r2.
- **Hunk/transformação aplicável:** no erro geral de `Value::Module`, substituir
  `if m.name() == "std" { "global" } else { m.name() }` por `m.name()`.
- **Assassino nomeado:**
  `p1303_sentinelas_de_span_module_e_nao_module` no probe `std.nope` e o
  existente
  `p1301_std_aliases_mensagem_vanilla_e_span_field_only_nos_quatro_perfis`.
- **Resultado esperado:** `Violated` em todos os probes ausentes sob `std`.
- **Testemunha:** mensagem observada diz `module `std`` em vez de
  `module `global``; o span field-only continua correto, isolando a causa.

### M14 — retirar o sucesso de `pdf.attach` e `pdf.artifact`

- **Hipótese:** o trio é corrigido à custa dos dois bindings ungated vizinhos e
  o gate só observa a coorte nova.
- **Hunk/transformação aplicável:** no construtor temporariamente mutado do
  módulo `pdf`, remover as duas definições adjacentes de `attach` e `artifact`,
  preservando o bloco do trio. A unidade semântica do ataque é “bindings
  ungated removidos”; a restauração é um único hunk inverso.
- **Assassino nomeado:** `p1303_sentinelas_pdf_ungated_e_html_disabled` e os
  existentes `p735_pdf_fields_sao_funcoes`,
  `p735_pdf_attach_produz_carrier_invisivel` e
  `p735_pdf_artifact_passthrough_do_body`.
- **Resultado esperado:** `Violated`; os dois vizinhos deixam de ter sucesso
  sem `a11y-extras`, independentemente de o trio permanecer correto.
- **Testemunha:** `type(pdf.attach)`/`type(pdf.artifact)` deixa de ser
  `function`, e as chamadas representativas falham em vez de preservar seus
  carriers/Content.

## Ordem, repetição, restauração e recibo por mutante

Para cada M01–M14, P7 deve executar a mesma sequência causal:

1. conferir hashes do contrato, oracle, testes e candidato; rodar o assassino
   focal no candidato limpo e obter PASS completo;
2. aplicar somente o hunk do mutante com `apply_patch` e guardar o diff literal
   e hashes pós-mutação;
3. compilar o mutante; falha de compilação é `INVALID_MUTANT_EXECUTION`, nunca
   `KILLED`;
4. executar o assassino em ordem normal, repetir a ordem normal e executar a
   ordem integralmente invertida; por chave `(perfil, probe)`, as três corridas
   devem concordar na testemunha de violação;
5. aplicar exclusivamente o patch inverso com `apply_patch`; são proibidos
   `git reset`, `git checkout`, restore destrutivo, staging e commit;
6. verificar que o hash do hunk/candidato voltou exatamente ao valor anterior,
   que `git diff --check` passa e que o assassino focal voltou a PASS antes de
   iniciar o mutante seguinte.

O ledger deve preservar por mutante: ID, hipótese, caminho e diff literal,
hashes antes/durante/depois, comando integral, features/perfil, três ordens,
exit codes, stdout/stderr integrais e hashes, classificação, duração,
testemunha e prova de restauração. M03 e M04 podem produzir o mesmo range nos
canários sem whitespace, mas continuam operadores distintos: M03 ataca a
fronteira lexical do ponto; M04 ataca aritmeticamente o início. Ambos precisam
de execução e recibo próprios; um resultado não substitui o outro.

Antes do selo e na verificação final, repetir o corpus completo nas ordens
normal e invertida, além da repetição da normal. Não comparar duração, diretório
temporário ou ordem física; comparar exatamente todos os observáveis listados
em C-P1303-v1. A ordem/repetição não aumenta o denominador: existem exatamente
14 mutantes aplicáveis.

## Política de `Unknown` e cálculo do score

`Unknown` e `EXECUTION_UNKNOWN` são proibidos em todos os doze casos principais,
sentinelas, execuções mutantes, ordens e repetições. Nunca são convertidos em
`Preserved`, `MATCH_VALUE`, `MATCH_DIAGNOSTIC`, PASS ou mutante morto. Parser
sem suporte, span irresolvível, identidade ambígua, timeout, crash, ausência de
output, falha de compilação ou budget esgotado tornam a execução
inconclusiva/bloqueada. Os opacos AT real e fallback PDF por versão estão fora
do corpus e do denominador; se observados adicionalmente, permanecem
`Unknown`, sem crédito.

O cálculo obrigatório é:

```text
mutation_score = mutantes válidos e mortos / mutantes aplicáveis
               = 14 / 14
               = 1.0
```

Qualquer mutante compilável sobrevivente, mutante obrigatório não executado,
testemunha ausente, restauração não provada ou ocorrência de `Unknown` impede o
selo e produz `P1303_BLOCKED_MUTANT_SURVIVED` ou
`P1303_BLOCKED_VERIFICATION`, conforme a causa. Não há arredondamento,
equivalência presumida, exclusão retroativa do denominador nem perdão por
“mudança pequena”.

## Budget de três tentativas e retorno decrescente

O budget causal é exatamente o do passo, no máximo três tentativas de candidato:

1. âncora do ramo especial em `access.field().span()`;
2. se a AST não preservar o span, transporte explícito do `Span` do field em
   `eval_field_access`, sem assinatura pública nova;
3. se o gate especial estiver causalmente errado, alinhamento do lookup interno
   ao fluxo vanilla no mesmo owner, preservando catálogo, mensagem e hints.

Cada tentativa exige hipótese refutável nova, conjunto focal e novo hash de
candidato. Duas revisões consecutivas com o mesmo vetor e a mesma causa exigem
revisão arquitetural antes da terceira. Três falhas pela mesma causa encerram
com evidência; não autorizam ampliar para entidades, `stdlib/pdf.rs`, CLI,
feature flags ou outros owners. O budget não permite afrouxar o contrato,
reduzir os 14 mutantes, adaptar assassinos ao patch ou promover `Unknown`.

Para a mecânica do gate mutante, uma transformação inválida pode ser corrigida
somente no mutador, sem tocar produto/contrato/oracle, até o limite causal acima;
se não for possível produzir o mutante compilável e observável, ele fica sem
assassino demonstrado e o avanço é bloqueado.

## Veredito adversarial anterior à execução

`P1303_ADVERSARIAL_PLAN_SUFFICIENT_FOR_EXECUTION`.

Os 14 mutantes mínimos exigidos pelo passo têm transformação aplicável,
assassino nominal, resultado esperado e testemunha observável. O plano cobre
as duas ordens, repetição, política estrita de `Unknown`, score obrigatório
`1.0`, restauração exclusivamente por `apply_patch` e budget de três
tentativas. Este é veredito de suficiência **do plano**, não selo do contrato,
execução dos mutantes, aprovação do patch ou certificação P1303. Um assassino
que não exista ou não produza a testemunha quando P5/P7 for materializado
invalida esta suficiência operacional e bloqueia antes da implementação ou do
selo, conforme a fase.

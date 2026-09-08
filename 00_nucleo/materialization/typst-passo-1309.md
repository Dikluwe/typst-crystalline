# Passo 1309 — rebaseline global pós-P1307/P1308 e seleção do próximo lote

**Estado:** pronto para execução.

**Natureza:** auditoria somente leitura do produto; não materializa paridade.

**Baseline inicial:** commit limpo
`eb24cd657fc2333dc7ea5393f7cfebf8c7192d39`.

**Vanilla ratificado:** upstream/main `a51e02804`.

## 1. Objetivo e limite da alegação

Reenumerar a superfície pública depois dos fechamentos P1307/P1308, executar
uma matriz bilateral fresca nos quatro perfis, reconciliar cada path histórico,
medir a paridade observável atual e selecionar deterministicamente uma única
coorte causal para o passo seguinte.

P1309 não edita Prompt L0, Núcleo Tekt, Rust, Cargo, fixtures históricas nem
testes produtivos. Seu allowlist de escrita contém somente:

- `00_nucleo/diagnosticos/p1309-*`;
- este passo durante sua redação inicial.

Qualquer mudança fora desse allowlist invalida a execução. O passo pode propor
o P1310, mas não o escreve nem o implementa. Uma mudança pública futura passa
pelos gates L0/ADR-0127 próprios.

O regime é uma auditoria bilateral segregada: inventário, execução,
classificação e veredito têm autorias distintas. Como o filesystem do projeto
é compartilhado, o relatório deve usar a formulação **“executado sem atestação
de isolamento técnico”**. Hashes provam identidade e ordem, não isolamento de
capacidades.

## 2. Evidência predecessora — não confundir focal com global

O commit baseline integra os fechamentos anteriores. Os recibos predecessores
relevantes são:

| evidência | SHA-256 | alegação limitada |
|---|---|---|
| `p1307-r6-final-report.md` | `f956fa66ef757845613d9b446a7b9cd08f6502b21caaaa616ff446d816068d84` | implementação dos encoders e transporte, com bloqueios históricos daquele instante |
| `p1307-r6-verification-final.json` | `48c89a1513ff0f6b56f79b286f1e81557b4bc5272423cd546a28a41b4edb1062` | veredito R6 então bloqueado |
| `p1308-r2-final-report.md` | `fc114f4892545ecc815e0b252f0190dcb27f569d9044ba4e34640a5b9804568a` | entrega acumulada autorizada e gates finais |
| `p1308-verification-r2-final.json` | `6f6d9cfd4d3cb8233915c0cf59f9734a1b021139486e09fb743f2646fa5c37c0` | PASS limitado P1308-R2 |
| `p1308-r2-public-matrix.json` | `89db8675ebe23d3cd8a9ddba4347445a8354f018378b70209d8ff45e0e901882` | 1982 células focais preservadas |
| `p1308-r2-workspace-tests.json` | `207dbd534febcdf207c914a5bd208e20416c9d9e1b5f35e7c561bb2fbb533c42` | 6620 testes passaram, zero falhas, três ignorados naquele estado |
| `p1308-mutation-ledger.json` | `c636972025a884bf33af0dedb316b77c82ef44535ca02ca78057d804d32e2837` | seis mutantes P1308 rejeitados |

Esses números possuem proveniência no working tree anterior ao commit e são
somente antecedentes. P1309 deve produzir medições novas no commit limpo; não
transportar as contagens como resultado atual.

A matriz P1308 é focal ao contrato P1307/P1308. Ela **não** substitui o
catálogo global de 627 probes P1304 e não prova paridade total da linguagem.
O verificador P1308 também registrou explicitamente 37 famílias históricas de
mutação P1307 não executadas e ausência de certificado geral P1307. P1309 deve
separar:

- **estado funcional observado:** o que a matriz fresca demonstra;
- **dívida de certificação:** ataques previstos mas ainda não executados;
- **dívida de linguagem:** divergência bilateral efetivamente observada.

Uma dívida de certificação não vira falha de linguagem sem testemunha. Uma
saída igual também não apaga a dívida de certificação.

## 3. Baseline histórico e reconciliação esperada — hipótese, não conclusão

P1304 mediu 627 probes × quatro perfis e manteve um ledger de 167 paths:

- 106 `MISSING_LANGUAGE_MEMBER`;
- 42 `INTENTIONAL_PRODUCT_EXTENSION`;
- 12 `WRONG_PUBLIC_REPR`;
- 1 `EXPECTED_FEATURE_DISABLED`;
- 6 `CLOSED_CONFIRMED`.

P1305 deveria mover os 12 paths de repr para fechados. P1307 deveria mover
`json.encode`, `toml.encode` e `yaml.encode` de ausentes para fechados. Se não
houver efeito incidental nem regressão, a reconciliação histórica esperada é:

- 103 membros ausentes;
- 42 extensões intencionais;
- 1 feature esperadamente desligada;
- 21 fechados históricos.

Isto é uma **previsão refutável**, não o resultado P1309. Qualquer contagem
diferente deve ser explicada path a path por observação fresca. Não ajustar o
catálogo para forçar estes números.

Inputs históricos a preservar sem editar:

| input | SHA-256 |
|---|---|
| `p1299-probe-catalog.json` | `649dd46f05e67d376a8096756a31207a6a7d4087acf5fcbdd6688a59c4f934ae` |
| `p1304-owner-ledger.tsv` | `01e038177dde246c3196af79bdb36eb13eb2e03364b0b091056cb2af8f4d6770` |
| `p1304-decision-report.md` | `92e7ecf2ac895c56831a0d4619ba79d703372cff029885bbe1101511f13425bd` |
| `p1304-encode-readiness.json` | `156f56b9f65ad5fd4310aa7785d4667c1f2c20e13f3f5c8be43b280700e7b950` |

## 4. Papéis e capacidades

Registrar executor, entradas legíveis, caminhos graváveis, contexto herdado,
hashes e instante de cada papel.

### A — autor do inventário

Recebe o vanilla pinado, o commit baseline e o método P1299/P1304. Produz novo
inventário e catálogo, sem ler resultados da matriz candidata e sem classificar
intenção. Pode escrever somente `p1309-inventory-*` e
`p1309-probe-catalog.json`.

### B — operador bilateral

Recebe catálogo congelado, binários e perfis. Produz outputs integrais e classes
mecânicas, sem editar catálogo, produto ou regras semânticas. Pode escrever
somente matrizes/logs P1309.

### C — classificador semântico

Recebe catálogo, matrizes, fontes vanilla/cristalinas e Prompts L0 explicitamente
necessários a cada divergência. Mede antes de decidir e produz o owner ledger.
Não altera L0/código e não seleciona por conveniência do patch.

### D — adversário/replayer

Recebe entradas congeladas e tenta refutar completude, estabilidade,
classificação e seleção. Não corrige runner nem ledger silenciosamente. Revisão
do runner gera sucessor com hipótese/delta/custo registrados.

### E — verificador

Recebe somente artefatos selados e recibos. Recalcula hashes, contagens,
transições, ownership e prioridade; não escreve os artefatos julgados. Emite o
certificado ou bloqueio.

Nenhuma autoridade controla simultaneamente catálogo, execução, classificação
e veredito.

## 5. P0 — congelamento reproduzível

Produzir `p1309-baseline.json` com:

- HEAD exato, branch, UTC/local, `git status --short`, staged, diff/stat e seus
  SHA-256;
- exigência de working tree tracked/staged limpa antes do primeiro build;
- lista/hash dos untracked preexistentes, sem apagá-los;
- `/usr/local/bin/typst` com SHA-256 esperado
  `7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8`;
- build release fresco do commit baseline em target descartável dedicado,
  guardando argv, cwd, env, início/fim, exit, stdout/stderr e hash do binário;
- hashes de scripts, catálogos, ledgers, fixtures e Prompts consultados;
- política: timeout, crash, saída não parseável, mismatch de fixture/binário ou
  observável obrigatório ausente = `EXECUTION_UNKNOWN`, sempre bloqueante.

Não identificar o baseline pela string `--version`. Não reutilizar binário RAM
P1308 como candidato atual.

## 6. P1 — reenumeração integral independente

Não basta reaplicar o catálogo antigo. O autor A deve reenumerar as superfícies
vanilla e cristalina e depois reconciliá-las:

1. inventário vanilla `default` e `html`, preservando feature requirements,
   namespaces, tipo, nome/repr e ancestors;
2. inventário cristalino correspondente a partir do binário fresco;
3. catálogo novo com união das rotas alcançáveis, incluindo ancestors
   bloqueados como probes executáveis;
4. reconciliação 1:1 com todos os 627 probes P1304 e os 167 paths do ledger;
5. lista explícita de `added`, `removed`, `renamed`, `split`, `merged` e
   `unchanged`; nenhuma remoção/deduplicação sem justificativa de linguagem;
6. controles explícitos para os quatro encoders públicos, módulos/repr,
   imports, spans P1303/P1306, constructors P1300 e gates PDF/HTML.

O alvo vanilla permanece pinado; descoberta nova pode revelar lacuna do
catálogo antigo, não mudança upstream. IDs históricos nunca são reciclados.

## 7. P2 — matriz quadrilateral fresca

Executar cada probe bilateralmente em:

- `default`;
- `html`;
- `a11y`;
- `html+a11y`.

Para cada lado guardar argv, fixture/source integral, exit, stdout, stderr,
diagnósticos estruturados quando disponíveis, duração e hashes. Classificar
somente como:

- `MATCH_VALUE`;
- `MATCH_DIAGNOSTIC`;
- `CRYSTALLINE_ONLY`;
- `VANILLA_ONLY`;
- `DIFFERENT_VALUE`;
- `DIFFERENT_DIAGNOSTIC`;
- `EXECUTION_UNKNOWN`.

Executar:

1. corpus integral em ordem canônica;
2. repetição integral da ordem canônica;
3. corpus integral em ordem invertida.

Os mapas por chave `(probe_id, perfil)` devem ser idênticos nas três ordens.
Não usar exit zero do runner como substituto da leitura das contagens e células.
Qualquer diferença de repetição/ordem ou `EXECUTION_UNKNOWN` obrigatório
bloqueia classificação e seleção.

## 8. P3 — sentinelas pós-fechamento

Além do catálogo global, manter suplemento que não infla sua cardinalidade:

### P1307 — encoders

- presença, kind e repr de `cbor/json/toml/yaml.encode`;
- JSON/TOML pretty default/true/false, YAML sem `pretty`;
- ordem, escaping, multiline, nesting, `none`, bytes, Symbol, Content, opacos e
  floats especiais conforme contrato fechado;
- erros de aridade/cast/named, mensagem, hints e span resolvido;
- parent decoders ainda chamáveis e rotas `.with` contratadas;
- `csv.encode`, `xml.encode` e `read.encode` ausentes.

### P1305/P1306/P1308 — repr, módulos e causalidade

- arrays 39/40/41/42/81/256 e preservação integral dos dados;
- `repr(std)`, `color.map` grande/pequeno, módulo importado nomeado e controles
  de anonimato separados;
- global real chamado `global`, arquivo ordinário `std.typ` chamado `std`,
  aliases/sombra/nesting/reexport e lookup existente;
- spans field-only dos módulos e gates PDF;
- traces `while calling`, callback source, Args+none, tipo longo de cast e forma
  pública de Args longo, nos casos autorizados P1308.

### Regressões gerais

- todo probe que era MATCH em P1304;
- todo path marcado fechado por P1300–P1308;
- pelo menos um controle positivo e um negativo por feature;
- stdout/stderr lateral: não apagar warnings para fabricar igualdade.

Suplementos são contados separadamente e nunca apresentados como novos paths
do catálogo principal.

## 9. P4 — ledger semântico novo

O classificador C produz `p1309-owner-ledger.tsv`, preservando todos os paths
históricos e acrescentando somente descobertas demonstradas. Cada linha contém:

- path e probe canônico;
- perfis e classe runtime por perfil;
- classe histórica e transição;
- classe semântica atual;
- fonte vanilla `file:line` e fonte cristalina `file:line`;
- Prompt L0 proprietário, SHA-256, `Hash do Código` declarado e consumer único;
- observável de linguagem, intenção normativa e distinção entre ambos;
- hipótese causal e condição concreta de refutação;
- gate ADR-0127 provável e ação recomendada;
- universo principal, suplemento ou dívida de certificação.

Classes semânticas mínimas:

- `NEW_REGRESSION`;
- `MISSING_LANGUAGE_MEMBER`;
- `WRONG_PUBLIC_VALUE`;
- `WRONG_PUBLIC_REPR`;
- `WRONG_PUBLIC_KIND_OR_IDENTITY`;
- `DIAGNOSTIC_DIVERGENCE`;
- `L0_CONTRADICTION`;
- `INTENTIONAL_PRODUCT_EXTENSION`;
- `EXPECTED_FEATURE_DISABLED`;
- `CLOSED_CONFIRMED`;
- `CERTIFICATION_DEBT`;
- `UNRESOLVED`.

Não inferir intenção da execução. Uma extensão só permanece intencional se o
L0/ADR vigente realmente a autorizar; silêncio normativo não basta. Ler
integralmente apenas os L0 dos paths que exigirem decisão e registrar essa
leitura. Confirmar ownership 1:1 e integridade dos Núcleos antes de recomendar
qualquer futura materialização.

## 10. Métricas de paridade — três denominadores, sem marketing

O relatório deve publicar separadamente:

1. **igualdade bruta de células:** `MATCH_VALUE + MATCH_DIAGNOSTIC` dividido
   por todas as células válidas da matriz;
2. **igualdade ajustada ao alvo:** mesmo numerador/denominador após excluir
   somente extensões intencionais documentalmente confirmadas; feature
   desligada esperada continua sucesso quando o diagnóstico coincide;
3. **cobertura de paths:** fechados, ausentes, divergentes, extensões,
   feature-gated e não resolvidos no ledger, sem converter quatro perfis em
   quatro features distintas.

Fornecer numerador, denominador, percentagem, fórmula, timestamp e estado de
fonte de cada número. Não chamar nenhuma dessas métricas de “paridade total”.
Layout, exportadores, outputs binários e comportamento fora do corpus continuam
fora da alegação.

## 11. P5 — transições e regressões temporais

Comparar P1309 com P1304 por `(probe_id, perfil)`:

- `MATCH_* -> não-MATCH` = candidato a `NEW_REGRESSION`;
- não-MATCH -> `MATCH_*` = fechamento observado;
- mudança entre tipos de não-MATCH = reclassificação, não fechamento;
- path novo = descoberta nova, nunca regressão temporal automática;
- diferença de adapter/fixture/binário = `Unknown` até corrigida.

Aceitação mínima:

- `json.encode`, `toml.encode`, `yaml.encode` deixam `VANILLA_ONLY` e passam a
  igualdade nos quatro perfis;
- os 12 paths de repr P1305 permanecem iguais;
- os seis controles fechados anteriores permanecem iguais;
- nenhum MATCH histórico regride;
- os diagnósticos P1306/P1308 focais permanecem preservados;
- toda discrepância da previsão 103/42/1/21 é explicada nominalmente.

Se alguma dessas condições falhar, o passo seleciona primeiro a regressão ou
produz bloqueio; não continua diretamente para feature nova.

## 12. P6 — dívida de certificação P1307

Criar um ledger separado, sem executar mutantes produtivos neste passo, para as
37 famílias históricas citadas pelo verificador P1308:

- localizar plano/origem e mapear duplicatas, supersessões e testemunhas já
  cobertas pelos seis mutantes P1308;
- marcar cada família `EXECUTED_KILLED`, `SUPERSEDED_WITH_EVIDENCE`,
  `STILL_PENDING`, `NOT_APPLICABLE_WITH_REASON` ou `UNKNOWN`;
- não contar mutante que não compilou/aplicou como morto;
- não alegar score 1.0 sem executar todas as famílias aplicáveis restantes;
- recomendar passo adversarial próprio se houver pendências, sem chamar isso de
  divergência de linguagem.

P1309 pode concluir simultaneamente “encoders iguais no corpus” e “certificação
adversarial incompleta”. Estes vereditos não se anulam.

## 13. P7 — seleção causal determinística

Agrupar apenas divergências demonstradas por causa e owner; glue não transforma
famílias semânticas distintas numa coorte única. Aplicar a prioridade:

1. `NEW_REGRESSION` reproduzível;
2. `L0_CONTRADICTION` com rota canônica já funcional;
3. `DIAGNOSTIC_DIVERGENCE` completa e isolada;
4. `WRONG_PUBLIC_VALUE/KIND/IDENTITY/REPR`;
5. `MISSING_LANGUAGE_MEMBER` com carriers existentes;
6. `MISSING_LANGUAGE_MEMBER` que exige nova entidade/contrato/fase;
7. contradição ou observável sem rota canônica demonstrada.

`INTENTIONAL_PRODUCT_EXTENSION`, `EXPECTED_FEATURE_DISABLED`, `UNRESOLVED` e
`CERTIFICATION_DEBT` não entram automaticamente na competição de paridade.
Produzir, porém, uma recomendação adversarial paralela quando existir
`CERTIFICATION_DEBT` pendente.

Dentro da mesma prioridade:

1. menor número de owners produtivos;
2. maior número de paths com a mesma causa comprovada;
3. menor superfície de regressão;
4. ID canônico lexicograficamente menor.

Publicar tabela de todas as coortes elegíveis, sua prioridade, owners, gates,
paths, risco e razão de perda. Selecionar exatamente uma para P1310. Se não
houver coorte elegível ou houver `Unknown` obrigatório, não inventar vencedor.

## 14. P8 — ataques ao auditor

O adversário D deve demonstrar que o protocolo rejeita pelo menos:

1. catálogo que omite um dos 627 probes históricos;
2. catálogo que omite membro novo descoberto;
3. deduplicação de dois paths por nome semelhante;
4. troca vanilla/cristalino;
5. reutilização do binário P1308;
6. perfil HTML/a11y trocado ou omitido;
7. stdout igual com stderr divergente apagado;
8. `VANILLA_ONLY` reclassificado como feature desligada sem fonte;
9. extensão inferida somente porque o cristalino a aceita;
10. encoders marcados fechados apenas por presença;
11. repr marcada fechada sem preservar dados integrais;
12. regressão histórica escondida por mudança de ID;
13. `Unknown` convertido em MATCH;
14. contagem de suplementos inflando o catálogo;
15. dívida de certificação contada como falha funcional;
16. seleção que ignora a ordem de prioridade ou o desempate;
17. owner 1:N ou Núcleo inválido tratado como pronto;
18. matriz com divergência entre ordem normal/repetida/invertida.

O gate discriminatório exige `18/18` ataques válidos rejeitados,
`mutation_score = 1.0` do **classificador/auditor** e testemunha por ataque.
Isto não substitui as 37 mutações de produto P1307.

Budget de calibração: duas revisões focais por classe de falha. Cada revisão
registra hipótese, delta, casos corrigidos/regredidos, causa dominante, custo e
hashes. Duas revisões consecutivas sem ganho obrigam redesenho; não autorizam
afrouxar o contrato ou repetir cegamente o corpus integral.

## 15. P9 — gates finais

Executar e registrar:

```text
cargo fmt --all -- --check
cargo build --workspace --release
cargo test --workspace --release --no-fail-fast
crystalline-lint .
git diff --check
```

Também exigir:

- working tree de produto/L0 byte-idêntica ao baseline após a auditoria;
- hashes do catálogo, três matrizes, suplementos, ledger e scripts;
- contagens recalculadas por verificador independente;
- zero `EXECUTION_UNKNOWN` obrigatório;
- estabilidade integral nas três ordens;
- ownership/núcleos válidos;
- ataques do auditor 18/18 rejeitados;
- nenhum stage, commit, push ou limpeza de artefato histórico.

Warnings/info do linter devem ser preservados e classificados; exit zero não
autoriza afirmar “zero findings”. Testes ignorados são reportados nominalmente.

## 16. Artefatos e vereditos

Artefatos mínimos:

```text
p1309-baseline.json
p1309-manifest.json
p1309-inventory-vanilla-default.json
p1309-inventory-vanilla-html.json
p1309-inventory-crystalline-default.json
p1309-inventory-crystalline-html.json
p1309-probe-catalog.json
p1309-catalog-reconciliation.json
p1309-matrix-normal.json
p1309-matrix-repeat.json
p1309-matrix-reverse.json
p1309-sentinels.json
p1309-transition-ledger.tsv
p1309-owner-ledger.tsv
p1309-certification-debt.json
p1309-selection.json
p1309-adversarial-ledger.json
p1309-gates.json
p1309-verification.json
p1309-certificate.json
p1309-final-report.md
```

Vereditos possíveis:

- `P1309_PASS_P1310_COHORT_SELECTED` — auditoria íntegra e uma coorte elegível;
- `P1309_PASS_NO_ELIGIBLE_PARITY_COHORT` — auditoria íntegra, sem divergência
  elegível;
- `P1309_BLOCKED_REGRESSION` — fechamento anterior regrediu e deve vencer;
- `P1309_BLOCKED_UNKNOWN_OR_INSTABILITY` — observável obrigatório opaco ou
  ordem/repetição divergente;
- `P1309_BLOCKED_AUDIT_INTEGRITY` — baseline, catálogo, ownership, ataques ou
  allowlist inválidos.

O certificado atesta somente o corpus, baseline, classificação e seleção
registrados. O relatório final apresenta separadamente: paridade observada,
transições, dívida de linguagem, extensões, feature gates, dívida de
certificação e próximo lote. Não declarar equivalência funcional geral.

Não fazer commit, stage, push, remoção ou alteração de artefatos históricos sem
pedido explícito do dono.

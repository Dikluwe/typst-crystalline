# Prioridade 4 — sessão validada de fontes materializada

A sessão está implementada no watch. Cada ciclo continua criando um World novo
para sources/bytes/dependências; somente uma geração validada de book/slots pode
ser conservada. Nas sondas pareadas, a CPU por edição caiu de forma consistente
nos três blocos: mediana das razões por par indica **47,54% menos CPU no Hello**
e **30,72% menos na tabela de 320 células**. Não é um ganho geral de compile nem
um certificado de estabilidade dos tempos absolutos.

O custo observado foi aproximadamente **4 MiB de RSS adicional** e **13 ms de
CPU adicional na primeira compilação**. A materialização e o reuso estão validados;
D10 conserva acompanhamento do custo de startup/memória e do restante da descoberta,
que ainda ocorre a cada ciclo. Esses custos não são ocultados como fechamento total.

## Ratificação e proveniência

Dono confirmou com `Confirmo` o pacote
`00_nucleo/materialization/watch-catalogo-2026-10-07/manifest.json`, SHA-256
`21dee05f473b777ae67233f30a1182d0d4368bbedff122b0ec146f03b019c610`.
Receipt imutável: `watch-catalogo-execucao-2026-10-07/ratification.json`.

HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado.
`before.json.gz` e `tested-final.json.gz` conservam UTC, estado exato dos arquivos
produtivos/L0, lista dos arquivos alterados e `git diff HEAD --stat`.
A verificação final foi registrada em 2026-10-08T01:42:59.027314+00:00, ainda
2026-10-07 em São Paulo. Cada corpus possui before/after próprios com seus UTCs,
identidades das fontes/configuração, scripts e binários. Não misturar esta rodada
com os tempos anteriores das prioridades 1–3 nem somar porcentagens históricas.

- Anterior congelado: SHA-256
  `96f22a1d63d51bfcc258ca2849563b037e66a67b7e34febaf4779da2dd146a4b`,
  `/tmp/watch-catalogo-2026-10-07/previous-96f22a1d`.
- Atual `target/release/typst`: SHA-256
  `fc5b2571a149763526c5686149a538259d64a045071103609600bb2823491d4d`.
- Vanilla: `lab/typst-original/target/release/typst`, SHA-256
  `7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`,
  pin `9f2b6e8715237cb086899a42873660fe744622e8`; receita de build P1489.
  Usado no corpus comportamental, não como denominador dos ganhos desta rodada.

Builds: `cargo build --offline -j 1` e
`cargo build --offline --release -p typst-wiring -j 1`; logs preservados.
O binário atual tem a mesma identidade no contador, corpus, comparação HTML,
geometria e todas as coletas de desempenho. A organização final dos metadados L0
não mudou o binário nem os corpos executáveis. Não houve commit.

## Implementação e fronteiras

API L3 opaca `typst_infra::world::FontCatalogSession::new()` e builder
`SystemWorld::with_font_catalog_session(Arc<FontCatalogSession>, &[PathBuf])`.
A sessão possui Mutex de uma geração e Arc do par FontSlots/FontBook; builders
continuam lazy. O helper privado de compile recebe sessão opcional; compile comum
conserva seu caminho fresco, watch passa uma sessão por execução.

Em cada nova demanda de catálogo: repetir descoberta fontdb/projeto, conservar
ordem/origem/índices, validar identidades Unix e comparar chave. Reuso conserva
slots carregados; mudança constrói o catálogo a partir do Database/slots já
preparados e revalida antes de guardá-lo. Recusa/erro/instabilidade usa rota fresca.
SharedFile recusa retenção e preserva bytes retidos no caminho fresco; Binary
continua excluído. Fora de Unix não há retenção. Limites de elegibilidade: 16384
descritores e 1 MiB de paths; exceder não trunca o catálogo fresco.

Sources, arquivos lidos, FileIds, dependências e falhas negativas não são herdados.
Coverage precomputada segue o book; coverage ausente continua por World. Inputs,
plugins/pacotes, relógio, métricas/shaping, eval/layout e contexto conservam seus
contratos. Sem novo unsafe, global, cache em disco, trait L1 ou flag. Mantidos
status-before-diagnostics, arm/publicação, último artefato, HTTP, evict e polling.
Fontes editadas sozinhas continuam sem disparar ciclo; são reconhecidas na próxima
recompilação, como no Tekt anterior.

Os sete owners produtivos aprovados foram os únicos alterados. Um oitavo owner,
exclusivamente de testes (`world/tests`), precisou adaptar um fixture que mutava
o par agora atrás de Arc. L0 atualizado primeiro em fluxo contínuo ADR-0127;
Arc::get_mut no Arc exclusivo preserva as assertions. Evidência E0596 e emenda em
`green-compile-failure.log` / `test-fixture-amendment.json`. Não amplia contrato.
`scope-verification.json` / `verification.json` confirmam 17 arquivos dentro do
escopo (consumers, owners e Núcleo), e nenhum arquivo herdado fora dele alterado.

## RED→GREEN, lint e limites da suíte

RED: interfaces de teste com implementação provisória compilaram; **10 testes
falharam e 5 passaram** (`red.log` e `red-state.json.gz`). GREEN final: **19 testes
novos passaram**; um helper marcado ignored é executado em subprocesso pelo teste
principal, isolando alterações de FONTCONFIG_FILE do restante da suíte.

Casos: key estável, adição/remoção, negativo→válido, mtime restaurado/mudança de
mesmo tamanho, substituição atômica, projeto aninhado, links, configuração,
SharedFile recusado, limites, identidade instável, poison, cache renovado,
FontSlot negativo recuperado, builders lazy/substituídos, diferentes configurações
concorrentes e sources/reads novos. Assertions de Arc caracterizam reuso interno;
a aceitação de linguagem está no corpus abaixo.

Suíte ampla `cargo test --offline --workspace -j 1 -- --test-threads=1`:
**8135 passaram, 9 falharam, 6 ignorados**. Ela não ficou integralmente verde.
As sete falhas HTML/bundle reproduziram com o anterior96f22 congelado, com mesmos
exit/stderr/HTML; witness em `behavior/html-baseline.json`. São resíduos anteriores,
não corrigidos nesta prioridade. Duas falhas restantes eram bind localhost negado
pelo sandbox; repetição autorizada fora da restrição passou **2/2** (`http.log`).
Bibliotecas core/infra/shell e query somam **7973 testes passados** nesta suíte.

O primeiro lint identificou blocos de Núcleos duplicados/fora da posição nos L0
instalados. Foram reunidos num bloco canônico antes da primeira seção, conservando
pins herdados e novo pin efetivo; resselo foi local, sem fix-hashes global.
Lint completo final: **0 erros, 535 warnings e 1640 infos**. Logs anteriores e
final preservados. `seals.json` confirma hashes dos oito pares proprietários.

## Corpus do watch e equivalência

`behavior/resources.json` e `mutations.json`: anterior, atual e vanilla pinado
passaram edição de main/import/read/PNG, remoção/recriação, erro de sintaxe e
recuperação, import antes ausente e criação. Erros preservaram o PDF anterior.
Ambos os Tekt reconheceram fonte de projeto substituída e adição em outro caminho
na próxima edição do main. Vanilla manteve fonte original, observado mas não
copiado como requisito. Fonte exclusiva e sua derivação/hash constam do receipt.
Rasters de PNG mudaram de vermelho para azul e coincidiram nos estados comparados.

Texto extraído coincidiu em todos os pares temporizados. `geometry.json` confirma
texto, posições das palavras, dimensões/páginas e seleção de fontes iguais nos
seis pares de PDFs finais Hello/tabela. Não é declaração de paridade global nem
regra de igualdade de bytes. Os demais resíduos PDF/HTML/SVG continuam independentes.

GDB fora dos tempos (`debugger.json`): compilação inicial + três edições produziram
**1/2/3/4 validações cumulativas, 1/1/1/1 aquisições e zero fallback fresco**.
Assim o catálogo foi materializado uma vez e validado nos ciclos seguintes.

## Desempenho pareado e custo

Primeira coleta sequencial: três processos por motor/documento, dez edições reais
por processo. Atual menor em todos os blocos, mas a última rodada oscilou muito;
`performance/results.json` foi conservado como indicativo.

A primeira tentativa intercalada invalidou sua qualificação: `write_text` expôs
um arquivo vazio entre truncate/write; o log mostrou compilação válida do estado
vazio. Harness corrigido para substituição **atômica** do source. A tentativa
inválida e seu código estão em `interleaved-invalid.json`/`interleaved-nonatomic.py`;
nenhum número dessa tentativa serve para fechar desempenho ou inferir stale cache.

Coleta final `performance/interleaved-results.json`: três pares de processos por
caso, dez pares de edições após aquecimento. Dois watchers coexistem, um é editado
por vez, alternando ordem; polling do parceiro permanece como interferência
ambiental pequena, não CPU atribuída ao processo medido. CPU obtida por runtime
das threads; latência inclui polling/publicação. Affinity fixa, RAYON=1, UTC,
locale C.UTF-8 e SOURCE_DATE_EPOCH=0; todos os argv no receipt. Nenhuma concorrência
reconhecida pelo monitor foi detectada, sem garantia de máquina inteira ociosa.

| Sonda | CPU anterior / atual (ms) | Edição→publicação anterior / atual (ms) | Redução de CPU, mediana das razões por par | RSS adicional |
|---|---:|---:|---:|---:|
| Hello, texto da revisão muda | 162,05 / 87,17 | 196,92 / 116,98 | 47,54% | 3,67 MiB |
| Tabela 320, primeira célula muda | 273,00 / 183,34 | 298,46 / 219,17 | 30,72% | 3,91 MiB |

Redução calculada por `1 - median(cpu_atual / cpu_anterior)` nos pares,
**não** divisão das duas medianas globais. Razões medianas por bloco: Hello
0,534 / 0,529 / 0,520; tabela 0,650 / 0,701 / 0,698. Atual venceu CPU em 30/30
pares Hello e 28/30 tabela. Bootstrap hierárquico, três clusters de processos,
95% para razão CPU: [0,512;0,546] e [0,580;0,750]. São três clusters apenas,
sem garantia de generalização. `analysis.json` conserva cálculos/limites.

Tempos absolutos variaram entre épocas/blocos; **não há certificado estrito de
estabilidade absoluta**. A direção relativa foi reproduzida nos três blocos
pareados, qualificando o benefício para estes ciclos de watch. Não extrapolar
para compile CLI comum, workloads sem fontes, outras máquinas ou plataformas.
RSS observado não prova limite para sessões longas alternando muitas famílias:
a sessão conserva slots/fonts carregados de sua única geração.

Startup separado (`startup-results.json`), três rodadas de três pares por caso:
Hello CPU 109,28→122,43 ms e parede 119,76→124,65 ms; tabela CPU 152,67→165,64 ms
e parede 155,06→168,94 ms. Essa série não tem monitor separado de concorrência,
portanto é indicativa. A identidade/guardas acrescentam trabalho na primeira
compilação; o ganho é nos ciclos seguintes. Reavaliação do caminho: manter a
retenção ratificada pelo benefício repetido por edição e atualização correta,
registrando custo de startup/memória como acompanhamento aberto em D10, sem
chamar todo o débito ou toda a performance de encerrados.

## Reprodução

`performance/measure.py`: série sequencial; `performance/interleaved.py`: pares
atômicos; `performance/startup.py`: startup isolado. `behavior/check.py`: recursos
e recuperação; `behavior/html-baseline.py`: resíduos HTML contra anterior.
`debugger.py` precisa de ptrace autorizado e deve rodar fora de benchmarks.
Os scripts encerram somente watchers próprios e editam fixtures em /tmp.
`verification.json` agrega identidade de binários, receipts, scope, seals e checks.

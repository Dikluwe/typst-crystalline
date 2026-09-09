# Passo 1335 — análise global de paridade após P1323–P1334

**Estado:** pronto para execução; somente o plano foi escrito.

**Natureza:** auditoria do produto. Não implementar correções nem alterar L0.

**Modelo autorizado:** `00_nucleo/materialization/typst-passo-1322.md`.
Não consultar outros passos ou a pasta context por iniciativa própria.
Os resultados e a decisão deste passo deverão ficar em diagnósticos, não aqui.

## 1. Pergunta e entrega substantiva

O que ainda falta para a paridade com o vanilla ratificado? O que foi realmente
fechado desde a análise global P1322? Houve regressões? Qual causa merece o
próximo lote, considerando o produto inteiro e não apenas a sequência recente?

Reenumerar a superfície pública e executar uma comparação global nova.
O corpus focal P1334 não substitui o inventário global. A sugestão de investigar
math em seu relatório é uma alternativa a medir, não o vencedor deste passo.
Não escolher antecipadamente calc.abs, math, CSV ou outra família.

O relatório final deve começar por diferenças de linguagem, testemunhas e efeito
para o usuário. Separar claramente: falta de funcionalidade, valor incorreto,
diagnóstico divergente, diferença mecânica e insuficiência da evidência.
Hashes, procedimento e logs sustentam a análise; não a substituem por gates.

## 2. Limites e estado de partida

Escrever somente este passo, artefatos novos `00_nucleo/diagnosticos/p1335-*`
(incluindo `p1335-fixtures/`) e temporários exclusivos registrados. Não alterar
Rust, L0, Núcleos, Cargo, configuração, testes produtivos ou evidências anteriores.
Sem mutações no produto, stage, commit, push ou limpeza. Usar apply_patch para
edições locais. Scripts de auditoria neste namespace não são consumers produtivos.

Na redação, HEAD `d31047d7b8af7837c84adae4ded3d2ff50c62093`, com working tree
não commitado. As alterações rastreadas estão nestes pares L0/consumer:

- field_access: `00_nucleo/prompts/compiler/eval/bindings/field_access.md`
  e `01_core/src/compiler/eval/bindings/field_access.rs`;
- call_dispatch: `00_nucleo/prompts/compiler/eval/call_dispatch.md`
  e `01_core/src/compiler/eval/call_dispatch.rs`;
- modules: `00_nucleo/prompts/compiler/eval/modules.md`
  e `01_core/src/compiler/eval/modules.rs`;
- tests: `00_nucleo/prompts/compiler/eval/tests.md`
  e `01_core/src/compiler/eval/tests.rs`;
- stdlib: `00_nucleo/prompts/compiler/stdlib/_comum.md`
  e `01_core/src/compiler/stdlib/mod.rs`;
- calc: `00_nucleo/prompts/compiler/stdlib/calc.md`
  e `01_core/src/compiler/stdlib/calc.rs`;
- loading: `00_nucleo/prompts/compiler/stdlib/loading.md`
  e `01_core/src/compiler/stdlib/loading.rs`;
- wiring: `00_nucleo/prompts/wiring.md` e `04_wiring/src/main.rs`.

Preservar essa árvore, não exigir HEAD limpo nem commitar para iniciar.
No começo da execução, congelar HEAD, branch, UTC, staged, status com pathspecs
que excluam pastas restritas, diff HEAD integral/stat, untracked relevantes e
inventário SHA-256 de produto/L0/evidências protegidas. Conferir o fechamento
P1334; qualquer diferença deve ser identificada, nunca descartada. Mudança externa
durante a execução invalida a medição afetada: preservar e recongelar antes de repetir.

Compilar a árvore efetiva em target dedicado, Cargo release com --locked.
Registrar argv, ambiente relevante, início/fim UTC, saídas e SHA-256 do executável.
Usar /dev/shm se capacidade e permissão permitirem; senão /tmp com motivo medido.
Cache copiado sem hardlinks mutáveis; não apagar temporários anteriores.
O binário P1334 é antecedente, não dispensa confirmar o estado atual.

Vanilla: upstream/main **a51e02804**, sem resync. Confirmar `/usr/local/bin/typst`,
SHA-256 `7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8`.
Não identificar binários por --version; o target da raiz é cristalino.

## 3. Antecedentes pinados, não resultados desta auditoria

Paths abaixo relativos a `00_nucleo/diagnosticos/`; hashes conferidos na redação:

| Entrada | SHA-256 | Uso |
| --- | --- | --- |
| p1322-o-que-falta-para-paridade.md | 1fec4e8816ee649244cdbaa3c86d9c8516ac84c944677e5b0d9c9929ff039b94 | análise global anterior e seus limites |
| p1322-probe-catalog.json | a9853e4c54663a258a9dd706c1c974930579c65e335b283243b10eae19b4889a | catálogo a reconciliar integralmente |
| p1322-classification-owner-ledger.tsv | 6fdf058c92dda6f3ac11dd691c1a53b4edf8c4fc3d5529cb32b9cd23bc1cce3a | causas/owners históricos |
| p1322-classification-selection-r2.json | 26f24e212bffa82ca9f3e48f8e46ce37c414cb514c96ae16cacaaea1050fc345 | decisão final, não a versão supersedida |
| p1322-closure.json | 46ed343a40d48070a4f3e7ea0cf69633b089fb56111361d10f21274bb77f5042 | proveniência global histórica |
| p1334-final-report.md | ca2d8080d08093a776416a66d844b1cf31ed887426b30060cf3c4afead3b4092 | fechamento focal e pendências |
| p1334-metrics.json | 307f80168b931184cd58744bf26eca5e758e5766521c9ac1ca39eff19b0b13c3 | métricas focais, fora do denominador global |
| p1334-ab-freeze-r1.json | 684143d5fdbb4005e5d641f772e603f48da1c32c928840c89873bbcff5fa50b5 | corpus focal e suplemento entre arquivos |
| p1334-closure.json | 4eda2f0d1ad7598471c088773c180f63ed453d88e5e2fb99e96d3ae84f63ade8 | estado produtivo e cadeia de preservação |

Consultar os relatórios/recibos de fechamento P1323–P1334 em diagnósticos e
congelar os inputs efetivamente usados antes das comparações. Usar os sucessores
finais, preservando retificações e versões inválidas como histórico.
Não carregar JSONs extensos indiscriminadamente: alguns contêm cópias do produto
e não podem ser enviados a um papel limitado a oráculos.

As contagens de P1322 e P1334 pertencem aos respectivos baselines identificados,
não são medição atual. Não comparar percentagens de inventário com as de um
corpus focal nem interpretar células repetidas como funcionalidades fechadas.

## 4. Inventário global novo

Reenumerar bilateralmente namespaces, membros, ancestors, requisitos de feature,
kind e repr públicos em default/html. Não instrumentar nem modificar o produto.
Se a descoberta estrutural exigir adapter externo, declarar sua projeção e
validar rotas reais no CLI; enumeração interna não prova disponibilidade pública.
Informação não observada fica não medida, nunca presumida igual.

Congelar catálogo sucessor com união das rotas, controles negativos e
reconciliação de cada ID P1322: unchanged, added, removed, renamed, split ou
merged. Não omitir membro porque o ancestor bloqueia lookup/chamada. Validar
expansão de modificadores Symbol antes da matriz, sem recombinações artificiais.
Preservar a distinção entre probes principais e linhas suplementares dos ledgers.

Executar bilateralmente nos perfis default, html, a11y e html+a11y, com flags
efetivas verificadas por comando. Repetir ordem canônica e executar inversa;
concorrência limitada/registrada, recomposição por (probe_id, perfil).
Guardar fontes, fixtures, cwd, argv, exit, stdout/stderr integrais, UTC e hashes.
Nenhum warning, hint ou trace é removido para fabricar igualdade.

Classificar MATCH_VALUE, MATCH_DIAGNOSTIC, VANILLA_ONLY, CRYSTALLINE_ONLY,
DIFFERENT_VALUE, DIFFERENT_DIAGNOSTIC e EXECUTION_UNKNOWN. Resultado igual com
diagnóstico diferente não é MATCH integral; registrar os canais separadamente.
Rejeição CLI esperada pode ser diferença observável, desde que o transcript e
controles provem essa causa. Não aceitar genericamente exit 2, crash ou truncamento.
Timeout, identidade errada ou observável obrigatório ausente são Unknown.

## 5. Suplementos funcionais e transversais separados

Reexecutar sentinelas e testemunhas dos fechamentos, sem inflar o denominador
do inventário. Deduplicar por identidade de caso explícita, preservando a relação
entre origem histórica, perfis e expectativas; igualdade com vanilla e preservação
do contrato anterior são projeções diferentes.

- Reconciliar cada fechamento P1323–P1334 com seu relatório e teste testemunha:
  confirmar o efeito alegado, residual aberto e eventual regressão. Não presumir
  que toda a família ficou pronta porque uma mensagem foi corrigida.
- Retestar canais de warnings HTML/query, flags reais de query, namespaces,
  lookup de campos, nomes/identidade/repr e chamadas indiretas. O MATCH da projeção
  JSON/DOM de P1322 não quitava hints nem stderr: conferir ambos desde o início.
- Reexecutar as testemunhas calc.abs preservadas no freeze P1334: casos válidos,
  tipos rejeitados, overflow, comprimento misto, precedência, missing/named/sobra,
  alias/import/With/Args/spread e origem entre arquivos. Expectativa histórica
  preservada não recebe automaticamente rótulo MATCH vanilla.
- Reexaminar as diferenças remanescentes P1334, incluindo resolução math de abs
  importado, parser do mínimo inteiro, fração infinita, path(), sqrt e usos de
  função em gradient/show/where. Medir as causas, sem reuni-las pelo nome calc.
- Manter sentinelas anteriores de CSV/read/decoders/encoders, Symbol, I/O,
  arquivo UTF-8, origem construída/capturada em closure e fronteira math. Confirmar
  ausências bilaterais como csv.encode antes de chamá-las de membros faltantes.
- Executar amostra transversal de eval, conteúdo/morfologia, layout, PDF e HTML.
  Declarar projeção, gates e limites de cada observável; comparar canais completos
  em paralelo à projeção. Inspeção visual PDF usa sua skill própria.

Copiar fixtures para namespace novo. Em comparações históricas com mudança de
localização, controlar raiz/caminho interno e externo antes de alegar regressão;
não normalizar o caminho real do diagnóstico para escondê-la. Bytes de PDF,
nome interno de fonte ou estrutura Rust diferentes não bastam para dívida de
linguagem. A amostra transversal não certifica layout/export global.

## 6. Causa, fechamento e dívida

Cada divergência candidata a decisão exige testemunha mínima reproduzível,
perfil, fontes/saídas bilaterais, file:line atual, L0/consumer proprietário,
hipótese causal e condição de refutação. Ler integralmente L0s necessários e
ADRs aplicáveis antes de propor arquitetura. Conferir ownership 1:1, Núcleos/pins
e provável gate ADR-0127, sem alterar nenhum deles.

Aplicar ADR-0107/0108: semântica/sintaxe/morfologia versus mecânica; medição antes
da classificação; intenção normativa distinta de comportamento observado.
Uma promessa de diagnóstico vanilla pode ser contrariada pela omissão de hints
mesmo que o L0 não transcreva cada hint. Não enfraquecer o contrato para manter
uma seleção cômoda, como a revisão final de P1322 precisou corrigir.

Publicar transições novas sem reescrever ledgers antigos: fechamento confirmado,
residual, regressão, descoberta em input novo, extensão fundamentada, feature
gated ou não resolvido. MATCH antigo que diverge agora só vira regressão depois
de isolar adapter/fixture/ambiente. Presença/kind/repr não provam comportamento
completo. Extensão autorizada e diferença intencional não são sinônimos de paridade.

Dívida de certificação/adversarial fica em ledger separado. Revalidar referências
históricas sem executar mutantes de produto. Ataques ao auditor não quitam essa
dívida; ausência de ataques não demonstra erro da linguagem.

## 7. Métricas e escolha sem vencedor antecipado

Publicar numerador, denominador, fórmula e recibo/estado de fonte de cada número.
Separar células, probes, paths, causas, perfis e repetições. Calcular igualdade
bruta e ajuste apenas por extensões fundamentadas em L0 atual. Unknown permanece
explícito, sem exclusão silenciosa. Não publicar percentual de toda a linguagem.

Comparar coortes por causa demonstrada com a prioridade usada em P1322:
regressão reproduzível; contradição L0 em rota canônica; diagnóstico isolado;
valor/kind/identidade/repr; membro ausente com carriers existentes; membro que
exige contrato/entidade/fase nova; restante sem rota canônica demonstrada.

Desempatar por menos owners necessários ao observável completo, mais paths com
a mesma causa provada, menor superfície de regressão demonstrada e ID lexical.
Risco desconhecido não é baixo. Não omitir transporte de origem, trace ou fachada
para reduzir artificialmente o número de owners.

Produzir tabela de coortes com testemunhas, impacto, owners, gates e justificativa.
Escolher exatamente uma recomendação para P1336, se elegível. Não escrever nem
implementar P1336. A seleção não autoriza seu contrato futuro. Unknown obrigatório,
instabilidade ou integridade inválida bloqueiam fechamento/seleção, mas permitem
publicar achados parciais válidos; não inventar vencedor para encerrar.

## 8. Revisão e execução do método

Na execução, ler a skill tekt-materializacao-segregada e usar papéis separados
para inventário, operação bilateral, classificação e revisão. O autor do ledger
não escreve seu próprio veredito de contagens/seleção. Agentes podem atuar em
paralelo ou sequência conforme capacidades e limite de concorrência. Sem
atestação técnica de isolamento, declarar a limitação; nomes de papéis não são selo.
Na redação deste plano não se executa esse regime nem se alega independência.

Calibrar o auditor com controles válidos e ataques em cópias dos próprios dados:
omissão/duplicação de ID, expansão Symbol fictícia, binário/perfil trocado,
flags ausentes, stderr/hints removidos, Unknown convertido em MATCH, fechamento
só por presença, perda de regressão, suplemento no denominador, extensão sem L0
e prioridade/owners ignorados. Congelar expectativas dos ataques antes de ajustar
o verificador; registrar somente os ataques executados e sua rejeição demonstrada.

Falha no runner/leitor/classificador exige evidência sucessora, sem sobrescrever
o original nem adaptar o oráculo ao produto. Validar focalmente antes de repetir
toda a matriz. Duas revisões sem ganho na mesma causa exigem rever o método.
Não copiar score histórico como resultado novo ou mutation score do produto.

## 9. Artefatos e término

Entregas novas com prefixo p1335 em diagnósticos:

- baseline/manifesto, recibo de build e inventários bilaterais;
- catálogo/reconciliação, matrizes normal/repeat/reverse e suplementos;
- ledger causal/transições, dívida de certificação e tabela/seleção de coortes;
- calibração/ataques do auditor, revisões e fechamento verificável;
- `p1335-o-que-falta-para-paridade.md`, com conclusões substantivas e recibos.

Executar e registrar build/testes workspace release --locked em target dedicado,
fmt --check, crystalline-lint, V5/V15/V26 estrito e git diff --check. Conferir
produto/L0 byte-idênticos ao baseline real, não necessariamente ao HEAD.
Explicitar avisos, testes ignorados e falhas preexistentes, sem corrigi-los nem
ocultá-los para aprovar a análise. Gates anteriores são antecedentes, não execução nova.

O relatório responde nesta ordem: o que falta; fechamentos confirmados;
regressões; cobertura/limites; próximo lote e por quê. Pode concluir auditoria
válida com recomendação, válida sem coorte elegível, ou bloqueada com achados
parciais. Descobrir regressão de produto não é defeito do auditor. Nenhum
resultado de execução é alegado pela redação deste passo.

# Passo 1322 — análise global de paridade após P1310–P1321

**Estado:** pronto para execução; somente o plano foi escrito.

**Natureza:** auditoria do produto, sem implementar correções ou alterar L0.

**Referência de método:** `00_nucleo/materialization/typst-passo-1309.md`,
explicitamente indicado pelo dono. Não consultar outros passos ou a pasta
context por iniciativa própria.

## 1. Pergunta que este passo deve responder

O que ainda falta para a paridade com o vanilla ratificado, o que realmente
foi fechado desde a análise global P1309 e qual deve ser o próximo lote?

Reenumerar a superfície pública e executar uma comparação global nova.
P1320 identificou lacunas e P1321 corrigiu um recorte de CSV; nenhum deles
substitui o inventário global. Não continuar automaticamente a sequência de
diagnósticos CSV nem escolher antecipadamente outra funcionalidade.

O relatório deve começar pelas conclusões de linguagem, testemunhas e efeitos
para o usuário. Procedimento, hashes e logs ficam em seções de evidência e
recibos vinculados, sem substituir a análise por uma lista de gates.

## 2. Escopo e limites de escrita

Não editar Rust, L0, Núcleos, Cargo, configuração, testes produtivos ou evidências
anteriores. Não executar mutações no produto, stage, commit, push ou limpeza.
Escrever somente este passo e artefatos novos `00_nucleo/diagnosticos/p1322-*`,
incluindo `p1322-fixtures/`, além dos temporários exclusivos registrados.

Ferramentas de auditoria podem ser criadas nesse namespace; não são consumers
produtivos nem legitimam uma alteração de linguagem. Corrigir um runner exige
recibo sucessor, sem sobrescrever a medição inválida ou mudar o oráculo para
aceitar o candidato. Usar apply_patch para editar os arquivos locais.

O passo recomenda uma coorte para P1323, mas não escreve nem implementa P1323.
Necessidade de mudança de produto vira diagnóstico e proposta de passo futuro.

## 3. Estado de partida — preservar a árvore real

Na redação, HEAD `d31047d7b8af7837c84adae4ded3d2ff50c62093`, com alterações
não commitadas P1319/P1321 e evidências P1320. Os pares alterados são:

- `00_nucleo/prompts/compiler/stdlib/loading.md` e
  `01_core/src/compiler/stdlib/loading.rs`;
- `00_nucleo/prompts/compiler/eval/call_dispatch.md` e
  `01_core/src/compiler/eval/call_dispatch.rs`.

Ao contrário da condição inicial histórica de P1309, não exigir árvore limpa,
não descartar mudanças e não fazer commit para viabilizar a análise. Congelar
o estado efetivo no começo da execução: HEAD, branch, UTC, status, staged,
diff HEAD integral/stat, arquivos untracked relevantes e hashes dos arquivos
de produto/L0. HEAD sozinho não identifica este baseline.

Compilar esse estado em target exclusivo, com Cargo --locked; registrar
comando, ambiente relevante, saídas, duração e SHA-256 do executável. O binário
P1321 serve de antecedente, não substitui a confirmação do estado atual.
Usar /dev/shm se houver espaço e permissão; senão /tmp, registrando o motivo.
Não usar hardlinks para cache mutável nem apagar temporários anteriores.

Vanilla permanece upstream/main **a51e02804**, sem resync. Confirmar
`/usr/local/bin/typst`, SHA-256 esperado
`7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8`.
O binário da raiz target é cristalino. Não usar --version como identidade.
Mudança externa do estado protegido durante a execução invalida a medição
afetada: preservar o recibo e recongelar antes de repetir.

## 4. Antecedentes verificáveis, não resultados atuais

Todos os paths desta tabela estão em `00_nucleo/diagnosticos/`.

| Entrada | SHA-256 na redação | Uso |
| --- | --- | --- |
| p1309-final-report.md | 84286f9a0649e441edf15ef38f13f10109224ce491eb478b3a8416371f71c74b | análise global histórica |
| p1309-probe-catalog.json | 20a5bdec9a848d24ebe1fcf451f143ce65499184e57984e4e5389aef5aecde3a | IDs e probes a reconciliar |
| p1309-owner-ledger.tsv | 36deab222babd8edbf90b030a56977482d28c13c6b15a55b01f9ea546c7f23f4 | classificação histórica |
| p1309-verification.json | b981a87a4f261a260000ea23d3871ed9a062bab6bb3dfa413c97cb1d9c6a9199 | veredito histórico |
| p1320-o-que-falta-para-paridade.md | 8ec8ad7a18d8a59150a8a7d5dedbc2b3c08a49ce86ddb860830c8e72b8a4c80c | lacunas focais e retificações |
| p1321-final-report.md | b5f04d62ddea0f1f7d8ec6656a86f14385101bb5762ad4794704de466681768d | fechamento delimitado de argumentos CSV |

P1309 observou 2182 probes, não apenas os 627 do catálogo predecessor. Esse
número pertence ao estado histórico identificado em seu relatório. Reconciliar
todos os IDs reais do catálogo pinado; não copiar suas percentagens ou supor
que presença/kind/repr equivalem à implementação completa de uma função.

Consultar diagnósticos de fechamento P1310–P1321 e os recibos que sustentam
cada alegação pertinente. Congelar os inputs usados antes das comparações.
Não ler JSONs extensos indiscriminadamente: alguns contêm cópias do produto e
não podem ser enviados a um papel com acesso restrito a oráculos.

## 5. Inventário novo e comparação bilateral

Reenumerar namespaces e rotas vanilla/cristalinas nos ambientes default/html,
com requisitos de feature, ancestors, kind e repr públicos. Sem instrumentar
ou alterar o produto. Informação que o método não observa deve ser marcada
como não medida, nunca presumida igual.

Produzir catálogo sucessor com união das rotas descobertas, controles negativos
e reconciliação de cada ID histórico: unchanged, added, removed, renamed,
split ou merged. Não apagar path porque seu ancestor bloqueia a chamada;
essa própria impossibilidade é observável. Justificar mudanças de catálogo
pela linguagem, não pelo nome semelhante ou pela conveniência do teste.

Executar o corpus congelado bilateralmente em default, html, a11y e html+a11y,
com argumentos de CLI confirmados em cada lado. Repetir a ordem canônica e
executar a ordem inversa; recompor por chave (probe_id, perfil), não pela ordem
de conclusão de processos. Concorrência limitada e registrada.

Guardar fontes/fixtures integrais, cwd, argv, exit, stdout/stderr, diagnósticos,
tempos e hashes. Não apagar warnings, hints ou traces para fabricar igualdade.
Separar MATCH_VALUE, MATCH_DIAGNOSTIC, CRYSTALLINE_ONLY, VANILLA_ONLY,
DIFFERENT_VALUE, DIFFERENT_DIAGNOSTIC e EXECUTION_UNKNOWN.

Timeout, crash, binário/fixture errado, saída não interpretável ou observável
obrigatório ausente são Unknown, não divergência de linguagem confirmada.
Unknown obrigatório ou instabilidade bloqueiam fechamento e seleção, mas não
impedem publicar o diagnóstico parcial e as testemunhas válidas.

## 6. Sentinelas de comportamento — fora do denominador do inventário

Além dos probes de superfície, executar suplemento funcional separado:

- preservações P1307/P1308: encoders, dados integrais de repr, módulos,
  mensagens/hints/traces e transporte de argumentos;
- P1310/P1312: casts dos decoders e de read, valor/origem e chamadas indiretas;
- P1311: campos públicos de funções, identidade e aliases/With;
- CSV P1313–P1321: Bytes, opções/duplicatas, valores válidos, causas e posições
  de parsing, texto/binário, arquivos, precedência de validação e missing;
- positivo e negativo de fonte/named/excesso, chamadas diretas, alias, With,
  Args, spread/sink/map e origens distintas; comparar erro completo;
- lacunas P1320: origem da string em closure, import no modo eval e controles
  de escape, Symbol em fonte/delimiter, diagnóstico externo UTF-8 válido,
  envelope de I/O e array(Bytes);
- fronteira math: chamada CSV direta versus alias/With; a diferença de
  namespace antes do consumer não é a mesma causa do validador CSV;
- presença/ausência de csv.encode, xml.encode e read.encode nos dois lados:
  ausência bilateral não é funcionalidade faltante;
- sentinelas transversais de eval, content/morfologia, layout, PDF e HTML,
  reaproveitando fontes apropriadas da auditoria P1320 em cópias novas.

Separar preservação do contrato anterior de igualdade vanilla. Um delta
normativo Symbol preservado não passa a ser paridade. Conferir também casos
válidos: corrigir a frase de erro não fecha uma funcionalidade ainda rejeitada.

Nos outputs paginados/exportados, medir o observável escolhido e declarar
limites: bytes diferentes, nome interno de fonte ou estrutura Rust distintos
não bastam para dívida de língua. Não chamar pequena matriz transversal de
cobertura global de layout/export. Inspeção visual de PDF segue sua skill própria.

## 7. Classificação causal e reconciliação

Cada divergência precisa de testemunha reproduzível, perfil, fonte/saída de
ambos os lados, file:line atual, L0/consumer proprietário, hipótese causal e
condição de refutação. Ler integralmente os L0 necessários antes de propor
arquitetura. Separar semântica/sintaxe/morfologia de mecânica (ADR-0107), medir
antes da decisão e distinguir intenção documentada de comportamento observado
(ADR-0108). Confirmar ownership 1:1, Núcleos/pins e provável gate ADR-0127.

Classificar regressão nova, membro ausente, valor/repr/kind incorreto,
diagnóstico divergente, contradição L0, extensão autorizada, feature gated,
fechamento confirmado ou não resolvido. Não herdar a intenção das extensões
calc de P1309 sem fundamento normativo atual.

Para cada fechamento P1310–P1321, registrar o que a medição nova confirma,
o que permanece aberto e eventual regressão. Em especial, separar o item
misto de P1320 sobre I/O/missing/unknown: P1321 não quitou I/O ao corrigir
argumentos. Nenhum fechado histórico é permanente sem nova decisão.

MATCH histórico que deixou de coincidir é candidato a regressão; diferença
de adapter, fixture ou ambiente precisa ser isolada antes de atribuir causa
ao produto. Não reescrever ledgers históricos: publicar transições sucessoras.

Dívida de certificação/adversarial fica em ledger separado. Confirmar o estado
das famílias históricas referenciadas por P1309, sem executar mutantes de produto
neste passo. Falta de ataque não é falha de linguagem; igualdade no corpus
também não quita ataques pendentes.

## 8. Métricas e seleção do próximo lote

Publicar numerador, denominador, fórmula, estado de fonte e recibo para cada
número: igualdade bruta de células, igualdade ajustada apenas por extensões
documentadas e cobertura de paths. Unknown é contado separadamente e nunca
desaparece silenciosamente do denominador. Repetições não multiplicam features;
suplementos funcionais não inflam o catálogo. Não publicar “paridade total”.

Comparar coortes por causa demonstrada, não “todos os loaders” ou “todo CSV”.
Manter a regra de prioridade explícita de P1309 para permitir comparação:
regressão reproduzível; contradição L0 em rota canônica; diagnóstico isolado;
valor/kind/identidade/repr; membro ausente com carriers existentes; membro
ausente com contrato/entidade/fase nova; restante sem rota canônica demonstrada.

Desempate: menos owners necessários ao observável completo, mais paths com
a mesma causa comprovada, menor superfície de regressão demonstrada e ID
canônico lexical. Risco desconhecido não conta como risco baixo. Não reduzir
artificialmente owners omitindo transporte de origem, como a R1 de P1321 mostrou.

Publicar tabela das coortes, testemunhas, impacto de linguagem, owners, gates
e motivo de prioridade. Escolher exatamente uma recomendação para P1323, sem
implementá-la. A seleção não autoriza o contrato futuro. Se faltar observação
obrigatória ou nenhuma coorte for elegível, explicar e não inventar vencedor.

## 9. Revisão da análise e controles de integridade

Na execução, usar papéis separados para inventário, operação bilateral,
classificação e revisão; auditoria de contagens/seleção não deve ser escrita
pelo autor do ledger julgado. Agentes podem ser usados sequencialmente,
respeitando capacidades, contexto e limite de concorrência. Nenhum papel
controla sozinho catálogo, interpretação e veredito. Ler a skill de
materialização segregada ao executar esse regime; sem atestação técnica de
isolamento, declarar essa limitação, sem selo implícito pelo nome do papel.

Revisar o auditor com controles válidos e ataques em cópias de seus próprios
dados: omissão de ID/membro, troca de binário/perfil, remoção de stderr,
Unknown convertido em MATCH, fechamento apenas por presença, captura nominal
de identidade, perda de regressão, suplemento inflando denominador, extensão
sem L0 e seleção que ignora causa/owners/prioridade. Não mutar produto real.

Congelar ataques/resultado esperado antes de ajustar o verificador. Registrar
os que foram realmente executados e rejeitados, com testemunha; não copiar o
score ou o número de ataques P1309 como resultado atual. Falha no próprio
auditor exige correção focal e nova revisão antes de repetir a matriz completa.
Duas revisões sem ganho na mesma causa exigem rever método/observabilidade.

## 10. Entrega e término

Artefatos mínimos, todos com prefixo p1322 em diagnósticos:

- baseline/manifesto e inventários dos dois lados;
- catálogo/reconciliação, matrizes normal/repeat/reverse e suplemento;
- ledger causal/transições, dívida de certificação e tabela de coortes;
- seleção, controles/ataques do auditor e verificação independente;
- `p1322-o-que-falta-para-paridade.md`, com resumo substantivo e links aos recibos.

Rodar e registrar build/testes workspace release com --locked em target
dedicado, fmt --check, crystalline-lint, V5/V15/V26 e git diff --check.
Verificar produto/L0 byte-idênticos ao baseline real, não necessariamente
iguais ao HEAD. Avisos e testes ignorados devem permanecer explícitos.
Falhas preexistentes não são corrigidas nem omitidas para aprovar a auditoria.

O relatório final responde, nesta ordem: o que falta; o que foi confirmado
fechado; regressões; cobertura e limites da evidência; próximo lote e por quê.
Pode concluir auditoria válida com coorte selecionada, válida sem coorte,
ou bloqueada por Unknown/instabilidade/integridade/gates. Uma regressão de
produto deve ser relatada como tal, sem confundir descoberta válida com defeito
do auditor. Nenhum resultado de execução é alegado pela redação deste passo.

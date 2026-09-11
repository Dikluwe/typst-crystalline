# P1339 — revisão 3: fronteiras causais auditadas antes do selo

Autor `/root/p1316_review`, apenas contrato, sem leitura de candidato. Regime
completo executado sem atestação de isolamento. Manifesto de autoridade r2,
freeze dos 31 L0 e antecedente P1338 não mudam. R1 e R2 ficam intactos.

## Predecessor e custo honesto

R2 `p1339-contract-r2.json`, SHA-256
`fb10967e8c22e5350a8c864b874cf282b5011c6dbc14b5c3eacc4f103b6aab71`,
resolve a circularidade de fase, mas foi emitido antes de concluir a auditoria
da origem de erros nos controles mistos. Preservamos esse arquivo recebido
pelo verificador e publicamos revisão 3; não o renomeamos para esconder custo.
É a terceira revisão do budget máximo de três. Este autor não executou
produtos/mutantes nem corpus completo. Não há selo ou candidato. Não se
alega ganho empírico; qualquer nova insuficiência exige redesenho explícito
do protocolo/budget, não quarta revisão disfarçada nem redução de gates.

## Medição: programas e ordem, não rótulos

`compiler/stdlib/foundations/selector.md:99-101` conserva disponibilidade e
diagnósticos de funções nuas. `foundations/query.md:102-119` aceita a nova
folha Element e preserva as formas antigas. A autorização de ocorrência
Strong/Emph não promove `query(strong)`, `counter(strong)` ou
`selector(strong)` nus. `infra/pipeline.md:785-801` mantém a passagem
ordinária e erros que impedem alcançar demanda Element.

O arquivo pinado `p1339-where-occurrence-probe-runs.json` contém dez fontes
de documento cuja primeira projeção consultada usa `query(strong)`, antes
das consultas `where`. O caso restante, `constructor_projection`, é eval
do produtor cru. Quatro fontes de
`p1339-where-integration-probe-supplement-runs.json` também consultam
query/counter Strong/Emph nu antes da forma filtrada. Seus nomes contendo
"filtered" não alteram a ordem do programa.

Logo, preservar o resultado integral desses ancestors é coerente com o L0;
exigir seu resultado vanilla integral exigiria reparar um caminho nu fora
do recorte. Para demonstrar W02, novos casos derivados devem substituir
explicitamente essas consultas por seletores `.where()` autorizados, com
IDs/fontes, referências vanilla e observações baseline novos. Isso muda o
grafo de leituras: não é uma ponte de transporte nem permite reutilizar
resultados antigos como se o programa fosse igual. Os ancestors originais
e seus canais permanecem, junto dos novos casos.

Nos controles `set_strong_delta` e `explicit_delta`, o produtor usa opções
fora do recorte; é preservado separadamente no baseline. O consumidor
filtrado obrigatório usa produtor suportado/default, conservando a distinção
entre novo snapshot consultado e projeção crua do constructor. No caso
`text_style_controls`, estilo visual isolado não cria ocorrência; nó Strong
próprio envolto em Styled tem somente sua ocorrência autorizada. Nada disso
libera parser de seletor nu nem constructor/set delta.

Contraprova à regra simplista "todo ancestor mantém saída inteira":
`p1339-stabilization-boundaries-runs.json`, caso `unrelated_error`, primeiro
constrói `counter(heading.where())` e depois contém o panic contextual.
Ausência/rejeição do `where` vazio está dentro do que P1339 deve corrigir.
Não se exige preservar esse primeiro erro. O predicado final preserva o
panic legado não selecionado, após a construção autorizada funcionar,
com sua posição/ordem e sem tornar esse bloco reexecutável por efeito de
um irmão filtrado. Exigir baseline integral aqui congelaria ausência nova.
Uma fixture independente sem a construção nova comprova separadamente a
semântica legada do panic. Referência fresca do ancestor demonstra o delta
autorizado; a medição baseline antiga continua histórica, não é reescrita.

Os quatro controles de `p1339-context-dependency-probe-runs.json`
plain_set_control/context_set_forward/context_set_final_before/stable_error_control
usam counter de string, sem `.where`; suas saídas inteiras continuam baseline.
Os casos interleaved_callbacks/counter_get com counter(heading) mantêm a
projeção legada e exercem separadamente as novas chaves Element: um erro
de ausência de Element não se torna uma obrigação de preservação.

## Fases e opacidade mantidas

Retém-se integralmente a separação de fase R2. Antes do selo: mapa completo,
fixtures/predicados/testes/harness congelados, todos os observáveis devidos
em referência/baseline e vinte mutantes reais, incluindo M12/M20. Depois
do candidato: execução compilada de todos os testes internos F01–F10 e
auditoria do grafo/owners reais. NotDue é só agendamento prévio dos testes
finais, sem crédito; não é classificação que substitui Unknown. Ausência
de observação devida, antes ou depois, bloqueia sua fase.

`context_reads_valid_for == false` não prova Unproven. A prova interna exige
observar a variante real do comparador privado em teste local ao owner e
auditar o acesso; a prova pública distinta exige não certificar reutilização.
Não se adiciona API pública, segundo comparador ou interpretação heurística
de mensagens para produzir o resultado desejado.

A interface prospectiva `p1339-mutant-closed-state-interface.md`, SHA-256
`7ff05129bd9213ffbe1a9c72b403262559834c782bb9f10928ea7d20bfd02f8d`,
permite construir carriers reais e chamar os três métodos públicos, mas
não observa sozinha a relação privada nem toda retenção/lifecycle. O mapa
pré-selo deve incluir portas simbólicas locais a testes para esses eixos.
Seus inputs, DTOs observados, assertions e pontos semânticos são congelados;
os nomes privados Rust finais não são presumidos nem viram API pública.

Depois do candidato, o implementador fornece a ligação como artefato
candidato com hash próprio. Corpos só delegam ao caminho produtivo real e
projetam dados/variantes sem perda; hooks cfg(test), se necessários, apenas
registram transições realmente tomadas. Não simulam tentativas, geram
identidades arbitrárias, decidem igualdade ou fabricam eventos a partir da
fixture. O verificador confirma o grafo até a operação usada pela compilação
ordinária, os pontos de observação, as configurações e execução real.

Portas mínimas: resultado privado Same/Different/Unproven; identidade causal
de produtor/registro/saída retida e invalidação de descendentes; transcript
real de tentativa/leitura/snapshot/documento/erro/sink. Cada uma precisa de
fixture concreta e predicado independente antes do selo. O nome da porta
não conta como cobertura, nem um cenário monolítico cujo adapter implemente
a política que deveria apenas observar. Mudança de input/DTO/predicado
congelado invalida o selo; resolver símbolo é a única informação tardia.

O suplemento `p1339-closed-harness-authority.json`, SHA-256
`08d0be7f19894c111130d0c791df6804cd1e7d89d2b8d6f418702be88af2125d`,
atribui tradução mecânica ao adversário, mantendo fixtures/predicados sob
autoria independente. Deve ser pinado pelo selo com o harness completo.
Não permite escrever algoritmo futuro, modificar expectativas ou usar
JSON simulado. Ligações futuras só resolvem símbolos/configuração do teste
congelado; qualquer acesso ao candidato permanece limitado pelas autoridades
declaradas, e a inspeção final é do verificador. O autor do contrato não
constrói nem atesta esse harness.

## Gates e refutações

Antes do selo, cada ancestor acima precisa de controle integral conforme
a política causal explícita, cada dimensão W02 de novo teste filtrado e
cada produtor fora do recorte de controle independente. Resultados não se
compensam: os três conjuntos são conjuntivos. Uma dimensão não observada
ou teste ainda indefinido impede selo. Se uma fronteira de produtor exigir
mudança fora do L0, registrar insuficiência, sem transformar teste em scope-out.

Todos os predicados W01–W10, as dez rotas, 539 IDs históricos, vinte famílias,
quatro perfis, ordens, matrizes internas finais e gates finais são preservados.
Somente a atribuição causal dos controles é corrigida e o suplemento de
autoridade é pinado. Não há redução de cobertura final, aprovação de candidato,
resultado discriminatório, RED/GREEN ou alegação de isolamento técnico.

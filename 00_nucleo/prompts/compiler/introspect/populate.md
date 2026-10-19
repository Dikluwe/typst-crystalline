# L0 — introspect/populate
Hash do Código: f1784ff8

## Ownership e medição

Camada L1. Consumer produtivo único: `01_core/src/compiler/introspect/populate.rs`.
No estado `df72a6c4a701917ab56a7c6169764a92ea775693` com árvore limpa, `01_core/src/compiler/introspect.rs:1376-1692` contém o comportamento que esta unidade recebe. A fonte e o estado estão registrados em `lab/results/construction/diagnostics/atomizacao-introspect-2026-09-30-before.json.gz`. A decisão é separá-lo por comportamento, sem alterar a API ou alegar ganho de desempenho.

## Contrato

A cada Tag::Start real ou derivada, atualizar índices, contadores, estados, citações e metadados na ordem causal. Callbacks Func permanecem para o pós-processador com runtime.

As invariantes de linguagem, a ordem causal e os limites de paridade continuam definidos em `00_nucleo/prompts/compiler/introspect.md`; este owner não cria uma segunda interpretação. O código usa chamadas estáticas na camada L1, sem vtable, I/O ou estado global mutável.

## Aceitação

Preservar os testes de introspecção e a suíte L1, compilar e passar `crystalline-lint .` com hashes coerentes. Mudanças semânticas futuras exigem medição de linguagem e atualização do L0 antes do código.


## Introspecção I1 — carriers e classificações nominais (2026-10-17)

### Medição anterior à decisão

HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado.
lab/results/construction/diagnostics/tekt-v16-introspect-carriers-2026-10-17/before.json.gz
preserva UTC/status/diff HEAD --stat, fontes/L0 e 2130 hashes conferidos do
fechamento anterior. contexts-before.json identifica os três V16 vigentes.
before-citation-probe.json conserva argv, rlib/programa com SHA-256, source
freeze e saída: nos 12 casos (quatro carriers × três posições), a ordem k2,k1
da árvore sem wrapper vira vazia; citações conhecidas são perdidas. A coleta
bibliography.rs:21–76 e a conversão em :79–220 não atravessam Source/ShowRecipe/
Located/Realized. Estes são carriers internos sem syntax vanilla; a prova é
metamórfica de transparência já exigida, não comparação de representação Rust.

labelled.rs:45–80 devolve ausência para Source, embora o walk encontre o alvo
interno. Hipótese: sua projeção de compatibilidade perde o texto do mesmo
Heading/Equation/Figure no mesmo Location; o teste RED deve confirmar antes de
alterar esse helper. Seu L0 proíbe desmontar Styled arbitrário; Source não é
Styled, e a decisão anterior é preservada. populate.rs:24–44 satura futuros
ElementPayload em None. O domínio atual tem 19 variantes; as condições bool
ativas devem continuar exigidas. before-probe-result.json registra 290 casos
simbólicos e zero E0004 ao acrescentar variantes fictícias aos dois enums:
proteção estrutural ausente, sem alegar falha numérica atual nem ganho de
leitura por IA. Payloads opacos e guard booleano não provam semântica de dados.

### Responsabilidade exclusiva

Somente no match automatic de populate_intr_from_tag_start, substituir o None genérico pelas 12 variantes não automáticas atuais e por Heading numbering false, Figure/Table is_counted false, Equation block false ou block true/numbering false: 17 alternativas. Conservar todo corpo ativo, cálculo de depth, action/custom key, ordem, stores e passes. Labelled e CounterUpdate não criam ocorrência automática extra; os caminhos específicos abaixo continuam intactos. Nenhum teste espelhado novo: controles simbólicos das combinações bool, replay e regressões reais existentes verificam o reparo nominal.

### Gate e aceitação

Fluxo interno/paridade ADR-0127: sem contrato público, default, fase, I/O,
registro dinâmico ou assinatura nova. L0 precede testes e código. RED funcional
antes da implementação; não usar falhas de montagem de sonda como RED. GREEN
exige as 12 ordens de citação iguais à base, visual de Realized sem contribuições
do original, desconhecidos preservados e fast path sem chaves sem reconstrução.
Label Source deve conservar a projeção direta e o Location/lang, enquanto
Styled e outros alvos continuam com os limites do contrato vigente.

O mutante descartável deve passar de zero para dois E0004 selecionados. Os
290 controles simbólicos mantêm a classificação, salvo os dois do helper de
label que agora reconhece Source. As 17 alternativas negativas de payload
incluem flags recusadas (Equation separa block false de block true/numbering
false); Source não é neutralizado como alvo desconhecido. Notas V19 permanecem
visíveis e justificadas por owner; sem V20 novo ou supressão.

Esperado no SARIF: remover só dois V16 (labelled/populate), acrescentar dois
V19 nominais e preservar o V16 de bibliography::collect_bib_keys até I2, com
os demais diagnósticos intactos por fonte/regra/severidade/mensagem. Não tratar
variantes estáticas ainda não percorridas como terminais legítimos. Replay
permite apenas os reparos prescritos e testes. Grafo V15/V26 antes do resselo
restrito aos três pares; build, testes focais/workspace, Clippy all-targets
-D warnings/-D unfulfilled_lint_expectations, Rustdoc estrito, fmt, linhagem
estrita/reversa, source freeze e diff --check. Pesados sequenciais sob
MemoryMax12GiB/High10GiB/swap0/jobs1. Sem commit/push/merge/release. O objetivo
global e as frentes anteriores continuam ativos; esta etapa não certifica
bibliografia ou referências universais.


## A1-S2 — leitura por objeto aprovada em 2026-10-17

### Medição anterior à decisão

HEAD b1b486f3d42431f8ea360bc5712637ee1e3d4f6c, working tree não commitado;
UTC/status/diffstat/fontes em before.json.gz e final-freeze.json.gz de
lab/results/construction/diagnostics/lote-a1-callbacks-reflexao-seletores-2026-10-17.
state-counterproof.json.gz registra RED bilateral: init 1/10 da mesma chave
após +1 exige 2/11; callback não lido não executa; atualização reutilizada
executa por ocorrência. Pin bd5ebec285505889e8ff1d4f68804ab147a699f3,
upstream introspection/state.rs:226–245, 419–455 e 477–520. O init do leitor
inicia a sequência; a ordem vem do documento, não do inteiro de Location.
O método atual trata Func como Set e o pós-passe eager perde essa semântica.
Estes são valores/erros de linguagem, não igualdade Rust ou algoritmo exigido.
A1-S1 foi retirada; gate A1-S2 confirmado pelo dono nesta sessão, revisão
lab/history/construction/records/00_nucleo/materialization/state-reader-runtime-2026-10-17/review.md,
SHA-256 6d6a176b8e27cc53c38fe6650c03c217448059f4e7287fd8848fe54e80247fa4.

### Responsabilidade exclusiva — supersede a fase histórica de State

Delegar cada payload StateUpdate ao registro privado de ações, em vez de
executar ou duplicar sua lógica Set/Func. Registrar inclusive Func sem init.
State legado ainda registra init; os índices, tags, labels e contadores mantêm
ordem e identidade. Walk não aplica callback e não avalia init de leitor.

### Aceitação e fronteiras

Gate A1-S2 ratificado para as cinco assinaturas e a fase de leitura. L0 primeiro,
RED funcional antes do código; GREEN contra o pin indicado para linguagem e
controles próprios das extensões Tekt. Não ampliar outros contratos públicos.
Build/workspace/doctests, Clippy/Rustdoc estritos, fmt, linhagem V1/V5/V15/V26 e
reversa; manter V16 sem aumento. MemoryMax12GiB/High8GiB/swap0/jobs1, tarefas
pesadas sequenciais. Medir consumo sem inferir ganho global só da representação.

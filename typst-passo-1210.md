# P1210 — rebaseline integral de paridade funcional pós-saneamento

**Estado:** ESCRITO — NÃO EXECUTADO  
**Data:** 2026-08-26  
**Dependência:** P1209 fechado como GREEN POR TRIAGEM  
**Baseline vanilla:** upstream/main ratificado `a51e02804`  
**Baseline cristalina ao redigir:** HEAD
`7fb5bb6d9ee73298af4fd09bb858cb26e5fdd568`, working tree não commitada,
706 ficheiros alterados (`737 insertions(+), 699 deletions(-)`), índice vazio.

## 1. Objetivo experimental

Medir novamente, de ponta a ponta, quanto da linguagem e do produto observável
do Typst vanilla ratificado é reproduzido pelo cristalino depois das séries
P1141–P1209.

A experiência mantém o **comportamento público do vanilla como variável de
controle** e muda a arquitetura interna: owners L0 1:1, Núcleos Tekt para
invariantes 1:N, camadas cristalinas, domínio puro em L1 e fronteiras
explícitas. O objetivo de longo prazo é verificar se, para a mesma linguagem e
os mesmos resultados observáveis, o cristalino é mais fácil de:

1. compreender e localizar por owner;
2. corrigir sem efeitos laterais;
3. testar e auditar;
4. estender sem quebrar comportamento existente;
5. manter com menor deriva entre especificação, código e testes.

P1210 mede a variável de controle — paridade funcional. Ele **não** conclui
sozinho que a arquitetura é mais manutenível; cria a linha de base necessária
para que passos posteriores comparem custo e risco de evolução sobre um
comportamento equivalente.

Paridade não significa copiar structs, algoritmos, proc-macros, ordem de
objetos PDF, bytes internos ou igualdade Rust do vanilla. Conforme ADR-0107,
significa equivalência de linguagem em semântica, sintaxe e morfologia, mais os
observáveis próprios de cada target.

## 2. Restrições do passo

- Não implementar bindings, membros, defaults ou layouts durante a medição.
- Não remover `EXTRA_BINDING` nem preencher `MISSING_MEMBER` mecanicamente.
- Não alterar contrato público, comportamento por defeito, fase do pipeline ou
  compatibilidade. Qualquer descoberta dessa classe vira passo próprio e gate
  ADR-0127.
- Não usar tag ou string de versão como oráculo. Confirmar o vanilla pelo hash
  pinado `a51e02804` e pelos binários ratificados.
- Não ler nem listar `00_nucleo/context/` ou
  `00_nucleo/materialization/`.
- Não tratar warnings V16–V21 já classificados em P1209 como falhas de
  paridade.
- Não produzir um único percentual que misture universos ou níveis distintos.

## 3. Congelar proveniência reproduzível

Antes da primeira medição, registrar:

1. HEAD cristalino, hora ISO-8601 e timezone;
2. `git status --short`, `git diff HEAD --stat`, digest SHA-256 do diff e estado
   do índice;
3. SHA-256 dos binários cristalino e vanilla usados;
4. confirmação da fonte vanilla `a51e02804`;
5. comandos, flags, features, fontes, locale, timezone e ferramentas externas;
6. número e lista exata dos casos de cada universo medido.

Como a working tree contém a migração documental acumulada, P1210 deve medir o
estado exato presente, sem atribuí-lo apenas ao HEAD. Se o estado mudar durante
as execuções, registrar nova hora, novo digest e separar a amostra; não fundir
números de estados diferentes.

## 4. Rebaseline A — superfície pública

Reexecutar `lab/surface-inventory` contra ambos os binários, sem reutilizar as
contagens P1140.26. Cobrir:

- bindings globais e feature-gated;
- módulos, tipos, funções e valores;
- membros estáticos e de instância;
- aliases e grandes famílias tabeladas, incluindo símbolos;
- kind público, `repr(type(...))`, assinatura e metadados que o harness consiga
  observar;
- superfície com e sem a feature HTML ratificada.

Produzir um JSON bruto e um TSV classificado com, no mínimo:

- `MATCH`;
- `MISSING_BINDING`;
- `MISSING_MEMBER`;
- `WRONG_KIND`;
- `WRONG_SIGNATURE`;
- `BEHAVIOR_DIVERGENCE`;
- `EXTRA_BINDING`;
- `UNVERIFIED_METADATA`;
- `HARNESS_LIMITATION`.

Para cada ausência ou extra, registrar owner candidato e separar:

- unidade funcional real;
- entrada de tabela/alias;
- membro condicionado por feature;
- metadado não observável pelo inventário;
- limitação ou falso positivo do harness.

Contagens não autorizam implementação. Uma lacuna só entra no backlog quando
uma sonda mínima prova que o programa Typst correspondente é válido no vanilla
e diverge no cristalino.

## 5. Rebaseline B — comportamento focal das frentes recentes

Reexecutar e ampliar as probes diferenciais de P1140 para confirmar o estado
final de:

- `path` e preservação da raiz virtual;
- `location` e identidade de conteúdo consultado;
- `page` e `page.numbering` por string/função;
- membros materializados em P1142–P1150;
- `Symbol` multi-codepoint e `emoji.heart`;
- módulo e construtores HTML materializados em P1165–P1178.

Cada probe deve registrar input, stdout, stderr, exit status e classificação no
nível da linguagem. Comparação byte-idêntica só é gate quando os bytes são o
observável público definido, como serialização ou mensagem de erro auditada.

## 6. Rebaseline C — pipeline P1–P4

Auditar primeiro o harness vigente em `lab/parity/` e confrontá-lo com
`00_nucleo/diagnosticos/typst-paridade-definicoes.md` e
`typst-paridade-plano-medicao.md`. Esses documentos são históricos e têm
status `PROPOSTO`; nenhum critério antigo prevalece sobre ADR-0107/0108 ou
sobre a implementação atual sem revalidação.

Medir universos separados:

| Nível | Observável | Gate primário |
|---|---|---|
| P1 — parse | sintaxe compactada | mesma árvore de linguagem, ignorando spans mecânicos |
| P2 — eval | valor, tipo, `repr`, erro e posição | mesma semântica e diagnóstico observável |
| P3 — layout | texto, morfologia, páginas, itens e geometria | relatório estratificado; nenhuma tolerância arbitrária fecha divergência |
| P4 — export | PDF e HTML observáveis | comparação normalizada/visual/semântica por target, nunca bytes PDF crus |

Executar dois corpora sem misturar denominadores:

1. **corpus cristalino declarado:** tudo que o projeto afirma suportar;
2. **corpus vanilla amplo:** amostra determinística da suíte ratificada,
   incluindo casos ainda não suportados.

Para cada nível, informar `passou / total / não executável / falha de harness`,
lista nominal das divergências e features envolvidas. Um caso só alcança Pn se
o harness conseguir chegar honestamente àquele nível; falha anterior não pode
ser contada como aprovação posterior.

## 7. Rebaseline D — eixos de produto

Criar matrizes específicas, sem esconder gaps num agregado:

- layout paginado: LTR, RTL/bidi, regiões, colunas, floats, footnotes,
  posicionamento e tipografia;
- math: semântica, morfologia e geometria;
- introspecção: query, counter, state, references, outline e location;
- recursos externos: fontes, imagens, bibliografia, packages, plugins e paths;
- PDF: texto, páginas, geometria, links, metadados, fontes e tagging;
- HTML: estrutura semântica, tags, atributos, escaping, whitespace, CSS,
  MathML e relações/posições quando públicas;
- CLI: targets, flags, defaults, stdout/stderr, exit status e mensagens.

Cada divergência recebe uma classe:

- `LANGUAGE-GAP`: sintaxe, semântica ou morfologia diferente;
- `PRODUCT-GAP`: target/CLI público diferente;
- `DIAGNOSTIC-GAP`: mensagem ou posição observável diferente;
- `MECHANICAL-DIVERGENCE`: implementação diferente sem efeito público;
- `FEATURE-GATED`: depende de feature explicitamente controlada;
- `HARNESS-GAP`: ainda não mensurável de modo honesto;
- `STALE-SCOPE-OUT`: L0 declara falta que a implementação já resolveu;
- `ADR0127-GATE`: correção exigiria contrato/default/fase/compatibilidade.

`MECHANICAL-DIVERGENCE` não conta contra paridade. `HARNESS-GAP` não conta
como sucesso nem como falha: permanece denominador separado a fechar.

## 8. Inventário dos scope-outs L0

Pesquisar os Prompts L0 vigentes por `scope-out`, “não suportado”, “aceite sem
efeito” e divergências declaradas. Para cada ocorrência:

1. ler integralmente o owner;
2. medir o observável no vanilla e no cristalino;
3. marcar `RESOLVIDO`, `LANGUAGE-GAP`, `MECHANICAL-DIVERGENCE`,
   `HARNESS-GAP` ou `ADR0127-GATE`;
4. apontar `file:line` do L0 e do consumer;
5. não corrigir texto ou código neste passo.

Isso evita tanto esconder dívida real em scope-out antigo quanto abrir trabalho
desnecessário para diferenças internas legítimas.

## 9. Saídas obrigatórias

Criar em `00_nucleo/diagnosticos/`:

- `typst-p1210-rebaseline-paridade-funcional.md` — relatório humano;
- `p1210-superficie-publica.json` — inventário bruto;
- `p1210-divergencias.tsv` — lista nominal classificada;
- `p1210-matriz-pipeline.tsv` — resultados P1–P4 por caso;
- `p1210-scope-outs.tsv` — auditoria dos L0s;
- artefato de proveniência com comandos, hashes e ambiente.

O relatório deve apresentar:

1. matrizes por nível e por target;
2. contagens reproduzíveis com denominadores explícitos;
3. divergências reais agrupadas por owner, dependência e gate;
4. limitações do harness;
5. mapa do caminho crítico;
6. clusters candidatos aos próximos passos, sem implementá-los;
7. uma linha de base separada para futura avaliação de manutenibilidade.

## 10. Linha de base para testar manutenibilidade

P1210 deve definir, sem inventar resultados, quais métricas serão acumuladas
nos futuros clusters de correção:

- número de owners L0 e consumers produtivos tocados;
- tamanho do diff funcional, excluindo resselo mecânico;
- testes RED necessários para localizar a causa;
- tempo entre RED reproduzível e GREEN;
- quantidade de regressões fora do owner;
- violações arquiteturais introduzidas e removidas;
- necessidade de mudanças em contrato, default ou fase;
- reincidência da mesma classe de defeito;
- extensão nova adicionada depois da paridade e número de owners afetados.

Essas métricas não devem ser comparadas diretamente com o histórico upstream
sem tarefas equivalentes e condições controladas. A conclusão forte “mais
fácil de manter e evoluir” exige experiências posteriores: corrigir ou ampliar
features equivalentes nos dois desenhos e comparar evidências, não apenas
linhas de código ou impressão subjetiva.

## 11. Gates de validação

P1210 fecha somente quando:

- todas as medições apontam para `a51e02804` e para um estado cristalino
  reproduzível;
- nenhum número decisório mistura estados, features ou denominadores;
- superfície, probes, P1–P4, targets e scope-outs possuem inventários nominais;
- toda divergência foi classificada como linguagem, produto, diagnóstico,
  mecânica, feature ou harness;
- o relatório não declara 100% enquanto houver `HARNESS-GAP` relevante;
- nenhuma implementação ou alteração de contrato foi feita;
- `cargo build --workspace --quiet`, testes do harness executável,
  `crystalline-lint .` e `git diff --check` terminam sem regressão causada pelo
  passo;
- o índice Git permanece vazio.

Se uma ferramenta ou nível não puder ser executado, registrar o bloqueio e
continuar os demais eixos. Resultado RED é dado da medição, não motivo para
interromper o rebaseline nem autorização para corrigir no mesmo passo.

## 12. Fechamento e próximos passos

Ao final, ordenar os clusters por:

1. falha em comportamento já declarado como suportado;
2. dependência que desbloqueia múltiplas features;
3. superfície pública ausente;
4. layout/export de alto impacto;
5. expansão opcional do produto.

Cada cluster posterior começa lendo o L0 proprietário, mede novamente a sonda
focal e segue L0 → RED → implementação → GREEN. Se tocar contrato público,
default, fase ou compatibilidade, escreve o L0 e para no gate ADR-0127.

P1210 não procura provar que o cristalino já venceu a experiência. Ele fixa a
mesma régua funcional do vanilla para que a comparação futura de manutenção e
evolução tenha uma variável de controle honesta.

# Prompt L0 — `wiring/tests/cli` — integração do binário typst
Hash do Código: f6ff401b

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/wiring/cli-observables.toml sha256:0e59744a924f0f6acbe7c4d20db9efc7caed4783c3b8d6b2c2c5a9cace340ece


**Camada**: L4
**Ficheiro alvo**: `04_wiring/tests/cli.rs`
**Criado em**: 2026-08-26 (P1198; individualização de `wiring.md`)
**ADRs**: ADR-0046, ADR-0048, ADR-0051, ADR-0126, ADR-0128, ADR-0129

---

## Medição antes da decisão

A suíte executa `CARGO_BIN_EXE_typst` como processo externo e observa status,
stdout, stderr e artefactos. Ela não implementa parsing, compilação, export ou
formatação. O antigo `wiring.md` era compartilhado com esta suíte, `main.rs` e
a suíte independente do linter, violando ownership 1:1.

## P1293.final — observação canónica do `repr` HTML multiline

### Medição anterior à decisão

O receipt bloqueante independente
`00_nucleo/diagnosticos/p1293-definitive-final-verification-blocker-receipt.json`
SHA-256 `6e658e3bb2293705755b70e5cf8a1cebdbac0ca51bc0fc3c665e5d3a23d72920`
mediu, em working tree não commitada sobre HEAD
`7dd25ff0e222b6c7c640d6bc7957b98f94227507`, `cargo test --workspace -q`:
core `5417/5417`, infra `918/918` e wiring CLI `64/71`; os sete REDs são
P1168 e P1173–P1178. O snapshot do receipt vai de
`2026-09-02T19:36:53.811303-03:00` a
`2026-09-02T19:48:37.865715-03:00`, com
`git diff HEAD --stat` SHA-256
`e3d170410960d31d1b9fb5aff31823c58f6e3623ff9b8fc686e1a8abb304f0db`.

O consumer medido, SHA-256
`cb19a63665aa5c66feae200a03ce4537097a4f25c67489dd457c03006031b288`,
mantém em `04_wiring/tests/cli.rs:1849-1867` buscas por
`elem(tag: ...)` no lote P1168 e, em `:2063-2079`, `:2108-2124`,
`:2153-2169`, `:2198-2214`, `:2243-2259` e `:2288-2304`, igualdades
monolinha para `abbr`, `mark`, `picture`, `summary`, `ruby` e `title`.
Cada teste preserva depois dessa asserção um controlo DOM próprio.

Execuções read-only do binário release no estado acima confirmaram a forma
vigente. Um `HtmlElem` com atributos e corpo é projetado como bloco `elem(`,
seguido, nessa ordem, por linhas indentadas `tag`, `attrs` e `body`, cada campo
com vírgula, e `)` final. O tuple P1168 conserva doze membros em ordem; formas
curtas cabem legitimamente numa linha, enquanto o membro `p` com `attrs`
projeta o mesmo bloco multiline. Os seis casos escalares preservam exatamente
tag, atributos ordenados e corpo medidos; nenhum termina por conversão para a
forma monolinha histórica.

Medição: os sete failures param nas expectativas textuais de `repr`; o receipt
regista os oráculos P1293 C-P08 em ambas as ordens, 51 focais, superfícies
default/HTML e DOMs produtivos como GREEN, sem witness DOM divergente.
Inferência: alinhar somente os observadores CLI à forma multiline vigente fecha
o stale-test sem mudar produto. Refutador: qualquer divergência de conteúdo,
ordem de membros/campos, tag, attrs, body, status, DOM posterior ou necessidade
de alterar produção exige parar e reabrir o owner causal correspondente.

### Obrigação test-only

P1168 deve comparar o `repr` completo do tuple canónico, preservando os doze
membros, sua ordem, os campos de cada `HtmlElem`, `body: none`, `body: [x]` e
`attrs: (id: "p", class: "a b", hidden: "")`. A expectativa reconhece
explicitamente o bloco multiline do membro `p`; não usa uma busca monolinha que
confunda ausência do tag com mudança de layout textual.

P1173–P1178 devem comparar exatamente o `repr` multiline canónico de `abbr`,
`mark`, `picture`, `summary`, `ruby` e `title`: abertura `elem(`, campos
indentados `tag`, `attrs`, `body` nessa ordem e fecho `)`. Conteúdo, valores,
ordem, escaping e morfologia permanecem os medidos. Não é permitido remover
linhas/indentação para aceitar ambas as formas, normalizar whitespace, voltar o
produto a monolinha ou afrouxar para buscas parciais que deixem de provar a
estrutura.

As partes compile/DOM dos sete testes, seus sources, outputs HTML, atributos,
escaping, conteúdo, ordem e expectativas ficam byte-conceitualmente
inalteradas. P1169–P1172 e todos os demais testes ficam fora desta retificação.
A única escrita posterior legitimada é a troca das sete expectativas/buscas de
`repr` no próprio consumer; nenhum produto, contrato/oráculo protegido ou L0
adjacente é autorizado.

Classificação ADR-0107/0108: a representação textual é o observável deste
`eval repr`; a medição precede a decisão e não transforma mecânica Rust em
contrato. ADR-0127: retificação test-only para o contrato C-P08 já confirmado,
sem API pública, default, fase ou compatibilidade nova; fluxo contínuo, sem gate
humano. ADR-0128 preserva HTML como target separado e ADR-0129 mantém este
Prompt como owner 1:1 exclusivo de `04_wiring/tests/cli.rs`.

## Responsabilidade

Validar a superfície observável de integração do binário `typst`:

- comandos, aliases, help e rejeições de argumentos;
- códigos de saída e disciplina stdout/stderr;
- criação e integridade mínima de PDF, PNG, SVG e HTML;
- warnings e errors formatados, inclusive spans entre ficheiros;
- resolução de root, font paths, inputs, certificate e document ID;
- serialização de `eval`, `query`, `fonts`, `info` e completions;
- materialização transacional de `init`;
- recompilação incremental e preservação do último artefacto em `watch`;
- gates de feature e famílias HTML já implementadas.

## Harness

O path do binário vem de `env!("CARGO_BIN_EXE_typst")`. Fixtures vivem no
diretório temporário, incluem o PID no nome e são removidas ao final. Testes de
watch encerram o child em `Drop` e esperam mudanças com timeout explícito.

A suíte usa apenas APIs de teste e processo; não chama funções privadas de L4
nem replica decisões internas do pipeline.

## Observáveis centrais

- Compile limpo: exit 0, stderr vazio quando não há warnings, artefacto válido
  e stdout vazio para destino em ficheiro.
- Warning: exit 0, artefacto produzido e mensagem em stderr.
- Erro semântico: exit 1, diagnóstico em stderr e nenhum falso sucesso.
- Argumento ou I/O inválido: exit 2.
- Warnings precedem errors quando ambos existem.
- PDF começa por magic header, termina em EOF e não é vazio.
- Flags explícitas vencem defaults e variáveis de ambiente conforme o
  contrato L2; a suíte verifica apenas o resultado do processo.

## Escopos especializados

Os testes P1137 cobrem a árvore de comandos e composição de compile, watch,
fonts, completions, info e init. P617 cobre IDs XMP. P772 cobre diagnóstico
cross-file. P819 cobre plugins. P866/P870 cobrem escolha de formato.
P1163 cobre valores Symbol em eval. P1166–P1178 cobrem gates e morfologia do
target HTML.

Esses números identificam regressões históricas; não autorizam a suíte a ser
owner das implementações produtivas correspondentes.

## Critérios de verificação

- `cargo test -p typst-wiring --test cli` passa integralmente.
- Nenhum teste importa diretamente módulos privados de `main.rs`.
- Fixtures não deixam child de watch vivo após o teste.
- A alteração de linhagem do P1198 não modifica corpos da suíte.

## P1295/W3 — histórico confirmado e materializado, não certificado

O gate ADR-0127 de W1/W2/W3/B1 foi confirmado em
`2026-09-02T22:57:37-03:00`, e a obrigação abaixo foi materializada. Ela
permanece como proveniência histórica e cobertura funcional P1137, não como
prova discriminatória vigente da ordem. P1295 não foi certificado: o receipt
`00_nucleo/diagnosticos/p1295-verification-receipt.json`, SHA-256
`06104b7d41f8e456e67df90c8036ba4362af67f3247880278bfa587e1d2c0573`,
é `BLOCKED` pela primeira suíte CLI integral vermelha durante recuperação.

### Medição anterior à decisão

No HEAD `76fb7336311bdb6497456ab5fdc0a8ce355ff39b`, o teste
`p1137_watch_dependencias_recuperacao_e_filtro` forma o diretório temporário
somente com PID em `04_wiring/tests/cli.rs:97`, chama `create_dir_all` sem
limpeza prévia em `:102` e usa a mera existência do output como primeira
condição em `:117`. O child é encapsulado para kill/wait em Drop (`:115`), mas
o teste não verifica que continue vivo durante cada fase. Seus timeouts
reportam apenas a condição genérica do helper, sem nomear publicação, asset ou
recuperação.

Medição: após a primeira publicação, o teste altera o irrelevante, espera 400
ms, altera o asset uma vez e exige bytes diferentes (`:120-128`); depois cria
erro transitório, preserva o segundo artefacto e recupera (`:130-137`). A race
vem de a publicação poder preceder a captura do baseline no produto, não de um
timeout curto no observador.

Classificação ADR-0107/0108: PID, polling e duração são mecânica do harness; a
recompilação por asset, o filtro de irrelevante e a preservação/recuperação são
observáveis do processo. Inferência: com W2, a primeira publicação nova prova
que o snapshot daquela iteração já foi capturado, permitindo uma única mudança
do asset sem sleep de prontidão. Refutador: se a publicação puder ocorrer antes
do snapshot na ordem confirmada de W2, esse sinal é inválido e o contrato deve
voltar ao gate.

### Obrigação test-only

O teste P1137 de watch deve:

1. remover recursivamente, em best-effort, o diretório temporário P1137
   calculado antes de recriá-lo, eliminando resíduo de execução abortada ou PID
   reutilizado;
2. remover qualquer output residual como consequência dessa limpeza e tratar a
   primeira publicação nova como sinal de armamento conforme W2;
3. verificar, por `try_wait`, que o child ainda está vivo antes e durante as
   fases relevantes; saída antecipada deve falhar com fase e status;
4. usar diagnósticos de timeout que identifiquem pelo menos `publicação
   inicial`, `recompilação por asset` e `recuperação`;
5. alterar o asset exatamente uma vez após a publicação e exigir uma nova
   versão do artefacto;
6. preservar a alteração do ficheiro irrelevante e provar que ela não muda o
   artefacto;
7. preservar o erro transitório, o último artefacto válido e a recompilação de
   recuperação;
8. continuar a encerrar e aguardar o child em todos os caminhos via guarda RAII
   e remover a fixture ao final.

É proibido corrigir a race aumentando timeout, acrescentando sleep de
prontidão, repetindo até passar ou reclassificando falha como flake. O timeout
existente pode permanecer como limite de falha, não como mecanismo de
sincronização.

### Aceitação

- uma execução focal deve falhar deterministicamente contra o produto sem W1/W2
  por ausência do contrato/API ou testemunha específica, nunca apenas por
  timeout aleatório;
- após materialização selada, o teste preserva a sequência publicação → asset
  → irrelevante → erro → último artefacto → recuperação;
- vinte execuções isoladas consecutivas devem ser verdes e registrar duração
  individual e total no recibo, sem loop até passar;
- a suíte CLI integral deve passar duas vezes e qualquer falha permanece no
  recibo.

### Estado do gate P1295

O gate ocorreu e autorizou a materialização. O RED posterior permanece e não é
apagado pelo stress focal `20/20` nem pela suíte integral verde que o sucedeu.
W3 conserva cobertura funcional, mas não é reutilizado como prova suficiente
da ordem no redesenho P1297/R1.

## P1296/O1 — histórico refutado e supersedido por P1297/R1

O conteúdo abaixo registra a hipótese test-only efetivamente tentada em P1296;
não é obrigação ativa nem autorização para nova revisão do sentinel.

### Medição anterior à decisão

O recibo independente P1295
`00_nucleo/diagnosticos/p1295-verification-receipt.json`, SHA-256
`06104b7d41f8e456e67df90c8036ba4362af67f3247880278bfa587e1d2c0573`,
registra a primeira suíte CLI integral com exit `101`, `70/71` e timeout de
20 segundos exclusivamente em `p1137_watch_dependencias_recuperacao_e_filtro`
durante `recuperação`. O stress focal anterior passou `20/20`; o verde posterior
`71/71` não absolve o RED.

No consumer congelado, `04_wiring/tests/cli.rs:166-170` escreve o source
inválido, usa `sleep(500 ms)` e observa apenas liveness + preservação do output;
em `:172` escreve `Recovered`. Não existe prova de que a iteração de erro já
terminou e capturou seu baseline antes dessa segunda escrita.

No owner produtivo congelado, `04_wiring/src/main.rs:95-101` termina a
compilação, normaliza dependências e captura o snapshot; no caminho de erro,
somente depois `:114-115` remove staging. Portanto a recuperação pode ser
incorporada ao baseline se ocorrer antes de `:101`. A inferência de prontidão
por 500 ms é refutada pelo RED sob carga da suíte.

Classificação ADR-0107/0108: PID, staging e remoção são mecânica do harness; a
mecânica é o observável causal nesta verificação de processo. Nenhuma semântica
Typst, output, API ou ordem produtiva muda. Refutadores: staging escrito numa
compilação com erro, descarte anterior ao snapshot ou path não derivável de
destination + child PID; qualquer um bloqueia a correção test-only.

### Obrigação test-only histórica

O contrato histórico fazia P1137 derivar o staging exato
`.<output-file-name>.crystalline-watch-<child-pid>.tmp`. Antes de escrever o
source inválido, cria nesse path um sentinel único. Em seguida:

1. escreve o source inválido uma vez;
2. aguarda a remoção do sentinel com o timeout existente de 20 segundos,
   diagnóstico `armamento após erro transitório` e `try_wait` em cada poll;
3. usa a remoção como prova de que `snapshot` já ocorreu, pois o L0 produtivo
   fixa `snapshot -> discard_output`;
4. confirma que o segundo artefato publicado permaneceu byte a byte;
5. escreve `Recovered` uma vez e aguarda nova publicação;
6. remove o `sleep(500 ms)` sem adicionar outro sleep de prontidão, timeout,
   retry, alteração corretiva ou loop-until-pass.

O sentinel vive somente na fixture temporária e é removido pelo produto ou
pela limpeza RAII. Asset continua alterado uma única vez; ficheiro irrelevante,
último artefato, liveness, encerramento do child e limpeza permanecem.

### Aceitação e fronteira históricas

- o controle P1295 passa sem qualquer alteração de L3/L4;
- mutante que descarta staging antes do snapshot falha na recuperação;
- mutante que omite descarte falha na fase de armamento;
- observador temporal antigo falha sob atraso adversarial que expõe a janela;
- P1137 passa duas vezes, depois `20/20` isolado e duas suítes CLI integrais;
- uma falha continua bloqueante e nenhum verde posterior a absolve.

ADR-0127: correção test-only em fluxo contínuo. Qualquer necessidade de alterar
produto, API pública, comportamento default ou fase de pipeline refuta esta
classificação e exige novo gate.

### Refutação registrada

Na revisão 1, o controle passou, MO2 e MO3 foram mortos, mas MO1 —
`discard -> snapshot` — sobreviveu. O observer consultava a remoção a cada
50 ms; o mutante podia remover o sentinel e capturar antes do poll seguinte,
portanto a escrita posterior de `Recovered` ainda ocorria depois da captura.
Score: `2/3 = 0.6666666666666666`, um survivor e zero `Unknown`. O refutador
exigido — o focal rejeitar MO1 mínimo sem timeout maior, sleep, retry ou sinal
adicional — não foi satisfeito.

Na revisão 2, uma dependência FIFO e barreiras test-only fizeram o primeiro
controle expirar durante `recompilação por asset` após 20 s. MO1 não foi
executado porque o positivo já havia regredido. O consumer foi restaurado byte
a byte ao SHA-256
`addf970325c6b9df11482b6d7f28bd1099cbb0d225080044268e3dc3a304206b`.
O receipt final
`00_nucleo/diagnosticos/p1296-test-receipt.json`, SHA-256
`be8aaf3537bb068aab8eff1c5b656fe74c56cec8bd144f22e48a12b486d3e8a0`,
registra duas revisões consumidas, zero ganho na segunda e nenhuma revisão
local adicional autorizada.

Conclusão: a remoção do sentinel é evidência histórica insuficiente para
discriminar a inversão. P1296/O1 está encerrado e supersedido; não define a
prova determinística vigente.

## P1297/R1 — fronteira da suíte CLI durante o gate

### Medição anterior à decisão

P1137 continua sendo o teste externo do processo para publicação inicial,
recompilação por asset observado, filtro de ficheiro irrelevante, erro
transitório, preservação do último artefacto, recuperação, liveness do child e
limpeza RAII. A campanha P1296 demonstrou, porém, que esse conjunto de
observáveis não prova sozinho se captura ou descarte ocorreu primeiro.

Classificação ADR-0107/0108: status, artefactos e liveness são observáveis
funcionais adequados ao owner CLI; a ordem causal interna não se torna
discriminável pela ausência posteriormente observada do staging. Inferência:
P1137 deve continuar cobrindo comportamento do processo sem reivindicar prova
determinística da inversão. Refutador: uma mudança R1 exigir retirar qualquer
uma das garantias funcionais P1137 ou alterar o corpo CLI para que o novo
contrato externo consiga passar.

### Única obrigação ativa R1 deste owner

P1137 deve permanecer funcional e preservar sua sequência de processo:
publicação, asset observado, irrelevante filtrado, erro transitório, último
artefacto válido, recuperação, liveness e cleanup RAII. Nenhum sentinel, FIFO,
sleep de prontidão, timeout maior, retry-until-pass ou carga passa a valer como
prova de armamento.

A prova determinística da ordem R1 pertence exclusivamente ao futuro owner
`00_nucleo/prompts/infra/tests/p1297_watch_capability_contract.md` e ao seu
único consumer `03_infra/tests/p1297_watch_capability_contract.rs`, a serem
criados por P2 somente após confirmação humana deste gate. Esse contrato
externo observará uma transição de filesystem já congelada pela capacidade;
não dependerá de P1137 observar o intervalo entre operações.

Durante esta parada, nenhum corpo de `04_wiring/tests/cli.rs` é alterado ou
legitimado. A única mudança no consumer deste Prompt é o resselo mecânico do
header. O owner/consumer externo R1 ainda não existe, evitando Prompt órfão e
preservando ownership 1:1 conforme ADR-0129.

### Gate ADR-0127

O contrato público e a forma de finalização R1 exigem confirmação humana após
os bytes concretos dos L0s. Este owner para antes de contrato externo, RED,
mutante, selo ou código; a confirmação deve preceder a criação do novo par
owner/consumer por P2.

## Histórico de revisões

| Data | Motivo | Arquivos afetados |
|------|--------|-------------------|
| 2026-04-23 | Passo 114 — suíte inicial de integração da CLI | `tests/cli.rs` |
| 2026-08-26 | P1198 — owner 1:1 e pin dos observáveis de processo | `wiring/tests/cli.md`, `tests/cli.rs` |
| 2026-09-02 | P1295/W3 — armamento observável sem sleep de prontidão | `wiring/tests/cli.md`, header de `tests/cli.rs` |
| 2026-09-03 | P1296/O1 — recuperação armada por sentinel causal | `wiring/tests/cli.md`, header de `tests/cli.rs` |
| 2026-09-03 | P1297/R1 — sentinel refutado; prova causal migra ao futuro contrato externo | `wiring/tests/cli.md`, header de `tests/cli.rs` |

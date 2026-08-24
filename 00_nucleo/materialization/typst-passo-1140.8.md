# Passo 1140.8 — estabilizar o baseline pós-P1140.6

**Estado:** planeado  
**Data:** 2026-08-24  
**Natureza:** auditoria de paridade + correção de baseline de testes  
**ADRs:** ADR-0107, ADR-0108, ADR-0127  
**Predecessor:** P1140.6  
**Sucessor deliberadamente fora de escopo:** expansão do PDF tagueado/PDF-UA

## 1. Objetivo

Remover as falhas conhecidas que impedem um baseline inequívoco depois do
P1140.6, sem iniciar uma nova frente funcional:

1. resolver os dois testes P862 sobre espaços em markup, decidindo pela
   semântica observável da linguagem e não pela forma interna do enum Rust;
2. caracterizar o teste TLS de CA customizada que requer um listener local e
   separar incapacidade do ambiente de uma regressão do produto;
3. terminar com as suítes de L1 e L3 sem falhas funcionais conhecidas.

Este passo não implementa novos papéis semânticos no PDF, não reivindica
conformidade PDF/UA e não altera contratos públicos nem comportamento padrão.

## 2. Proveniência da medição inicial

Medição feita em `2026-08-24T10:44:37-03:00`:

- commit base: `ffd527c85dd7d547413d33cbc2d27a80e32a3f8c`;
- working tree não commitado;
- `git diff HEAD --stat`: **89 ficheiros alterados, 3250 inserções e 307
  remoções**;
- `git status --short | wc -l`: **95 entradas**;
- `git diff --check`: sem erros;
- vanilla de referência executado: `/usr/local/bin/typst`, correspondente ao
  baseline ratificado `a51e02804` conforme a regra vigente do repositório.

Os números acima descrevem somente o estado em que a triagem foi feita. Devem
ser medidos novamente no fecho; não podem ser reutilizados como prova do estado
final.

## 3. Medição anterior à decisão — P862

### 3.1 Estado dos testes cristalinos

Em `01_core/src/compiler/eval/tests.rs:10457-10476` há dois testes:

- `p862_repr_plain_text_splits_on_space` exige que
  `repr([hello world])` resulte em
  `sequence([hello], [ ], [world])`;
- `p862_content_tree_splits_plain_text_on_space` exige a representação Rust
  `Content::Sequence([Text("hello"), Space, Text("world")])`.

Na árvore medida, o primeiro recebe `text("hello world")` em vez da sequência
esperada e o segundo recebe `Content::Text("hello world")`. Portanto, são duas
asserções sobre uma só escolha de morfologia.

### 3.2 Oráculo ratificado

Comando:

```sh
/usr/local/bin/typst eval 'repr([hello world])'
```

Resultado:

```text
"[hello world]"
```

### 3.3 Classificação

- O `repr` é um observável da linguagem e deve seguir o vanilla ratificado.
- A escolha entre `Content::Text` e `Content::Sequence` é mecânica interna do
  cristalino; igualdade estrutural Rust não é paridade de linguagem
  (ADR-0107).
- A afirmação do comentário P862 de que a sequência é “paridade vanilla” é
  refutada pelo oráculo atual.
- Inferência inicial: os testes P862 ficaram obsoletos e o comportamento atual
  pode estar correto. Essa inferência será refutada se sondas adicionais
  mostrarem perda observável em fronteiras de markup, styling, whitespace ou
  `repr`.

Não se deve alterar o lexer para satisfazer a forma interna pedida pelo segundo
teste sem antes encontrar um observável de linguagem que o exija.

## 4. Auditoria L0 obrigatória

Antes de editar testes ou implementação:

1. ler `00_nucleo/prompts/compiler/eval.md` e localizar o contrato de avaliação
   de markup e `repr`;
2. ler `00_nucleo/prompts/entities/content.md` e preservar a decisão de que
   `Text`, `Space` e `Sequence` são primitivos internos do hub;
3. localizar o L0 específico do lexer/eval de markup, se existir, sem inferir
   contrato a partir do teste P862;
4. acrescentar ao L0 dono a medição reproduzível do §3 e declarar que a
   segmentação Rust não constitui critério de paridade;
5. ressellar os hashes afetados antes de alterar L1.

Esta é uma correção de paridade interna: segue fluxo contínuo segundo
ADR-0127, salvo se a auditoria descobrir mudança de contrato público,
comportamento padrão, fase do pipeline ou compatibilidade. Se descobrir uma
dessas quatro classes, parar após o L0 e pedir confirmação do dono.

## 5. Execução RED → GREEN — P862

### 5.1 Sondas mínimas

Medir no vanilla ratificado e no cristalino, pelo menos:

```typst
#repr([hello world])
#repr([hello  world])
#repr([hello\nworld])
#repr([hello *world*])
#repr([hello#h(1pt)world])
```

Registar comando, saída, hash/estado da árvore e distinguir:

- espaço ordinário dentro de texto;
- espaços repetidos;
- newline de markup;
- fronteira com elemento estilizado;
- fronteira com elemento explícito.

### 5.2 Correção esperada se a inferência se confirmar

1. substituir o teste observável P862 por expectativas medidas no vanilla;
2. remover ou reformular o teste que exige `Text/Space/Text`, porque ele fixa
   mecânica Rust sem provar efeito na linguagem;
3. manter um teste interno apenas se documentar uma invariante cristalina
   necessária a algum consumidor real, citando esse consumidor;
4. adicionar regressões para as fronteiras em que o vanilla realmente produz
   sequência;
5. não introduzir normalização de whitespace baseada somente em `plain_text()`.

Se alguma sonda refutar a inferência, escrever primeiro a regra observável no
L0 e fazer a menor correção possível no lexer/eval, preservando a liberdade de
representação interna permitida pela ADR-0107.

## 6. Medição anterior à decisão — teste TLS com CA customizada

O teste
`03_infra/src/package_downloader.rs:391`,
`custom_ca_autoriza_cadeia_local_mas_nao_hostname_incorreto`, cria um
`TcpListener` em `127.0.0.1:0`. No ambiente restrito usado na validação de
P1140.6, essa operação falhou com `Operation not permitted`.

O L0 `00_nucleo/prompts/infra/package_downloader.md` exige uma sonda TLS local
sem rede pública. O L0 `00_nucleo/prompts/shell/custom-ca-cert.md` exige provar
simultaneamente que a CA customizada autoriza a cadeia e que hostname incorreto
continua rejeitado. Logo, simplesmente apagar ou ignorar sempre o teste viola o
contrato de segurança.

Classificação inicial: limitação ambiental, não falha funcional demonstrada.
Refutação: o mesmo teste falhar depois de conseguir criar o listener, ou falhar
num ambiente que permita loopback local.

## 7. Execução — capacidade ambiental sem mascarar regressões

1. separar a criação do listener da asserção TLS, retornando erro explícito em
   vez de `unwrap` opaco;
2. identificar de forma estreita apenas a ausência de permissão/capacidade para
   bind local;
3. quando o runner não permitir sockets locais, marcar o caso como não
   executável de modo visível, com mensagem que cite a capacidade ausente;
4. quando o bind for permitido, executar obrigatoriamente as duas metades:
   `localhost` aceito pela CA e `127.0.0.1` rejeitado por hostname;
5. não tratar falha de handshake, parsing de CA, validação de hostname, timeout
   ou erro HTTP como limitação ambiental;
6. conservar os testes puramente locais de PEM vazio/inválido e roots normais;
7. executar a sonda TLS num ambiente com loopback permitido antes de fechar o
   comportamento como aprovado.

Se a solução exigir mudar o contrato público ou relaxar a segurança TLS, o
passo para no L0 pelo gate ADR-0127. Um helper privado de teste ou uma
classificação estreita de capacidade do runner não abre esse gate.

## 8. Ordem de execução

1. capturar novamente HEAD, hora e `git diff HEAD --stat`;
2. executar as sondas P862 no vanilla e no cristalino;
3. atualizar e ressellar os L0 donos;
4. corrigir primeiro os testes P862; editar produção somente se um observável
   medido exigir;
5. executar os testes P862 isolados e depois toda a suíte de `typst-core`;
6. caracterizar a capacidade de listener no teste da CA;
7. validar a sonda em ambiente que permita loopback;
8. executar toda a suíte de `typst-infra`;
9. executar build, lint cristalino, formatação e verificação do diff;
10. escrever diagnóstico de fecho em `00_nucleo/diagnosticos/`, sem colocar
    relatório de estado em `prompts/`.

## 9. Comandos de validação

Os nomes confirmados nos manifests são `typst-core` e `typst-infra`.

```sh
cargo test -p typst-core p862_repr_plain_text_splits_on_space
cargo test -p typst-core p862_content_tree_splits_plain_text_on_space
cargo test -p typst-core
cargo test -p typst-infra custom_ca_autoriza_cadeia_local_mas_nao_hostname_incorreto
cargo test -p typst-infra
cargo build --workspace
cargo fmt --all -- --check
crystalline-lint .
git diff --check
```

## 10. Critérios de aceitação

O passo fecha somente quando:

- o `repr` dos casos de whitespace escolhidos coincide semanticamente com o
  vanilla ratificado;
- nenhum teste usa a igualdade estrutural de `Content` como substituto de
  paridade da linguagem;
- os dois vermelhos P862 deixam de existir por decisão documentada, não por
  `ignore` genérico;
- o teste de CA distingue falta de capacidade para bind de qualquer falha TLS;
- a sonda completa passa ao menos uma vez em ambiente com loopback permitido;
- todas as falhas restantes, se houver, são enumeradas com classificação e
  prova; “já existia” não basta para aceitar uma falha;
- `cargo build --workspace`, `cargo fmt --all -- --check`,
  `crystalline-lint .` e `git diff --check` passam;
- o diagnóstico final regista HEAD ou working tree exata, `git diff HEAD
  --stat`, hora das medições relevantes, comandos e resultados.

## 11. Fora de escopo e próximo passo

Ficam fora deste passo:

- novos papéis da árvore estrutural PDF;
- headings, listas, tabelas, links, figuras e artifacts no PDF tagueado;
- validação ou declaração de conformidade PDF/UA;
- débitos de rede não relacionados com a CA customizada;
- limpeza geral de warnings não causada pelas correções deste passo.

Depois de o baseline ficar verde, o próximo passo deve medir a cobertura
semântica do PDF tagueado e escolher um lote pequeno de papéis por impacto e
dependências, sem reivindicar PDF/UA antes de uma validação específica.

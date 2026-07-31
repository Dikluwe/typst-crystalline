# Passo 935 — por que `fontdb-0.23.0` do vanilla pré-computa cobertura por ~300ms total, e as tentativas do cristalino custaram 756ms-1.8s só para o pré-cálculo

**Precede este passo**: `typst-passo-934-relatorio.md` — confirmou que o vanilla real é ~300ms para
CJK/emoji, o cristalino 7.8×-27× mais lento nesses casos; identificou `fontdb-0.23.0` como a peça
que faz essa diferença, mas não investigou o mecanismo interno.

**Este é o passo que P925 devia ter sido — investigar o mecanismo real do vanilla, não presumir a
partir do nome da crate.** `fontdb` já é usado no cristalino (mencionado em relatórios anteriores,
`ADR-0055` e outros) — a pergunta não é "trocar de biblioteca", é "por que o cristalino, usando
código próprio para extrair coverage, é tão mais caro que o próprio `fontdb` fazendo algo
aparentemente parecido".

**Pré-condição de árvore**: `git status`. Confirmar estado P933-fixed (correção do shaper)
presente, P934 sem alterações de código.

---

## Fase A — confirmar o que `fontdb-0.23.0` faz de verdade, com instrumentação, não só leitura

1. Localizar a versão exacta de `fontdb` usada pelo vanilla (`0.23.0`, confirmado por P934 via
   strings do binário) e confirmar se é a mesma versão já presente nas dependências do cristalino,
   ou uma diferente — se for diferente, isso já pode ser parte da resposta.
2. Ler o código-fonte de `fontdb` 0.23.0 (crate pública, disponível via `cargo doc`/código-fonte
   baixado) — confirmar exatamente o que `Database::load_system_fonts()` faz por fonte: que
   tabelas lê, se usa parse completo (`ttf_parser::Face::parse`) ou parse parcial
   (`ttf_parser::RawFace`, mencionado em P925 como mais barato), e como constrói qualquer estrutura
   de cobertura.
3. **Confirmar com instrumentação directa, não só leitura**: escrever um pequeno programa Rust
   separado (fora do cristalino, um binário de teste isolado) que só chama `fontdb::Database::
   load_system_fonts()` e mede o tempo — isolando esse custo de qualquer outra coisa que o
   `typst`/cristalino faça. Comparar esse número com os ~300ms totais medidos por P934 para o
   vanilla inteiro (compilação completa, não só o load de fontes) — confirmar quanto desse
   orçamento total é mesmo gasto em fontes.
4. Confirmar se o vanilla usa `fontdb` só para **descoberta e indexação inicial** (nomes, famílias,
   caminhos — rápido, per leitura de P925: "~20ms para 1906 faces" usando `RawFace`) e faz a
   extração de **cobertura de caracteres** (a parte cara, iterar a tabela `cmap`) em separado, de
   forma lazy, tal como o cristalino já faz desde P927/932 — nesse caso, a diferença não estaria em
   "pré-computar tudo mais barato", estaria noutro lugar (por exemplo, no próprio código de
   fallback/shaping consumir essa informação de forma mais eficiente). **Não presumir que o vanilla
   pré-computa tudo eager só porque P925 leu isso no código-fonte da aplicação** — `fontdb` pode
   ter uma API que parece eager mas na prática só materializa o que é pedido.
5. Se `fontdb` de facto extrai coverage de todas as fontes eager e isso é mesmo barato: confirmar
   por que é mais barato que a extração do cristalino — comparar as duas implementações
   linha a linha (algoritmo de iteração da cmap, formato de dados usado para representar cobertura,
   se há paralelismo interno na própria crate, se usa `mmap` em vez de leitura de ficheiro
   completa).

## Fase B — implementar a abordagem do vanilla via `fontdb`, medir quanto já resolve

**Objetivo explícito deste passo: portar o mecanismo real do vanilla para o cristalino (não só
diagnosticar), medir a distância que sobra, e deixar isso como base para uma segunda rodada de
soluções, se ainda for preciso.** Só avançar para aqui depois da Fase A confirmar, por
instrumentação directa, o que `fontdb` de facto faz — não implementar em cima de suposição.

1. **Se a Fase A confirmar que `fontdb::Database::load_system_fonts()` (ou a combinação que o
   vanilla usa) já entrega, sozinha, coverage pronta e barata**: substituir o mecanismo actual do
   cristalino (extracção própria de coverage via `ttf_parser` em `font_info_from_bytes`/
   `candidates_for_char`) por uma chamada directa a essa API do `fontdb`, portando a fórmula/
   abordagem literalmente — mesma disciplina de `ADR-0123` aplicada aqui: ler o mecanismo real,
   portar fielmente, não reinventar uma versão "parecida".
2. **Se a Fase A confirmar que o ganho vem de outro lugar** (por exemplo, `fontdb` só indexa
   rápido e a coverage real ainda é lazy, e a diferença está em como o vanilla consome isso no
   shaper): portar esse mecanismo em vez do que a hipótese original presumia.
3. **Protocolo de dois agentes** (mesmo de P898, dado o risco de mudança estrutural em
   carregamento de fontes): um agente escreve testes com base no mecanismo confirmado na Fase A,
   outro implementa.
4. **Gate**: se a integração mudar contrato público (`FontMetrics`, `World`, ou estrutura de
   `FontInfo`/`FontBook`): parar, editar L0s, sincronizar hashes, confirmar com o dono antes de
   prosseguir — mesmo protocolo de sempre nesta frente.
5. Suíte completa verde, discriminada por crate.
6. **Medir tudo, com o vanilla real** (`lab/typst-original/target/release/typst`, confirmado por
   string distintiva, não por nome de caminho — lição de P934): os 7 cenários canônicos
   (`depois/antes`, para confirmar zero regressão no caso comum) **e** os 5 casos UTF-8
   (`utf8-latin`, `utf8-greek`, `utf8-cjk`, `utf8-emoji`, `05-utf8`), desta vez comparando contra o
   vanilla real confirmado, não contra um binário mal identificado.
7. Registar a distância que sobra depois desta implementação — mesmo que não feche 100% a
   diferença, isso vira a base de partida para uma rodada seguinte de tentativas, com o ganho já
   conquistado nesta preservado (não descartar o que funcionou só porque não fechou tudo).

## Fase C — se sobrar distância, registar candidatos para a próxima rodada

Se, depois da Fase B, o cristalino ainda estiver significativamente mais lento que o vanilla em
CJK/emoji: documentar exactamente quanto sobra, com números, e listar hipóteses não confirmadas
para investigação futura — não tentar fechar tudo num passo só se a causa não for óbvia depois da
implementação inicial.

## Resultado esperado

- Mecanismo real de `fontdb-0.23.0` confirmado por instrumentação directa (programa de teste
  isolado), não só leitura de código-fonte.
- Mecanismo do vanilla **implementado** no cristalino (não só diagnosticado), com gate cumprido se
  mudar contrato público.
- Medição final contra o vanilla real (confirmado por string distintiva), 7 cenários canónicos +
  5 casos UTF-8, mostrando quanto da distância de 7.8×-27× foi fechada.
- Se sobrar distância: candidatos registados para segunda rodada, com o ganho desta preservado.

# Passo 1345 — substituir o oráculo heurístico por gramática canônica fechada e materializar P1344

## Estado e regime

Regime: **executado sem atestacao de isolamento**.

Este passo usa o protocolo completo de materialização segregada. Ele não concede
uma terceira revisão local ao oráculo P1344. Abre uma cadeia nova porque P1344
parou corretamente antes do candidato após repetir duas classes públicas de
falha.

Estado medido em `2026-09-11T07:02:43-03:00`:

- `HEAD = 2f42d64253547734564513a1159ee6b584c1c4b4`;
- `sha256(git status --porcelain=v1) =
  ee300649d6c028fe8f6bb2ba85455708ff8e069d1cd2a2005c223580bf1d5dfd`;
- `sha256(git diff HEAD --stat) =
  9783f07bc86c21e8874ebacec9d92ce5a12208e127748377c571b091a1c92e15`;
- working tree não commitado: 76 arquivos no stat medido por P1344, com
  alterações anteriores que não pertencem automaticamente a este passo.

## Entradas protegidas

- passo P1344: `00_nucleo/materialization/typst-passo-1344.md`, SHA-256
  `1fff15c6f17800ca293379e17d8d2ed63b2fe9043a0aebb0baf16a9b9b0ea6f3`;
- contrato P1344 R1: `p1344-contract-spec-r1.json`, SHA-256
  `157877514d350947f6483ec371c356dd2181c2e0f98e8fa9eb5e465527983279`;
- binding P1344 R1: `p1344-contract-binding-r1.json`, SHA-256
  `1f4d8812d0a4e2a9451d2f8c08931fb71e1baa54a3484a411d2cf76727115bab`;
- baseline de cápsulas P1344: `p1344-capsule-baseline-r1.json`, SHA-256
  `f64744d5bdc31734f791af24acf4204949a77c019542d5baa61c48b07e7dcaea`;
- freeze L0 P1344: `p1344-l0-freeze-r1.json`, SHA-256
  `6cc3a61f2d5177b3ede1d7805f8db8f7edfcd0a2f9ea55aec50405db7832c75d`;
- relatório adversarial P1344 R2: `p1344-adversary-report-r2.json`, SHA-256
  `2c536123d13e3aa537d5c7071f2910bb5b502fdf14aea82eb931b875d9d6bf9f`;
- recibo adversarial P1344 R2: `p1344-adversary-receipt-r2.json`, SHA-256
  `0c830744ffa6714599bc7c428277b1d37246db6d7815364b04044045752b7c4b`.

Todos os artefatos P1343/P1344 anteriores são história de ataque ou entradas
compostas nos hashes acima. Nenhum deles é selo P1345.

## Medição antes da decisão

P1344 R2 rejeitou os 23 ataques R1, mas 14 de 17 ataques novos sobreviveram:
13 sob `INHERITED_AND_FUNC_PRODUCTIONS_NOT_CLOSED` e um sob
`UNKNOWN_LAUNDERED_WITHOUT_WITNESS`. O score agregado foi `26/40 = 0.65`,
com `full_corpus_runs = 0`. As falhas vêm de contagem de nomes, blacklists e
autoatestado de sete strings; não vêm da topologia owner-local, que fechou no
recorte.

A opção de usar APIs HIR/MIR internas foi medida e rejeitada para este passo:

```text
rustc 1.92.0 (ded5c06cf 2025-12-08), stable-x86_64-unknown-linux-gnu
rustc-dev: não instalado
```

Inferência: um verificador baseado em `rustc_private` não seria reproduzível no
ambiente congelado. Refuta esta inferência a presença futura de uma toolchain
pinada com `rustc-dev`; isso exigirá outro passo, não alteração silenciosa deste.

## Decisão protocolar

O contrato semântico P1344 R1 permanece íntegro. A falha está na concretização
do oráculo. P1345 substitui os predicados heurísticos por uma linguagem de fonte
finita e fechada:

1. o autor do oráculo constrói uma árvore positiva sintética apenas a partir do
   contrato e do baseline candidate-free;
2. extrai de cada uma das 38 cápsulas a sequência canônica integral de tokens,
   ignorando apenas whitespace e comentários;
3. sela, por cápsula, ID, caminho, owner, espécie insert/replace, tokens, digest,
   âncoras e replacement inverso;
4. o source verifier extrai a cápsula candidata por markers reais e exige
   igualdade integral da sequência de tokens; qualquer token extra, ausente,
   reordenado ou substituído é `Violated` com primeiro índice divergente;
5. normalização inversa continua obrigada a recuperar os 12 hashes completos;
6. `rustfmt`/Cargo resolvem sintaxe, nomes, tipos e borrow; o A/B compilado e
   desafiado resolve reachability, identidade, cardinalidade e efeitos runtime.

Esta igualdade de tokens é um gate arquitetural das cápsulas privadas de
observação, não uma alegação de paridade mecânica da linguagem Typst. O
certificado permanece limitado ao binding P1342/P1345. Formatação e comentários
não são observáveis; todos os demais tokens da produção fechada são.

### Política de `Unknown`

- source, autoridade, schema, ausência de runtime ou forma não suportada nunca
  retornam `Unknown`; retornam `Violated`;
- corpus não fornece classificação nem witness autoatestado;
- o único `Unknown` opaco é derivado pelo checker ao executar um probe selado,
  com challenge fresco fornecido pelo verificador, hash do executável e
  observações primitivas no recibo;
- recibo ausente, reutilizado, com challenge divergente ou emitido pelo
  candidato é `Violated`;
- na verificação final o probe/A-B usa o candidato compilado real; sintético é
  permitido somente no corpus discriminatório pré-selo.

## L0 e arquitetura Tekt

Não há nova mudança de L0 neste passo. Os três L0 ajustados antes de P1344 já
legitimam helpers internos `cfg(p1339_observation)` nos consumers 1:1 reais:
`Func` em `func.rs`, `Content` em `content.rs` e `CounterUpdate` em
`counter_update.rs`. O freeze P1344 e V5/V15/V26 devem ser revalidados antes do
contrato e no fim.

É proibido:

- criar 13º consumer ou novo owner para satisfazer o verificador;
- mover helpers para wrapper, trait, macro, const container ou impl não local;
- usar import/reexport reverso;
- alterar API pública, comportamento por defeito, fase do pipeline ou
  compatibilidade;
- aceitar templates alternativos por blacklist, regex, contagem ou semântica
  inferida de nomes.

Qualquer necessidade acima aciona ADR-0127 e para antes do código.

## Autoridades e allowlists

Um manifesto P1345 registra executor, entradas, escritas e hashes. As funções
podem usar o mesmo filesystem, mas nenhuma autoridade acumula obrigação,
solução e veredito.

1. **Baseline/protocolo**: lê apenas entradas protegidas, L0 e toolchain; escreve
   freeze/manifesto/recibo P1345. Não lê candidato.
2. **Autor do contrato**: lê manifesto, P1344 R1 e blocker; escreve contrato,
   binding e recibo P1345. Não lê candidato nem oráculos.
3. **Autor do oráculo**: lê contrato selável e baseline; escreve tabela canônica,
   fixture positiva, source verifier, checker, corpus, probe e recibo. Executa
   somente focais.
4. **Adversário**: lê contrato/oráculos/baseline; escreve mutantes, runner,
   relatório e recibo. Não corrige o oráculo.
5. **Pré-verificador**: não escreve entradas; executa um full normal/repeat/reverse
   e emite relatório, recibo e selo somente com score `1.0`.
6. **Testador A/B**: recebe L0, contrato e selo, mas não o patch; escreve o teste
   Rust P1345 e recibo RED.
7. **Implementador**: recebe L0, contrato, baseline, selo e RED; escreve somente
   os 12 consumers/38 cápsulas. Não edita diagnósticos, contrato ou testes.
8. **Verificador final**: lê tudo, não corrige; executa gates e escreve recibos e
   certificado focal.

## Corpus discriminatório novo

O corpus P1345 deve compor, sem reinterpretar:

- os 40 ataques válidos P1344 R1+R2;
- todos os controles positivos P1344;
- ao menos uma mutação token-level por cada uma das 38 cápsulas;
- mutações de token extra/ausente/reordenado/substituído, comentário e string
  decoy, marker/owner/anchor/normalização, corpus/hash/root/JSON duplicado;
- controles do probe: challenge fresco, replay, receipt ausente, emissor errado,
  hash de executável errado e caso opaco real.

Cada negativo parser-valid deve resultar `Violated`. A tabela canônica precisa
ser derivada antes do candidato e pinada fora do checker; uma tabela alternativa
self-consistent deve falhar por authority root.

## Ordem de execução

1. congelar manifesto, toolchain, 12 L0/consumers e baseline candidate-free;
2. autorar contrato P1345 e validar composição integral do P1344 R1;
3. autorar oráculo/table/probe/corpus e executar apenas focais;
4. ataque independente; corrigir somente classes novas dentro do budget;
5. pré-verificador independente executa o único full pré-selo em
   normal/repeat/reverse e sela score `1.0`;
6. testador independente escreve o teste Rust, substitui somente o include
   histórico inválido P1342/P1344 pelo include P1345 e prova RED;
7. implementador materializa as 38 cápsulas/12 consumers;
8. testador/verificador executa GREEN três vezes com `/dev/shm` para temporários;
9. verificador final executa full final, normalização, rustfmt, Cargo e Tekt e
   emite certificado limitado.

## Gates de aceitação

- hashes e schemas fechados sem chaves JSON duplicadas;
- 38 sequências canônicas únicas e 12 owners; consumo integral de tokens;
- 12 normalizações inversas recuperam os hashes candidate-free;
- positivos `Preserved`, negativos válidos `Violated`, único probe opaco
  `Unknown`; score completo `1.0` em normal/repeat/reverse;
- pré-selo existe antes de teste/candidato;
- RED independente e GREEN repetido três vezes com inputs frescos;
- `cargo fmt --all -- --check`;
- `RUSTFLAGS='--cfg p1339_observation' cargo test --workspace --no-run`;
- teste A/B focal P1345;
- `cargo build --workspace` normal sem cfg de observação;
- V5/V15/V26, `crystalline-lint .` e `git diff --check` sem violações novas;
- hashes protegidos e arquivos não allowlisted permanecem intactos.

## Budget e parada

- um contrato P1345 inicial e no máximo uma revisão;
- no máximo duas revisões focais do novo verificador, cada uma com nova
  testemunha, causa pública e ganho discriminatório;
- autores e adversário executam zero full;
- uma única execução full pré-selo e uma final;
- no máximo dois ciclos de correção do candidato;
- mesma causa/vetor por duas tentativas para o passo sem terceira correção;
- qualquer sobrevivente, regressão positiva, score menor que `1.0`, drift,
  recibo opaco autoatestado, falta de runtime real, mudança ADR-0127 ou
  enfraquecimento de negativo para e impede selo/certificado.

Budget não autoriza waiver, template candidato-controlado, conversão de falha em
`Unknown` nem alegação de equivalência funcional geral.

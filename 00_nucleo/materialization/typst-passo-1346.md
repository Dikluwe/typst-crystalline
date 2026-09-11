# Passo 1346 — fechar o transporte do oráculo, pré-selar e materializar as 38 cápsulas

## Regime e objetivo

Regime: **executado sem atestacao de isolamento**.

P1346 é uma cadeia nova de protocolo completo. Não concede revisão adicional a
P1345. Corrige três defeitos de transporte comprovados depois do stop P1345 e,
somente se o novo gate discriminatório atingir `1.0`, executa pré-selo, RED,
materialização e verificação final das 38 cápsulas em 12 consumers.

Estado medido em `2026-09-11T09:49:57-03:00`:

- `HEAD = 2f42d64253547734564513a1159ee6b584c1c4b4`;
- `sha256(git status --porcelain=v1) =
  d0fe07af6aed468535557cbfd02834e72cb70b4cbfb6657cd19882436732f95b`;
- `sha256(git diff HEAD --stat) =
  9783f07bc86c21e8874ebacec9d92ce5a12208e127748377c571b091a1c92e15`;
- working tree não commitado; alterações fora da allowlist P1346 são preservadas.

## Entradas protegidas e bloqueio medido

- P1345: `typst-passo-1345.md`, SHA-256
  `c2ef7ff7b0555a4f44ca6811ff3e687ef6d6a30f55e0cd106fba4118f3a18fb0`;
- blocker final: `p1345-blocker-final-r1.md`, SHA-256
  `dc1424e7f3ac1b2a4990aeb803ca4bb57caf6d445a75fac5e06443d7a82658cc`;
- blocker receipt: `p1345-blocker-final-receipt-r1.json`, SHA-256
  `db4405e38c845bf7855175485903f23527fb95e4e8e8cf89fda3c134ffad269f`;
- contrato FINAL P1345: spec R2 SHA
  `f40c2b42fe83a4b75e276ab2ea8a9639fa1553e80c06581caf8c53d948f9d38d`
  e binding R2 SHA
  `ecc3a8a403d9d14d51808a35876f1f43d7773732c1d20f58574dff42d5187778`;
- tabela canônica: SHA
  `ffe9322a609c3f36ea152ce8a872407ebd7daddfef033660b495e235f0d2301f`;
- fixture positiva: SHA
  `8cf4078146a3625931027d65a56f2610b132b2e9f6cf775ab4761b849a4d9e12`;
- source verifier R2: SHA
  `9434ca3f2191cade3207d236a553b76045a016a2770da243251dc5d2d9527250`.

Medições que obrigam o passo:

1. focal P1345 final: `120/122`, score
   `0.9836065573770492`; `P1344-A21` e `P1344-R2A17` recebem `exit 2`, mas
   o adapter registra `Preserved` porque `argparse` intercepta flags ausentes
   antes do reason `AUTHORITY_ROOT`;
2. caller R3 rejeita o próprio recibo de falha antes de executar o focal;
3. `crystalline-lint --checks v5,v15,v26 .` falha em PARSE sobre
   `p1345-opaque-probe-r1.rs:117`, embora `rustfmt` e `rustc 1.92.0` aceitem;
4. os mesmos bytes do probe compilam com `rustc --crate-name ...` quando
   guardados como `.rs.txt` em `/dev/shm`.

## Decisão protocolar

O contrato semântico, a tabela de 38 produções, a fixture e a topologia P1345
permanecem normativas. P1346 substitui somente o transporte:

### Preflight antes do parser de CLI

- uma leitura manual, fechada e sem abreviações identifica exatamente uma
  ocorrência de cada flag de autoridade e do corpus antes de `argparse`;
- corpus/hash/root alternativo é rejeitado como `AUTHORITY_ROOT` mesmo se
  outras flags obrigatórias estiverem ausentes;
- duplicação, abreviação, alias textual de path e argumento desconhecido são
  rejeitados na ordem normativa;
- somente depois do preflight o parser fechado valida a invocação completa.

### DAG acíclico de autoridade

```text
table + fixture + probe-source + source-verifier + corpus + checker
    -> authorship receipt verde
    -> caller (pina checker + receipt + authority root)
    -> delivery receipt (pina caller)
    -> pre-verificador recebe delivery-receipt SHA fora da banda
```

O caller nunca é pinado pelo recibo que ele próprio consome. Recibo de falha não
é entrada de caller selável. O pré-verificador rehasha delivery receipt, caller,
checker, authorship receipt e root antes da execução; substituir todos e também
os pins fornecidos não é um ataque ao mesmo trust anchor, mas outra autoridade.

### Probe fora do namespace Rust do repositório

`p1345-opaque-probe-r1.rs` é artefato de uma cadeia falhada e nunca foi selado.
Antes do freeze P1346, uma autoridade de baseline deve:

1. copiar seus bytes exatamente para
   `00_nucleo/diagnosticos/p1346-opaque-probe-r1.rs.txt`;
2. confirmar o mesmo SHA-256
   `863fa1588083638ec9f52b7ee663023e260a09880244c61cc948df36e946e2dc`;
3. remover somente o path histórico `.rs` e registrar a relocação;
4. compilar o `.rs.txt` explicitamente com `rustc --crate-name` para `/dev/shm`.

Isso preserva os bytes probatórios e impede que o linter confunda fixture de
diagnóstico com consumer Rust materializável. Não se altera o linter.

### Execução do probe

O checker copia bytes rehashados do executável para `memfd`, sela o descritor e
executa `/proc/self/fd/N`; challenge, nonce, schema, canonical JSON e emissor são
recomputados. Path, symlink, swap/restore e TOCTOU nunca selecionam bytes depois
do hash. Ausência de `memfd`/sealing é `Violated`, não fallback.

## L0 e escopo Tekt

Nenhuma mudança L0 é necessária. P1346 revalida o freeze 12/12 e V5/V15/V26.
Os L0 já legitimam as cápsulas internas `cfg(p1339_observation)` e os owners
reais `Func`, `Content` e `CounterUpdate`. Não há contrato público,
comportamento default, fase ou compatibilidade novos.

É proibido novo consumer, wrapper, trait, macro/const container, impl não local,
import reverso, side table ou alteração do linter para esconder o probe.

## Autoridades

1. **Baseline/manifesto**: reloca somente o probe histórico, congela 12 pares,
   toolchain, tabela/fixture e escreve manifesto/recibo P1346.
2. **Contrato de transporte**: compõe P1345 FINAL e formaliza preflight+DAG;
   não lê candidato.
3. **Oráculo**: escreve probe-source `.rs.txt`, checker/caller/corpus/receipts
   P1346; executa apenas focal.
4. **Adversário**: reexecuta os 122 negativos P1345 e ataques de transporte;
   não corrige.
5. **Pré-verificador**: recebe delivery SHA fora da banda, executa um único full
   normal/repeat/reverse e sela somente score `1.0`.
6. **Testador A/B**: recebe L0+contrato+selo, não o patch; escreve teste P1346 e
   prova RED.
7. **Implementador**: recebe L0+contrato+selo+RED; escreve somente 38 cápsulas
   nos 12 consumers.
8. **Verificador final**: não corrige; executa full final, runtime e arquitetura
   e emite certificado limitado.

Cada papel registra allowlist, contexto, inputs, outputs, hashes e instante. A
partilha do filesystem impede alegar isolamento forte; por isso o regime não
muda.

## Corpus e ataques obrigatórios

- reexecutar os 122 negativos e oito controles finais P1345;
- preflight: corpus/root alternativo com flags ausentes, flags duplicadas,
  abreviadas, desconhecidas e aliases de path;
- DAG: ciclo caller↔receipt, receipt de falha, caller trocado e delivery pin
  divergente;
- probe: schema/tipos/canonical bytes, challenge/nonce/replay, emissor/binário,
  symlink e swap/restore concorrente;
- pelo menos uma mutação por cada uma das 38 cápsulas e os 40 ataques P1344;
- JSON duplicado e troca coordenada julgada pelo caller externo com os pins
  canônicos recebidos, nunca por self-root.

## Ordem e gates

1. relocação hash-preserving, freeze e manifesto;
2. contrato de transporte;
3. oráculo focal;
4. adversário focal; qualquer survivor bloqueia;
5. pré-verificador: exatamente um full normal/repeat/reverse, score `1.0`, selo;
6. A/B independente: novo teste, include P1346, RED reproduzido;
7. implementação das 38 cápsulas;
8. GREEN três vezes com inputs frescos e temporários em `/dev/shm`;
9. full final uma vez;
10. `cargo fmt --all -- --check`;
11. `RUSTFLAGS='--cfg p1339_observation' cargo test --workspace --no-run`;
12. `cargo build --workspace` normal;
13. V5/V15/V26, `crystalline-lint .` e `git diff --check`;
14. certificado focal P1342/P1346.

## Aceitação e orçamento

- 38 cápsulas/12 consumers, 2.506 tokens canônicos e normalização 12/12;
- todos os negativos `Violated`, positivos `Preserved`, único probe opaco
  executado `Unknown`, score `1.0` em todas as ordens;
- source `.rs.txt` compila e nenhum `.rs` de probe fica sob diagnósticos;
- caller e receipts formam DAG acíclico e passam pins externos;
- preseal antecede RED/candidato; full pré-selo=1, full final=1;
- no máximo uma revisão contratual de transporte, uma revisão focal de oráculo
  e dois ciclos de candidato;
- autores/adversário fazem full=0;
- mesma causa/vetor duas vezes, score menor que `1.0`, drift, falta de trust
  anchor externo, falha de lint, ADR-0127 ou negativo enfraquecido para e
  impede implementação/certificado.

Nenhum resultado autoriza equivalência funcional geral, fechamento de P1340 ou
P1339, NT01–NT06, retenção, descarte, invalidação ou política terminal.

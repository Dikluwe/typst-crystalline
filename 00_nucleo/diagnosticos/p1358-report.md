# P1358-R2 — verificação pós-implementação independente

**Veredito final R2: PASS.** O candidato produtivo continua exatamente em `b40500f02f30ac17b80020766b0a79fa4e9dcb9c41fa5bf8629f077d7163edb3`. A R2 altera somente uma string de fixture no teste obsoleto, alinha-a ao contrato selado de alpha posicional e faz o teste exato passar sem modificar seus asserts.

Execução R2 UTC: `2026-09-13T00:07:48Z`. Executor: `/root/verificador_p1356`. Regime: protocolo completo de materialização segregada. **Executado sem atestação de isolamento.** O verificador não alterou candidatos nem inputs protegidos.

## Histórico das revisões

- **R0 — FAIL:** `color.rs` media `1bc421465011759ae27749850dc6658c7b434568cbd6c3d58a02b698b17b1263` e declarava o SHA bruto `c0938544` como `@prompt-hash`; o linter calculava `c3001912` e emitia V5. Artefatos R0: verification `b99716f561b6aa6c5bd694bfba811c158da1277aa535c77d4f6920aebbe6e4aa`, report `5fb6b992170ee61efbc80bd6cd0beae08775724786170cfbbe5640fbf6ace8db`.
- **R1 — PASS funcional e de linhagem:** a alteração comentário-only para `@prompt-hash c3001912` resolveu V5; verification `8dbe87b235664a6de09e3d0c700986a4cbd2c37268fd1726c4587171ebdb6aea`, report `3ac7bb45c4623677639065f162a6fea1acbda0c958443f56bd9e531fca1e0165`.
- **R2 — PASS do gate integral focal:** a fixture obsoleta deixa de usar o named arg rejeitado pelo novo contrato. Nenhum achado bloqueante permanece.

## Prova do delta R2

Em `01_core/src/compiler/eval/tests.rs`, há uma única substituição:

```text
luma(54.02%, alpha: 40%) -> luma(54.02%, 40%)
```

O arquivo atual mede `95020fe449edece4e3c469dcd2efe306e3e6f1a6d3d4a03e91f28df22632e730`. A reversão exclusiva dessa string, feita em memória, produz `06370a21b4b86f5ea4d91b628bed43a215d04cf7487ee1071f22af9fd81d8056`. O diff é 1 inserção e 1 remoção; nenhum assert mudou.

A semântica testada permanece idêntica: o array deve conter cinco elementos, cada elemento deve ser `Ratio`, e cada alpha deve diferir de `0.4` por menos de `1e-6`. A nova chamada apenas passa alpha como segundo componente posicional, conforme o contrato P1358; named args continuam rejeitados.

O recibo declarado `/tmp/p1358-r2-exact-test.txt` mede `d0d25e8ef21e9774fa2879cb959268d64189203de5f9256edf33400d6f820e48` e registra 1/1 PASS. A reexecução independente do comando exato também terminou com 1 passed, 0 failed e 5670 filtered out.

Como evidência complementar, o executor `/root` informou a reexecução de `cargo test --workspace --release --locked --no-fail-fast -- --quiet` com exit 0: `typst-core` lib 5671/5671, infra 923/923 e demais alvos/doc-tests PASS. Esta evidência integral é registrada com sua proveniência e não é apresentada como execução do verificador.

## Evidência funcional e discriminatória preservada

A R2 não altera bytes do produto, portanto permanecem aplicáveis as medições sobre o mesmo `color.rs`:

- RED 4/4 falhas antes da implementação; GREEN focal 4/4 e módulo `color` 32/32 depois dela.
- Gate abstrato: 108/108 positivos Preserved, 3/3 opacos Unknown sem crédito, 10/10 mutantes mortos, `mutation_score=1.0`.
- Matriz bilateral contra vanilla ratificado `586e1bd43fae6c9a973218163d3165c53ab8d16d`: quatro perfis, quatro execuções, 816 processos e 212/212 observações creditáveis Preserved; 0 Violated, 0 Unknown.
- V01–V09: 72/72; D01–D16: 128/128; P01/P02: 12/12. Os quatro hashes determinísticos permanecem `37538f2b52643ae7262de3c0ba4f6d3fc6ea48ec7f0c707e47262d1cb31fc018`.

A nota não bloqueante de D01 permanece: vanilla `[1,0,6]`, cristalino `[1,4,2]`; ambos localizam publicamente a ausência de `lightness`, e o cristalino destaca `()`.

## Integridade, escopo e gates

Os inputs congelados foram revalidados sem deriva: manifest `8286a48131b94810623dd53787325dff748888b95d69916ea0fd281f7dc1ea0c`, L0 bruto `c0938544b8fcf20a34b43729bd29a49b945a510fc370b5caa9cbaf1c0a590d3c`, passo `9078953e7c6e9de0082072c858d6d1ce7a007fa150b6df3bec339ee56a52b709`, preflight `534facef55519ce2aec8a49c47e405a873a0e69740d49caa20e7449ab09bc98f`, contrato `85715abf321c2f5438780999336fcade0bcb685e71b4cd96db61e8726483ea1c`, oráculos `a90ee4b0c2cd19720db1f27239b1e6999222fb2bdb249649effb24e1b2529d8a`, mutantes `9a4ade0e8f9eb1ccb5f37b38b095f93f40d553e71fdb181c101ddef416d4bbfa` e selo `811a3929c6382df034ed684f50b365950189dce4406ee1fbecaccf0090673d60`.

O escopo P1358 em L1–L4 contém somente o consumer produtivo `color.rs` e a fixture em `compiler/eval/tests.rs`. `cargo fmt --all -- --check`, o teste exato, `crystalline-lint .` e `git diff --check` passaram. A busca focal na saída integral do linter confirmou V5=0, V15=0 e V26=0.

Estado da medição: HEAD `8ceea20f92dff06b6ca5a34828135c0fd3e35248`, árvore `c3c2fa834e6ec318690ddb88e181277ff2e25b27`, working tree não commitado. O inventário exato do estado e os comandos reproduzíveis constam em `p1358-verification.json`.

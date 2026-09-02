# P1293 — receipt de reabertura final do observador `compiler/layout/tests`

**Papel segregado:** autor contratual/L0  
**Estado:** `L0_UPDATED_AWAITING_SINGLE_HEADER_RESEAL_AND_TEST_ONLY_FIX`  
**Timestamp:** `2026-09-02T18:49:01-03:00`  
**HEAD:** `7dd25ff0e222b6c7c640d6bc7957b98f94227507`  
**Working tree:** não commitada e compartilhada; este papel atribui a si apenas
o L0 e este receipt. Consumer, produto, oráculos, manifesto e selo não foram
editados.

## Entrada bloqueante e escopo

O receipt independente
`00_nucleo/diagnosticos/p1293-final-verification-blocker-receipt.json`, SHA-256
`6dbf19c3ff89a0d10324ef9b5e273645e673eaf107a0cc268cc171b489708d2c`,
encerrou com `BLOCKED_NOT_READY_FOR_FINAL_CERTIFICATE`: build release, fmt,
lint, surfaces, oráculos P1292/P1293 e discriminação 31/31 passaram, mas
`cargo test --workspace -q` falhou porque `13` de `5416` testes core ficaram
RED. Zero `Unknown` foi convertido em sucesso e nenhum certificado foi emitido.

A autorização desta reabertura é estritamente:

- L0 owner `00_nucleo/prompts/compiler/layout/tests.md`;
- futuro consumer test-only `01_core/src/compiler/layout/tests.rs`, somente o
  helper recursivo/auxiliares de observação;
- este receipt.

Não há autorização para `equation.rs`, outro produto, expectativa, fixture,
oráculo, manifesto ou selo nesta fase.

## Medição anterior à decisão

O consumer SHA-256
`92a2239354c3f82b364671ffa2d5d6a26b4767adfe3756571f7cf73b5f4f1c6c`
declara `@prompt-hash 9eaf493c`. Seu helper
`frame_items_recursive` em `01_core/src/compiler/layout/tests.rs:31-48`
devolve referências aos itens e atravessa `Semantic`, `Group` e `Link`, mas
não carrega `Group.pos` nem `Group.matrix`. Os 13 REDs medidos atravessam P813,
P896, P987, P1088, baseline inline e limits; testemunhos locais como `x=0` e
`y≈7.7` são comparados às coordenadas globais congeladas `x≈294.34` e
`y≈99.47`.

Em `01_core/src/entities/layout_types.rs:484-502`, `Group.pos` pertence ao
espaço do pai, `matrix` é afim e os filhos estão no espaço local. Em
`:1045-1093`, `TransformMatrix::concat/apply` já fornece composição e projeção.
O Núcleo pinado `layout-coordinates` exige o mesmo rebase para FrameItems,
grupos e emissões diferidas.

Medição direta: a perda ocorre na fronteira do helper antes das asserções.
Inferência: acumular a transformação ancestral recompõe as coordenadas globais
sem alterar produto ou expectativa. Refutador: qualquer um dos 13 casos ainda
divergir após a composição completa, mudar morfologia/cardinalidade dos itens,
ou exigir alteração produtiva; nesse caso o implementador test-only para e o
owner produtivo causal precisa ser reaberto separadamente.

O receipt bloqueante contém internamente o owner SHA prefixado `deee338…`, mas
o ficheiro pré-decisão medido diretamente foi
`deee32e8fadf311c9a8686004e27b098230b7ef348a4d8b09dd91f9f72d4f157`.
Essa divergência documental não sustenta a causalidade; helper, consumer e
witnesses foram auditados diretamente.

## Decisão L0 e classificação

O L0 agora exige que helpers recursivos geométricos acumulem o transform do
ancestral e, ao entrar em `FrameItem::Group`, componham a translação de
`Group.pos` e `Group.matrix` na ordem vigente. Grupos aninhados compõem
transitivamente e pontos/linhas usados em asserções são projetados uma única
vez ao referencial global. `Semantic` segue transparente e `Link` não recebe
deslocamento inventado ou duplicado.

Carrier/cópia continua privado e test-only. Todas as 13 expectativas,
fixtures, tolerâncias, morfologia e ordem permanecem byte-conceitualmente
iguais. `01_core/src/compiler/layout/equation.rs` e seu L0 ficam congelados.

ADR-0107/0108: geometria global é observável; recursão, borrowing e cópia são
transporte de teste, medido antes da decisão. ADR-0127: correção test-only sem
API pública, default, fase ou compatibilidade; fluxo contínuo, sem gate humano.
ADR-0129: owner e consumer permanecem 1:1.

## Hashes e preservação

| Artefato | SHA-256 |
|---|---|
| L0 antes | `deee32e8fadf311c9a8686004e27b098230b7ef348a4d8b09dd91f9f72d4f157` |
| L0 depois | `35d0fa736f5cdbfaacff3645b559836149ce8ffdf6d0e76c988dbd5a90b36153` |
| consumer intocado | `92a2239354c3f82b364671ffa2d5d6a26b4767adfe3756571f7cf73b5f4f1c6c` |
| `equation.rs` intocado | `ea81dc9e291d29e1ac6d17a240b98929228af31dc2a2a1563efae20ba25d9e23` |
| L0 `equation.md` intocado | `9efca34585e1776267286b66dc74ed6ca213832b91568a8ffd79ccd2b8578179` |
| manifesto intocado | `e576ac0a84e396268fd8b2c1b8ef258712b90dd1e1a1a11132b887bfa3a5f5fa` |
| selo intocado | `435d2728681254f90d1b2d07f30d5c830de9bb7f90e9c25e18d314bb882fa6f4` |

O `Hash do Código` do L0 foi sincronizado documentalmente com o corpo canônico
vigente `e19a69a6`, sem escrever no consumer. A alteração L0 invalida o handoff
final congelado para novas decisões até manifesto/selo substitutos, mas esses
artefatos permanecem byte-idênticos por proibição desta fase.

## Gates e dry-run

- V15 com `--fail-on warning`: PASS, zero violações;
- V26 com `--fail-on warning`: PASS, zero violações;
- `git diff --check`: PASS;
- V5: exatamente um drift transitório esperado, sem segundo consumer:
  `01_core/src/compiler/layout/tests.rs`, `9eaf493c → 0d853401`;
- `crystalline-lint --fix-hashes --dry-run .`: PASS e exatamente:

```text
Would fix ./01_core/src/compiler/layout/tests.rs prompt=00_nucleo/prompts/compiler/layout/tests.md old=9eaf493c hash-a=0d853401 hash-b=e19a69a6
```

Nenhum `--fix-hashes` de escrita foi executado.

## Próximo passo permitido

O coordenador pode ressellar mecanicamente somente o header de
`01_core/src/compiler/layout/tests.rs`. Depois, um implementador test-only
separado pode alterar apenas `frame_items_recursive`/auxiliares privados para
o transporte global e deve parar para workspace/gates/verificação independente.
Qualquer necessidade de mudar expectativa ou produto é blocker.

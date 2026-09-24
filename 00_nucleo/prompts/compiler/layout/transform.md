# Prompt L0 — `transform`
Hash do Código: 88518d33

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/element-form-b.toml sha256:6dbf8faa56960845c60734f5e047685ec5b3a6f14c0ce3a48d333b8982d0baa5

**Camada:** L1 render
**Ficheiro alvo:** `01_core/src/compiler/layout/transform.rs`
**ADRs:** ADR-0107, ADR-0108, ADR-0109, ADR-0129

## Medição e contrato

Aplicar rotate, scale, skew e move a subframe medido, resolvendo pivô, matriz, Group e fixups aninhados no mesmo referencial.

O dispatcher mantém braço magro, exaustivo e estático. A função da feature
acede ao motor por descendência de módulo; entidade de domínio não importa o
consumer e nenhum despacho dinâmico é introduzido.

## Aceitação

Testes focais e a suíte do subsistema preservam comportamento e morfologia;
alteração observável exige medição e decisão próprias.
## P1493 — AABB neutra e transporte afim do carrier HTML

### Medição anterior à decisão

`transform.rs:48-68,218-243` deriva AABB/pivô da posição de todos os itens. O
carrier P1493 podia alterar esses valores; removê-lo antes do Group, por outro
lado, perderia a transformação necessária à posição final.
Sonda vanilla pinada em 2026-09-24T16:41:36-03:00 confirmou que
`move(dx: 11pt, dy: 5pt)` dentro de `place(dx: 30pt, dy: 20pt)` projeta a
Location em 41pt/25pt. O RED cristalino mostrou 30pt/20pt: a reconstrução
`final_matrix_exact` conserva `a/b/c/d`, mas descarta `matrix.tx/ty`. A medição
usou HEAD `b5abe9b288ce168c494a4c5172830e325b9aedae` e o working tree
não commitado enumerado no diagnóstico P1493; refuta tratar translação como
mera AABB neutra.


### Contrato ratificado

A AABB e o pivô ignoram somente o carrier privado, enquanto o Group conserva o
carrier entre seus filhos e lhe aplica a mesma matriz/translação dos itens reais.
A composição centrada preserva também `matrix.tx` e `matrix.ty`; os termos de
pivô somam-se a eles, não os substituem. A extração compõe essa matriz e remove
o sentinel; a página detalhada permanece idêntica à fachada legada corrigida.

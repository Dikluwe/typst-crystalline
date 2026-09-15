# Prompt L0 — `repeat`
Hash do Código: f9aa0b9d

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/element-form-b.toml sha256:6dbf8faa56960845c60734f5e047685ec5b3a6f14c0ce3a48d333b8982d0baa5

**Camada:** L1 render
**Ficheiro alvo:** `01_core/src/compiler/layout/repeat.rs`
**ADRs:** ADR-0107, ADR-0108, ADR-0109, ADR-0129

## Medição e contrato

Repetir body para preencher espaço horizontal disponível com gap resolvido, terminando defensivamente sob região infinita ou body vazio.

O dispatcher mantém braço magro, exaustivo e estático. A função da feature
acede ao motor por descendência de módulo; entidade de domínio não importa o
consumer e nenhum despacho dinâmico é introduzido.

## Aceitação

Testes focais e a suíte do subsistema preservam comportamento e morfologia;
alteração observável exige medição e decisão próprias.

## P1407 — expansão adiada sob largura fracionária finita

### Medição antes da decisão

No baseline `ed7340ada`, `repeat.rs` dispõe somente uma cópia. Em uma caixa
`1fr` de 100 pt, a sonda limitada produziu 1 ponto no cristalino e 23 no vanila
ratificado. `boxed.rs` registra a fronteira fracionária depois de dispor o body;
portanto a parcela só é conhecida em `expand_fr_spacings`.

### Contrato

O owner dispõe uma peça e registra em estado privado sua faixa, origem, largura,
gap e justify. Quando a parcela `fr` imediatamente posterior for resolvida,
expande a peça por `floor((fill + gap) / (width + gap))`, limita a 1000 cópias
e distribui o resíduo entre gaps somente quando justify está ativo e há pelo
menos duas cópias. Largura/peça inválida, região infinita ou fronteira ausente
terminam de forma limitada e limpam o pendente; não inventam sucesso. O recorte
P1407 cobre região fracionária inline finita; RTL, quebra interna e região sem
fronteira permanecem Unknown.

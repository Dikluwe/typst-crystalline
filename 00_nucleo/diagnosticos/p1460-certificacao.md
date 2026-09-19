# Certificação segregada — Passo 1460

**Veredito:** VERIFIED no recorte de ownership do fallback raw de heading

**Produto certificado:** `3ba006129cc78840210ac7e601f41574bf43b15e`

**Árvore certificada:** `d3bc464bb9b614df158fc64a9576bac55e32ed2a`

**Medição final:** `2026-09-19T03:30:24Z`

**Regime:** executado sem atestação de isolamento

## Escopo verificado

O produto P1460 elimina a decisão produtiva
`Content::text("Section")` do serializer L2 de heading. A CLI chama
diretamente `heading_supplement_for_lang(None)`; o owner L1 e seu reexport são
públicos, mas o módulo físico e a tabela permanecem privados. Não existe
wrapper, constante, branch linguístico ou segunda tabela no shell.

`None` representa ausência de língua causal no `Content` raw. A forma legacy
cristalina permanece byte-idêntica, enquanto `LocatedContent(Some(fields))`
continua autoritativo e conserva, por exemplo, o snapshot alemão
`Abschnitt`.

O inglês que permanece em `heading_supplement.rs` é o fallback canónico do
owner junto das 45 entradas ratificadas, não uma decisão repetida no
consumer. A varredura final não encontrou sucessor produtivo L2; ocorrências
restantes no shell são expectativas de teste.

## Identidades e campanha pública

- preseal preservado: SHA-256
  `6603c905f705a1ab7d3814a33aeeaf28e4f627429135802ed47d5d76b6209ab4`;
- passo materializado: SHA-256
  `b94fc6a13e9e42885ee9590e0392ad756e541e7a3776caf02cb3c20a9bb0fd96`;
- emenda pós-selo: SHA-256
  `6bc2beae3bea3e676aa7f7d96e5461573bdd5a4153f1d079493438b8cd4e8dc8`;
- contrato/oráculos/mutantes pré-candidato: SHA-256
  `71c146dd672b7b0840aa1ce4fada4a28d94d11b2bd38a86f63f9094c29dff9c5`,
  `a978e1966139d5d8bb5bfe67cad70a2a6d981d2740bb4c1ce015c8d91e62cde7`
  e `ec82313de7519402dd77bc583c6f282e2bb6307d63d0d39f851167df448395bf`;
- vanilla ratificado: SHA-256
  `eb60986b522d9843172cdf318dd46c81f5922109f503ab1733cfe8baaeb1468f`;
- cristalino release reconstruído no produto `3ba006129`: SHA-256
  `f98e658c2e04da5fa4db092656060c2fadf2899356b62d64e45bfe353ed88882`;
- campanha pública reproduzida na árvore commitada: 10/10 casos em forward,
  10/10 em reverse e 10/10 em forward-repeat, sem falhas;
- erro raw também coincidiu byte a byte com o oráculo congelado.

## Testes e ataques

- RED final pré-candidato: somente `E0603`, provando que o consumer L2 ainda
  não alcançava o owner privado;
- `cargo test -p typst-shell p1460_ --quiet`: 2 aprovados, 0 falhas, repetido
  na árvore commitada;
- regressões `cargo test -p typst-core p1459_ --quiet`: 3 aprovadas, 0 falhas;
- `cargo test --workspace --quiet`: o test binary de core conservou 5919
  aprovados e as mesmas 8 falhas históricas, sem falha P1460;
- runner estático/de modelo: 16 ataques rejeitados, 0 sobreviventes, score
  discriminatório 1,0; esse número **não** é apresentado como mutation
  testing compilado;
- mutação compilada manual: 4 mutantes válidos, 4 apanhados, repetidos nas
  três ordens, 12/12 execuções apanhadas, score 1,0;
- recibos versionados: estático
  `f52f8ee90548c940d33a3e4074ab723f1aeadcddf1fb6afef5b90bee1e9fa5db`,
  compilado
  `09a565916c47be5a37842add10a5ea5caae27450148f67de6ddfb2514ec97f04`
  e público
  `e81467c34e7c49f90189aa44f6ea652644079ccb59f78e1c092483e8a7af908e`.

O recibo de mutação compilada resume transformações, testemunhas e três
ordens, mas não incorpora patches/logs brutos. O verificador separado conferiu
aritmética, restauração do SHA-256 do candidato e coerência dos testemunhos;
por mandato somente leitura, não reinjetou os mutantes.

## Gates da árvore certificada

- release e `cargo build --workspace`: sucesso;
- `cargo fmt --all -- --check`: sucesso;
- `git diff --check`: sucesso;
- todos os JSON P1460: válidos;
- runner estático reproduzido: 16/16, zero sobreviventes;
- `crystalline-lint .`: exit 0, sem violations e sem bloqueio V5/V15/V26;
- working tree limpa antes da criação deste certificado.

A revisão final por verificador separado, ainda sem atestação de isolamento,
reproduziu os hashes dos transcripts, validou os três pares L0↔consumer, a
API mínima, a precedência do snapshot, a ausência de sucessor L2 e os limites
do veredito. Não encontrou inconsistência bloqueante.

## Não verificado e não fechado

1. P1460 não muda a forma raw para a forma menor do vanilla; essa divergência
   permanece deliberadamente fora do recorte P1307-R5.
2. A causalidade introduzida apenas por show-set continua na fronteira
   pré-show/render e requer passo próprio mais confirmação ADR-0127.
3. O passo não remove outros defaults sintéticos de Figure, Equation ou de
   outras famílias de `Content`, nem alega eliminação global de toda string
   literal do repositório.
4. O helper agora é API pública aditiva. Sua remoção futura exige tratamento
   explícito de compatibilidade.

Logo, **VERIFIED** significa somente: o hardcode produtivo de suplemento raw
de heading em L2 foi eliminado por delegação ao owner L1 mínimo, com outputs
e snapshots contratados preservados. Não significa paridade raw universal,
fecho de show-set ou cadeia integralmente isolada.

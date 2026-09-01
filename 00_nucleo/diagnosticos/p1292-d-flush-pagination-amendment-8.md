# P1292 — amendment-8: paginação causal de `place.flush`

**Papel:** autor segregado do contrato + auditor de ownership

**Manifest autorizado:**
`f81636bbdeb3a3730a86678c8513e04724a6be763f21a98a0d0a459aef3f6978`

**Predecessor v8:** canônico
`d11d47b51063a09157cf47f5b52f92fc50d0bd81e2c45f6b25b8a1c8f75c828b`,
seal `5c8c229db061fec8a97c9f049e28f70d85e47d9dc76eb64998f4b09c354a3e6a`.

**Proveniência:** `HEAD`
`0eb39f8ecb48930515f2cadb6a378450855b5a72`, working tree não commitada.
Medição/gate inicial em `2026-09-01T02:59:39-03:00`, 39 arquivos,
1.820 inserções e 341 remoções no `git diff HEAD --stat`; após os L0s, em
`2026-09-01T03:00:32-03:00`, 40 arquivos, 1.897 inserções e 341 remoções.

Binários: vanilla ratificado `/usr/local/bin/typst` SHA-256
`7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8`;
candidato direto `target/debug/typst` SHA-256
`c312e4f9c222aca7ae4b540a3f16bfbd09e495bc8c66a419b9ab4dfe5df923f9`.
Implementação refutada `flush.rs` SHA-256
`6e6f7b8474dce6de92d4d456b295f2e4036f9c6b12e3f91c15e00592d1960d51`;
cursor baseline
`120f04f65f53aa431f49b31288481466f4ac6bcae86815b82fc13db1ba17e932`.

## Medição black-box independente

Cada fixture usa página 100pt × 100pt e tokens visíveis. Compilação e leitura:

```text
<binário> compile <fixture.typ> <resultado.pdf>
pdftotext -bbox <resultado.pdf> -
pdfinfo <resultado.pdf>
```

| Caso | Vanilla ratificado | Candidato drain direto | Decisão |
|---|---|---|---|
| prefix/suffix | 3 páginas: PRE p1, float-prefixo p2, AFTER + float-sufixo p3 | 1 página, todos sobrepostos/antecipados | refuta drain direto |
| no-flush | 2 páginas; AFTER permanece p1, float p2 | 1 página; float e AFTER p1 | controle causal do marker |
| top+bottom | 2 páginas; top+flow p1, bottom+AFTER p2 | 1 página, top/flow sobrepostos | exige distribuidor normal |
| no-floats | 1 página e idêntico ao controle sem marker em cada renderer | idem | no-op obrigatório |
| nested | 1 página e idêntico ao controle sem marker em cada renderer | idem | marker não alcança outer |
| fronteira | 2 páginas com ou sem marker; vanilla move float/AFTER a p2 | candidato mantém float/AFTER na p1 visual, embora haja p2 vazia | sem quebra artificial; placement continua causal |

Na fixture principal, `pdftotext -bbox` mediu no vanilla: `PREFLOW` p1,
`FLOATPREFIX` p2, `AFTERFLUSH` p3 e `FLOATSUFFIX` p3. No candidato os quatro
ficaram na p1. Os valores `yMin` próprios desses tokens foram, respectivamente,
`-2.596`, `90.166`, `-2.596`, `90.166` no vanilla e `-2.596`, `10.166`,
`20.166`, `75.778` no candidato. Eles documentam esta fixture independente;
não substituem as coordenadas exatas já congeladas nos vetores D do seal.

Hashes das nove fixtures:

```text
boundary-no-flush.typ 502524da13a4433d11dd69acc61cdb7274dd248d333d39a91ffde75388063853
boundary.typ          002f495a418b67d695be822f0c31e63472d7a3f41d3e2461236aa0cd076bf464
nested-no-flush.typ   93ffe564c3863538ade281554a4efe2c7566243b783f877971dde8c1a32cdbc3
nested.typ            9c0e851c31cb655d5a7e36b458e9e2b142173b405542f3f9a59aea50170748fd
no-floats-control.typ 68e801544afe3946f14f0ce29b3d048bc25dbd7fc5e2d07b9c3802e5c4ee6e36
no-floats.typ         1a5cb614d7d0500c0d01e3f9d450181f42153fa2ba54756a04365ca36a86408b
no-flush.typ          dafc1fc532a3094845fcddda4ca27f941b64e8052b18d1b2ee12ed8bf50a72ab
prefix-suffix.typ     f8a6e82d474625611821a80f71ccaf90e167ae96a5a2e32f65632fec1c0d9028
top-bottom.typ        b85fb04642e61967d60979a23775df0ae4538103098151cbf9ae5f47a6c184a0
```

## Mecânica medida e classificação

No cursor cristalino baseline, `new_page` chama `flush_pending_floats` antes
de fechar a página; o drain toma o buffer integral e o emite na região
corrente. Chamá-lo no marker não avança o distribuidor, exatamente a falha
observada.

No upstream ratificado, SHA-256
`36f0732a6004d189024da6dfade8767f9f53ad0cacc346211d0b2d3cf3c9b09b`,
`flow/distribute.rs:514-521` pede `Finish(false)` quando o marker encontra
floats pendentes. `collect.rs:79-91`, SHA-256
`0baa7e20f1b2e1bfb5d7901ce24f90073fd53648281f4b41c25b090d43318b11`,
conserva o marker como child terminal do flow. A linguagem exige o efeito
causal terminar/avançar até realizar o prefixo; `Stop` e a composição upstream
são mecânica, não API a copiar (ADR-0107/0108).

## Decisão mínima e ownership

`compiler/layout/flush.md` especifica a política da sentinela: captura a
fronteira por ocorrência e chama somente
`finish_float_prefix_at_marker(prefix_boundary)`. Não chama drain direto,
`flush_line`, `new_page` incondicional nem contém distribuição.

`compiler/layout/cursor.md`, owner real de `cursor.rs`, especifica o hook no
distribuidor paginado existente: enquanto uma ocorrência do prefixo continuar
pendente, termina/avança a região ativa; cada região realiza apenas o prefixo
elegível e conserva o restante para a seguinte. Flow posterior retoma somente
após o prefixo; floats criados depois nunca são antecipados. Nested usa somente
o distribuidor ativo.

Isto permanece na fase layout paginada existente. Não cria item, passagem,
assinatura pública, wrapper ou segundo distribuidor. `compiler/layout.md`
continua correto como owner do dispatch magro e não foi alterado.

Refutadores: emissão integral na página corrente; quebra incondicional;
algoritmo duplicado em `flush.rs`; flush de linha; item próprio; suffix
antecipado; mudança top/bottom/clearance; ou acesso ao outer. Float maior que
região vazia e combinações não medidas permanecem `Unknown`, nunca justificam
loop sem progresso ou relaxamento dos vetores.

## L0s e paragem

```text
compiler/layout/flush.md
431e05c4baac680540b76063fe34f39907bd874e25602dd118731eeefd2126cf

compiler/layout/cursor.md
4f0084b06f682f3f8b8484ea4c31a54aa63bc84f699e7693db81accfd476c019

compiler/layout.md (auditado, inalterado)
a0b310323c44777a0a906ea09f620221d1e0ec0d85d6cb920436ca275bae0da4
```

O grafo passa de 23 para 24 owners. Nenhum código, teste, oráculo, ataque ou
veredito foi editado; `--fix-hashes` não foi usado. **PARAGEM.**

# Relatório — Passo 931: cache de fontes em ficheiro único, lookup direto na cmap e índice de intervalos

**Data de execução:** 2026-07-30T20:39:53-03:00  
**Commit base:** `acef3e88d0d5d7cc62c06dfbb741590aca2cc331`  
**Estado final da árvore:** protótipos revertidos; apenas este relatório, o passo original e os entregáveis de performance (`tools/perf/benchmark-p931-braco*.py` + `tools/perf/results/p931-braco*/`) permanecem como artefactos não rastreados.

---

## 1. Resumo executivo

Nenhum dos três braços testados, nem as combinações 1+2 ou 1+3, cumpre os critérios de aceitação definidos no passo 931. O **emoji puro** (`utf8-emoji.typ`) continua a ser o veto decisivo: nenhuma variante consegue melhoria ≥ 20 % sem regressão colateral em `05-utf8.typ` ou no caso comum.

| Braço | Ideia central | Emoji (vs. P932-lazy) | CJK (vs. P932-lazy) | 05-utf8 (vs. P932-lazy) | Caso comum | Veredicto |
|---|---|---|---|---|---|---|
| 1 | Cache único de metadados/coverage em disco | Quente 0,94× (6 % melhor) | Quente 0,36× (2,8× mais rápido) | Quente 1,04× (4 % pior) | Quente 0,64–0,90× | **Rejeitado** — custo frio proibitivo; ganho quente abaixo do gate de 50 % |
| 2 | Lookup direto na cmap dos sobreviventes | 1,61× (61 % pior) | 0,59× (1,7× mais rápido) | 1,69× (69 % pior) | ~1,0× | **Rejeitado** — emoji e 05-utf8 regredem |
| 3 | Índice de intervalos (array ordenado) | Quente 1,03× (3 % pior) | Quente 0,24× (4,2× mais rápido) | Quente 0,80× (20 % melhor) | Quente 0,75–0,97× | **Rejeitado** — emoji não atinge 20 %; custo frio proibitivo |
| 1+2 | Cache + lookup cmap | Não medido diretamente | — | — | — | **Não justifica** — Braço 2 já falha sozinho |
| 1+3 | Cache + índice de intervalos | Não medido diretamente | — | — | — | **Não justifica** — Braço 3 não resolve emoji |

**Decisão final:** registar como **limitação aceite** (formato P929, Fase C). Não avança para passo de implementação. A solução exige abordagem diferente — provavelmente cache de lookup de codepoint a nível de cluster/text run, ou otimização do caminho de fallback do shaper, em vez de estruturas globais de cobertura.

---

## 2. Proveniência e binários

| Papel | Caminho | SHA-256 |
|---|---|---|
| Referência P932-lazy | `target-original/release/typst` | `8446552fa49a24220021e2ac2601a6b7c09322ee024507412b14be855dbb2072` |
| Braço 1 | `target/tmp/typst-braco1` | `ccabad3c9f26fa22e5e2e8ff82c67201703d30810355309c46a82a31c7d4e036` |
| Braço 2 | `target/release/typst` | `b7a6f893a5df4a18624c05ff8359198f6f17640ba82974753c4728ba13582ba5` |
| Braço 3 | `target/release/typst` | `df9f0d0b6b08cd10837530b150d09a4751303d956e29435fce21d07b76ff5cc8` |

**Scripts:** `tools/perf/benchmark-p931-braco1.py`, `tools/perf/benchmark-p931-braco2.py`, `tools/perf/benchmark-p931-braco3.py`.  
**Atestações:** `tools/perf/results/p931-braco1/attestation.json`, `tools/perf/results/p931-braco2/attestation.json`, `tools/perf/results/p931-braco3/attestation.json`.

---

## 3. Resultados detalhados

### 3.1 Braço 1 — cache único de metadados/coverage

O protótipo escreve um ficheiro único `target/tmp/font-cache/fonts.cache` com `FontBook` serializado + coverage por fonte. A validação quente usa `stat()` por ficheiro de fonte. O binário foi preservado em `target/tmp/typst-braco1`; o código fonte foi revertido.

#### Canônicos — run fria (cache apagado antes de cada execução)

| Documento | P932-lazy (ms) | Braço 1 (ms) | Rácio |
|---|---:|---:|---:|
| 01-hello | 99,05 | 739,41 | 7,46× |
| 02-lorem | 112,06 | 653,69 | 5,83× |
| 03-images | 95,28 | 635,36 | 6,67× |
| 04-math | 148,53 | 793,84 | 5,34× |
| 05-tables | 105,04 | 760,70 | 7,24× |
| 06-long | 323,56 | 897,67 | 2,77× |
| 07-context | 134,38 | 697,18 | 5,19× |

O custo frio é de ~600–900 ms, consistente com a construção do cache (descoberta + scan de coverage + serialização + escrita atómica). Documentos maiores (`06-long`) amortizam parte do custo porque o próprio trabalho de layout domina.

#### Canônicos — run quente (cache preenchido)

| Documento | P932-lazy (ms) | Braço 1 (ms) | Rácio | Ganho |
|---|---:|---:|---:|---:|
| 01-hello | 92,51 | 59,37 | 0,64× | 36 % |
| 02-lorem | 117,15 | 82,42 | 0,70× | 30 % |
| 03-images | 99,31 | 66,83 | 0,67× | 33 % |
| 04-math | 155,89 | 123,67 | 0,79× | 21 % |
| 05-tables | 98,86 | 65,52 | 0,66× | 34 % |
| 06-long | 327,30 | 295,55 | 0,90× | 10 % |
| 07-context | 137,53 | 107,03 | 0,78× | 22 % |

A run quente melhora, mas **não atinge o gate de 50 % de redução do arranque** exigido pelo critério 2. O ganho é real, mas insuficiente para justificar a superfície de engenharia.

#### UTF-8 — run fria

| Documento | P932-lazy (ms) | Braço 1 (ms) | Rácio |
|---|---:|---:|---:|
| 05-utf8 | 8334,91 | 8966,49 | 1,08× |
| utf8-latin | 89,34 | 777,44 | 8,70× |
| utf8-greek | 96,50 | 720,83 | 7,47× |
| utf8-cjk | 7224,02 | 2646,06 | 0,37× |
| utf8-emoji | 7348,84 | 7348,72 | 1,00× |

#### UTF-8 — run quente

| Documento | P932-lazy (ms) | Braço 1 (ms) | Rácio |
|---|---:|---:|---:|
| 05-utf8 | 7480,79 | 7763,85 | 1,04× |
| utf8-latin | 88,46 | 69,79 | 0,79× |
| utf8-greek | 88,22 | 82,11 | 0,93× |
| utf8-cjk | 7161,32 | 2566,18 | 0,36× |
| utf8-emoji | 8102,29 | 7651,61 | 0,94× |

**Observação:** CJK continua a beneficiar-se massivamente do cache (o fallback dispara e a coverage já está materializada), mas emoji puro não melhora de forma significativa — o tempo continua dominado por outros fatores (parse de faces, shaping, ou a própria complexidade da cmap emoji).

---

### 3.2 Braço 2 — lookup direto na cmap dos sobreviventes

O protótipo filtra os candidatos do bitmap e, para cada sobrevivente, faz lookup exato do codepoint na cmap (format 4/12). Não houve cache de metadados em disco — só memoização em memória do par `(font_id, char)`.

#### Canônicos

| Documento | P932-lazy (ms) | Braço 2 (ms) | Rácio |
|---|---:|---:|---:|
| 01-hello | 100,19 | 96,08 | 0,96× |
| 02-lorem | 117,58 | 119,58 | 1,02× |
| 03-images | 98,35 | 101,85 | 1,04× |
| 04-math | 155,06 | 156,80 | 1,01× |
| 05-tables | 97,24 | 100,56 | 1,03× |
| 06-long | 318,16 | 320,70 | 1,01× |
| 07-context | 140,24 | 142,41 | 1,02× |

Caso comum intacto — todos dentro da banda de ruído de 1,05×.

#### UTF-8

| Documento | P932-lazy (ms) | Braço 2 (ms) | Rácio |
|---|---:|---:|---:|
| 05-utf8 | 8233,53 | 13952,87 | 1,69× |
| utf8-latin | 93,90 | 96,06 | 1,02× |
| utf8-greek | 94,26 | 100,05 | 1,06× |
| utf8-cjk | 7778,86 | 4578,42 | 0,59× |
| utf8-emoji | 8578,09 | 13824,54 | 1,61× |

**Veredicto:** o lookup direto na cmap **elimina falsos positivos**, mas o custo de fazer esse lookup para cada um dos 238 candidatos do emoji é maior do que o custo de simplesmente parsear as faces no caminho original. CJK ganha porque tem poucos candidatos reais (32), mas emoji e `05-utf8` (que inclui emoji) pioram de forma clara. **Braço 2 rejeitado.**

---

### 3.3 Braço 3 — índice de intervalos (array ordenado)

O protótipo constrói, no momento de criação do cache do Braço 1, um array ordenado de intervalos `[start, end]` por fonte a partir da cmap. O lookup usa busca binária. A integração com o Braço 1 foi medida no mesmo binário (o índice é serializado dentro do ficheiro único de cache).

#### Canônicos — run fria

| Documento | P932-lazy (ms) | Braço 3 (ms) | Rácio |
|---|---:|---:|---:|
| 01-hello | 91,21 | 716,60 | 7,86× |
| 02-lorem | 113,57 | 782,11 | 6,89× |
| 03-images | 100,88 | 855,69 | 8,48× |
| 04-math | 153,97 | 861,25 | 5,59× |
| 05-tables | 96,18 | 744,65 | 7,74× |
| 06-long | 308,17 | 957,92 | 3,11× |
| 07-context | 132,02 | 757,45 | 5,74× |

Custo frio semelhante ao Braço 1 (~700–950 ms), como esperado, porque a construção do índice acrescenta trabalho à criação do cache.

#### Canônicos — run quente

| Documento | P932-lazy (ms) | Braço 3 (ms) | Rácio |
|---|---:|---:|---:|
| 01-hello | 91,05 | 67,95 | 0,75× |
| 02-lorem | 112,36 | 109,47 | 0,97× |
| 03-images | 93,26 | 89,65 | 0,96× |
| 04-math | 152,32 | 128,16 | 0,84× |
| 05-tables | 92,99 | 72,48 | 0,78× |
| 06-long | 303,52 | 282,32 | 0,93× |
| 07-context | 129,21 | 108,81 | 0,84× |

O caso comum quente mantém-se dentro da banda de ruído (≤ 1,05×), com ganhos modestos em alguns documentos.

#### UTF-8 — run fria

| Documento | P932-lazy (ms) | Braço 3 (ms) | Rácio |
|---|---:|---:|---:|
| 05-utf8 | 7808,61 | 7102,73 | 0,91× |
| utf8-latin | 89,21 | 744,37 | 8,34× |
| utf8-greek | 91,60 | 876,82 | 9,57× |
| utf8-cjk | 7285,12 | 2347,29 | 0,32× |
| utf8-emoji | 8107,51 | 7144,76 | 0,88× |

#### UTF-8 — run quente

| Documento | P932-lazy (ms) | Braço 3 (ms) | Rácio |
|---|---:|---:|---:|
| 05-utf8 | 8290,27 | 6653,40 | 0,80× |
| utf8-latin | 88,70 | 67,11 | 0,76× |
| utf8-greek | 89,77 | 68,71 | 0,77× |
| utf8-cjk | 7190,28 | 1696,88 | 0,24× |
| utf8-emoji | 7858,08 | 8083,13 | 1,03× |

**Observação:** CJK melhora drasticamente (0,24× quente), `05-utf8` melhora 20 %, mas **emoji puro melhora apenas 12 % no frio e piora 3 % no quente**. Não atinge o gate de 20 % no emoji. **Braço 3 rejeitado.**

---

## 4. Matriz de decisão (Fase D do passo)

### Critérios aplicados

1. **Caso comum ≤ 1,05×:**
   - Braço 1: quente ✔; frio ✘
   - Braço 2: ✔
   - Braço 3: quente ✔; frio ✘
2. **Arranque quente ≥ 50 % de redução (Braço 1):**
   - Melhor resultado: 01-hello 92,5 → 59,4 ms = 36 % ✘
3. **Fallback: melhoria ≥ 20 % em `utf8-emoji` sem regressão em `utf8-cjk` nem `05-utf8`:**
   - Braço 1 quente: emoji 0,94× (6 %), cjk 0,36×, 05-utf8 1,04× ✘
   - Braço 2: emoji 1,61× ✘
   - Braço 3 quente: emoji 1,03× (pior), cjk 0,24×, 05-utf8 0,80× ✘
4. **Robustez do cache:** não foi medida formalmente neste passo porque o Braço 1 já falha nos critérios de performance.

### Conclusão da matriz

Nenhum braço cumpre todos os critérios. As combinações 1+2 ou 1+3 não mudam o veto do emoji (Braço 2 piora emoji; Braço 3 não melhora emoji ≥ 20 %). Logo, **não há passo de implementação derivado deste passo**.

---

## 5. Modelo de ameaça do cache (Fase E do passo)

Taxonomia da validação por "tripla gorda" (metadados `stat()` / `GetFileAttributesEx`):

| Classe | Exemplo | Coberto? |
|---|---|---|
| Acidente comum | atomic replace (write tmp + rename) | ✔ inode / creation time |
| Acidente comum | restauro de backup (`rsync -a`, `cp -p`) | ✔ ctime (Unix) / creation time (Windows) |
| Acidente comum | update de pacote | ✔ tamanho / mtime |
| Acidente raro | janela de cache de atributos NFS/SMB | ✘ residual, **auto-curativo** (próxima validação vê) |
| Acidente raro | granularidade FAT/exFAT (2 s) + mesmo tamanho | ✘ residual, auto-curativo |
| Acidente raro | build reproduzível com timestamp pinado e mesmo tamanho | ✔ ctime, salvo colisão no mesmo segundo |
| Forja deliberada | escrita in-place + `utimens`/`SetFileTime` | ✔ ctime no Unix; ✘ **residual no Windows** (sem admin necessário) |
| Forja deliberada | root + relógio do sistema | ✘ fora do modelo de ameaça |

**Enquadramento de severidade:** o dano máximo de um cache stale é um PDF compilado com uma versão obsoleta de uma fonte — um **bug de reprodutibilidade, não de segurança**. A maioria das classes é auto-curativa. O modo paranoico (`--verify-font-cache` com hash forte de conteúdo) fica registado como escape hatch futuro para contextos de pré-impressão/jurídico; o seu custo (re-ler todos os bytes de todas as fontes) justifica não ser o default.

---

## 6. Armadilhas e notas operacionais

- ✔ Não se recaiu na granularidade ficheiro-por-coverage (P929 Ideia 2).
- ✔ Não se preencheu coverage/índice no arranque para o caso comum (P930); o custo foi pago só no caminho frio.
- ✔ Não se paralelizou o scan (P929 Ideia 1).
- ✔ Estado do cache declarado em cada tabela (frio/quente).
- ✔ SHA-256 dos binários registados.
- ✔ **wasm está fora de escopo** — sem fontes de sistema, o cache de fontes não se aplica.
- ✔ **.ttc:** a identidade da entrada de coverage é `(path, face_index)`; a validação continua a ser por ficheiro.

---

## 7. Validação final da árvore

- `cargo build --workspace --release`: ok.
- `cargo test -p typst-infra --lib`: 743 passed, 0 failed.
- `crystalline-lint .`: 0 violations (apenas V7 pré-existente, `package_version_resolution.md`).
- `target/tmp/font-cache/` foi apagado pelos scripts de benchmark.

---

## 8. Recomendação para passos futuros

A limitação aceite deste passo aponta para duas direções não exploradas:

1. **Cache de resolução a nível de cluster/text run**, não de cobertura global. O emoji puro sugere que o custo dominante não é a descoberta de candidatos, mas a resolução repetida dos mesmos codepoints dentro do processo de shaping.
2. **Investigação do caminho de fallback do shaper**: o facto de CJK melhorar drasticamente enquanto emoji estagna indica que o padrão de acesso às faces emoji é diferente (mais faces dispersas, cmap maiores, ou maior custo de parse).

Ambas as direções exigiriam novo passo de investigação com protótipos temporários e medição, antes de qualquer código de produção.

---

## 9. Entregáveis

1. Este relatório.
2. Scripts: `tools/perf/benchmark-p931-braco1.py`, `tools/perf/benchmark-p931-braco2.py`, `tools/perf/benchmark-p931-braco3.py`.
3. Atestações: `tools/perf/results/p931-braco1/attestation.json`, `tools/perf/results/p931-braco2/attestation.json`, `tools/perf/results/p931-braco3/attestation.json`.
4. Binário preservado do Braço 1: `target/tmp/typst-braco1` (o restante código foi revertido).

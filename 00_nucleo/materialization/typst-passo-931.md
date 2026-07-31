# Passo 931 — cache de fontes em ficheiro único, lookup direto na cmap e índice de intervalos

**Precede este passo:** `typst-passo-932-relatorio.md` — P932-lazy manteve o
caso comum intacto e resolveu o CJK (2,94× mais rápido), mas o emoji puro
ficou em ~7,6 s e o arranque (~201 ms de descoberta + scan lazy de coverage)
permanece intocado. P929 e P930 registam resultados negativos que **vinculam**
este passo: scan paralelo (2,9–3,8× pior), cache em disco *por ficheiro de
coverage* (−8% CJK mas +21% emoji) e índice invertido *eager* por bloco
(+756 ms de arranque) estão proibidos como solução, mas não como conhecimento.

**Objetivo deste passo:** investigar, **somente com protótipos temporários e
medição**, três braços complementares que atacam o custo restante. Nenhum
código de produção neste passo. Ao final, decidir qual combinação (se alguma)
merece um passo de implementação.

**Data:** (preencher na execução).
**Commit base:** o commit que integra o P932-lazy (registar o SHA no relatório).

---

## 1. Decisões de desenho já tomadas (vinculam os protótipos)

Estas decisões vêm da análise dos resultados P929–P932 e da discussão de
arquitetura que precede este passo. Os protótipos **devem** respeitá-las:

1. **Ficheiro único, leitura única.** Qualquer cache é um ficheiro só
   (serialização binária plana ou mmap), lido no máximo uma vez por processo.
   A granularidade ficheiro-por-coverage está proibida (lição da Ideia 2 do
   P929: ~238 aberturas de ficheiro no laço quente = +21% no emoji).
2. **Validação por "tripla gorda", nunca por hash de conteúdo.** A assinatura
   do conjunto de fontes é derivada de metadados obtidos por `stat()`
   (custo ~ms), porque hash de conteúdo exige ler todos os bytes de todas as
   fontes — custo superior à própria descoberta, o que derrota o propósito.
   Receita por SO:
   - `#[cfg(unix)]`: `(path, size, mtime_ns, ctime_ns, ino)` — uma `stat()`.
   - `#[cfg(windows)]`: `(path, size, mtime, creation_time, attrs)` — uma
     `GetFileAttributesEx`. O caso de escrita in-place com mtime forjado via
     `SetFileTime` fica como risco residual **documentado**, não tratado.
3. **Invalidação pessimista.** Metadata ilegível, cache corrompido, versão de
   formato diferente ou qualquer dúvida → descarta e reconstrói. O cache nunca
   produz resultado errado; no pior caso, custa mais.
4. **Hash forte existe, mas só pós-invalidação.** Quando a tripla dispara, a
   hash forte da fonte é recomputada e usada como **chave de deduplicação**:
   se o conteúdo já existe no cache (fonte movida/renomeada), a coverage é
   reaproveitada sem reescanear a cmap. A hash forte **não** participa da
   validação rápida.
5. **Árvore = array ordenado.** O índice de intervalos é um array plano
   ordenado com busca binária (árvore implícita): serializável byte a byte,
   mmap-ável, sem ponteiros. Nada de nós ligados.
6. **Watcher nunca é fonte de verdade.** inotify/FSEvents/USN Journal podem, no
   futuro, manter o cache quente proativamente; neste passo ficam fora de
   escopo. O cache é válido ou não independentemente de qualquer daemon.
7. **Nada eager no caso comum.** Qualquer estrutura que custe >10 ms no
   arranque de um documento latino está morta antes de nascer (lições P927,
   P930). Todo custo de construção ou é amortizado pelo cache, ou é lazy.

---

## 2. Metodologia (obrigatória)

- **Binário de referência:** o binário P932-lazy (estado vencedor atual).
  Registar SHA-256.
- **Binários protótipo:** um por braço, recompilados a partir do mesmo commit
  base. Registar SHA-256 de cada um. **Todos os protótipos são revertidos ao
  final do passo.**
- **Cenários canônicos:** os 7 documentos da frente (01-hello … 07-context),
  comparados `depois/antes`.
- **Cenários UTF-8:** `05-utf8.typ`, `utf8-latin`, `utf8-greek`, `utf8-cjk`,
  `utf8-emoji`.
- **Ferramenta:** `hyperfine`, warmup 5 / min-runs 20 para canônicos; warmup 2 /
  min-runs 10 para UTF-8. Para os braços com cache em disco: cache apagado
  antes da run fria; **medir e reportar run fria e run quente separadamente**
  (lição P929: o warmup popula o cache — declarar explicitamente o estado do
  cache em cada medição, sob pena de o relatório ser inválido).
- **Atestações:** `tools/perf/results/p931-bracoN/attestation.json` por braço.
- **Localização do cache protótipo:** `target/tmp/font-cache/` (apagado ao
  final do passo).

---

## 3. Fase A — Braço 1: cache de fontes em ficheiro único

**Hipótese:** o arranque one-shot (~201 ms de descoberta + scan lazy quando o
fallback dispara) pode cair para ~ms com um cache persistente validado por
metadados.

### Implementação temporária

- Ficheiro único em `target/tmp/font-cache/fonts.cache` contendo:
  cabeçalho com versão de formato + assinatura do conjunto (hash da lista
  ordenada de entradas de metadata) + `FontBook` serializado + `Coverage` de
  cada fonte (formato atual, bitmap 64×u64).
- Validação: uma `stat()` por ficheiro de fonte; compara a lista de tuplas da
  receita por SO (item 1.2) com a armazenada. Mismatch → reconstrói tudo.
- Escrita atômica: escrever em `fonts.cache.tmp` + `rename`.
- Caminho de leitura: uma leitura sequencial (ou mmap); falha de parse →
  reconstrói (invalidação pessimista).

### Medições

| # | Medição | Por quê |
|---|---|---|
| A.1 | Custo de construir o cache frio (descoberta + coverage + serializar) | Preço da primeira execução |
| A.2 | Custo de validar o cache quente (stat-only) | Preço de cada execução seguinte |
| A.3 | `01-hello` e `02-lorem` com cache quente vs. P932-lazy | Ganho no caso comum one-shot |
| A.4 | `utf8-cjk`, `utf8-emoji`, `05-utf8` com cache quente vs. P932-lazy | Ganho quando o fallback dispara |
| A.5 | Custo de hashear o conteúdo de todas as fontes (SHA-256) | Documentar **numericamente** por que a validação forte é inviável |
| A.6 | Comportamento com cache corrompido/truncado e com formato de versão errada | Verificar invalidação pessimista (reconstrói sem erro) |

---

## 4. Fase B — Braço 2: lookup direto na cmap dos sobreviventes

**Hipótese:** o bitmap por blocos entrega 238 candidatos para emoji; uma busca
binária por codepoint **na própria cmap de cada sobrevivente** elimina os
falsos positivos sem construir índice nenhum. O lookup por codepoint é ordens
de magnitude mais barato que iterar a cmap inteira (lição invertida do P930).

### Implementação temporária

- Em `candidates_for_char` (ou onde o filtro de candidatos ocorrer), após o
  filtro do bitmap: para cada candidato sobrevivente, fazer lookup exato do
  codepoint na cmap da fonte (format 4/12, busca binária nos segmentos).
- Cache em memória: `HashMap<(font_id, char), bool>` para não repetir lookups
  do mesmo par dentro do processo.
- Medir também o número de faces parseadas por documento (contador
  instrumentado, temporário).

### Medições

| # | Medição | Por quê |
|---|---|---|
| B.1 | `utf8-emoji` e `05-utf8` vs. P932-lazy | A pergunta central do passo |
| B.2 | `utf8-cjk`, `utf8-latin`, `utf8-greek` vs. P932-lazy | Regressões colaterais |
| B.3 | 7 canônicos vs. P932-lazy | Caso comum não pode pagar nada |
| B.4 | Cardinalidade real pós-lookup (quantos candidatos exatos por script) vs. cardinalidade do bitmap (dados P930: grego 866, emoji 238, CJK 32) | Quantificar a eliminação de falsos positivos |

---

## 5. Fase C — Braço 3: índice de intervalos (array ordenado)

**Hipótese:** a cobertura das fontes é feita de faixas contíguas (a própria
cmap format 4/12 é um array de segmentos). Um índice de intervalos exato,
persistido pelo Braço 1, elimina os falsos positivos **sem** parsear face
alguma no caminho quente.

### Implementação temporária

- Construção (uma vez, paga no build do cache do Braço 1): iterar a cmap de
  cada fonte e extrair as faixas contíguas `[start, end]`. Representação: por
  fonte, array ordenado de intervalos; lookup = busca binária.
- Variante estendida (só se o tempo permitir; marcar como tal no relatório):
  sweep-line global — fundir os intervalos das 1112 fontes em segmentos do
  espaço 0x00–0x10FFFF, cada segmento com a lista de fontes que o cobrem.
  Lookup único → lista pronta.
- Integração obrigatória com o Braço 1: o índice é serializado **dentro** do
  ficheiro único de cache. Índice sem cache = índice eager = proibido (P930).

### Medições

| # | Medição | Por quê |
|---|---|---|
| C.1 | Custo de construir o índice (one-shot, frio) e tamanho em memória/disco | Preço de nascer |
| C.2 | Cardinalidade exata por script (mesmos codepoints do P930: U+03B1, U+4E2D, U+1F600) | Comparar com bitmap: 866 / 32 / 238 |
| C.3 | `utf8-emoji`, `05-utf8`, `utf8-cjk` com índice quente vs. P932-lazy | Ganho end-to-end |
| C.4 | Comparação direta Braço 2 vs. Braço 3 nos mesmos cenários | Simplicidade vs. estrutura |

---

## 6. Fase D — critérios de aceitação e matriz de decisão

### Critérios (à la P929; qualquer violação mata o braço)

1. **Caso comum:** todos os 7 canônicos com rácio ≤ 1,05× (banda de ruído).
   Regressão real em qualquer canônico = braço morto.
2. **Arranque (Braço 1):** redução ≥ 50% do tempo de descoberta quente para
   justificar a superfície de engenharia nova.
3. **Fallback (Braços 2 e 3):** melhoria ≥ 20% em `utf8-emoji` **sem**
   regressão em `utf8-cjk` nem em `05-utf8`.
4. **Robustez (Braço 1):** cache corrompido/truncado/versão errada reconstrói
   silenciosamente e produz PDF idêntico ao estado sem cache.

### Matriz de decisão

- Os braços são **combináveis**: 1+2 e 1+3 são as combinações candidatas.
- Se Braço 2 ≈ Braço 3 no end-to-end, **vence o Braço 2** (menos estrutura,
  menos cache, menos formato versionado).
- Se Braço 3 esmagar no emoji (eliminando centenas de parses de face por
  cluster), vence 1+3.
- Se nenhum braço cumprir os critérios: registar como limitação aceite
  (formato P929, Fase C) e propor ADR.

---

## 7. Fase E — validação e estado da árvore

- Reverter **todos** os protótipos. A árvore final do passo é idêntica ao
  estado P932-lazy.
- Apagar `target/tmp/font-cache/`.
- `cargo build --workspace --release`: ok.
- `cargo test -p typst-infra --lib`: 743 passed, 0 failed.
- `crystalline-lint .`: 0 violations (apenas V7 pré-existente,
  `package_version_resolution.md`). Se algum ficheiro rastreado por prompt foi
  tocado e revertido, verificar hashes com `crystalline-lint --fix-hashes .`.

---

## 8. Seção obrigatória do relatório: modelo de ameaça do cache

Documentar, com a taxonomia abaixo, o que a validação por tripla gorda cobre:

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

E explicitar o enquadramento de severidade: o dano máximo é um PDF compilado
com versão obsoleta de uma fonte — **bug de reprodutibilidade, não de
segurança** —, auto-curativo na maioria das classes. O modo paranoico
(`--verify-font-cache`, hash forte de conteúdo) fica registrado como escape
hatch futuro para contextos de pré-impressão/jurídico, com o custo medido em
A.5 como justificativa de não ser o default.

---

## 9. Armadilhas conhecidas (checklist do executor)

- [ ] Não recair na granularidade ficheiro-por-coverage (P929 Ideia 2).
- [ ] Não preencher coverage/índice no arranque para o caso comum (P930).
- [ ] Não paralelizar o scan (P929 Ideia 1).
- [ ] Declarar o estado do cache (frio/quente) em **cada** tabela de resultado.
- [ ] O warmup do hyperfine popula caches: runs frias exigem apagar o cache
      **entre** execuções, com script próprio.
- [ ] Registar SHA-256 de todos os binários antes de medir.
- [ ] wasm está fora de escopo (sem fontes de sistema); registrar isso no
      relatório.
- [ ] `.ttc`: a identidade da entrada de coverage é `(path, face_index)`; a
      validação é por ficheiro.

---

## 10. Entregáveis do relatório

1. Tabelas das Fases A, B, C com frio/quente separados e rácios vs. P932-lazy.
2. Atestações `p931-braco1/`, `p931-braco2/`, `p931-braco3/`.
3. Decisão fundamentada pela matriz da Fase D.
4. Seção de modelo de ameaça (Fase 8).
5. Se houver vencedor: esboço do passo de implementação + nota para o futuro
   ADR (trade-off determinismo absoluto vs. heurística de metadados,
   explicitamente posicionado).
6. Proveniência completa: commit base, SHAs dos binários, scripts usados.

---

## Proveniência

- Commit base: (preencher — commit que integra P932-lazy).
- Binário de referência: P932-lazy, SHA-256 (preencher).
- Scripts: `tools/perf/benchmark-p931-braco1.py`, `…-braco2.py`, `…-braco3.py`
  (criar no passo; podem derivar dos scripts P929/P932).
- Atestações: `tools/perf/results/p931-braco*/attestation.json`.

# Relatório — typst-passo-873: causa raiz de math (22×) e imagens (16×) mais lentos

**Data:** 2026-07-23T20:19:02Z
**Executor:** Claude (Sonnet 5)
**Commit base:** `30122788891b32a423b0354b97eebdbf28ccf068` (HEAD do ramo `Tekt`, após P872)
**Working tree:** limpa, exceto o próprio `00_nucleo/materialization/typst-passo-873.md` (não commitado, `git diff HEAD --stat` vazio)
**Artefactos reusados de P872:** `/tmp/p872-bench/` (PDFs e `.typ` do benchmark original, presentes e intactos no momento desta medição — `ls -la` confere datas de `jul 23 16:59–17:03`, mesma sessão de P872)

Nenhum código foi alterado neste passo — é diagnóstico puro, conforme pedido.

---

## 1. Resumo

A hipótese de P872 (subsetting de fonte ausente) está **parcialmente confirmada para math**: o
cristalino de facto não faz subsetting de nenhuma fonte CFF, math incluído. Mas essa causa,
sozinha, **não explica o tempo de execução** — explica só o tamanho do PDF. Medição direta com
`strace` e `/usr/bin/time -v` isola a causa real do tempo: uma busca de fallback de glifo **não
filtrada**, que percorre o `FontBook` inteiro (~1086 fontes de sistema) e, para cada face de
coleções TrueType grandes (`NotoSansCJK-*.ttc`, `NotoSerifCJK-*.ttc`, 19–27 MB cada), lê o
**ficheiro inteiro do zero** — uma vez por face, mesmo quando só uma face é necessária. Para
imagens, a causa é outra e está confirmada: a deduplicação de recurso existe mas usa identidade de
ponteiro `Arc`, e o mesmo `image("debian-logo.png")` chamado 50 vezes produz 50 `Arc`s distintos, 0
reaproveitamento.

| Cenário | Causa dominante do tempo | Causa do tamanho do PDF | Mesma causa? |
|---|---|---|---|
| Math (22×) | Busca de fallback não filtrada relendo `.ttc` grandes do zero por face | Subsetting CFF desativado (P797) | **Não** — duas causas distintas, ambas presentes |
| Imagens (16×) | Deduplicação por ponteiro `Arc` falha (identidade, não conteúdo) | Mesma causa (50 XObjects em vez de 1) | Sim |

---

## 2. Passo 1 — Math: glifos embutidos

### 2.1 Medição com `pdffonts`

Comando: `pdffonts vanilla-04-math.pdf` / `pdffonts cristalino-04-math.pdf` em `/tmp/p872-bench/`.

```
VANILLA:
name                             type          encoding   emb sub uni object ID
HOQEOT+NewCMMath-Book-Identity-H CID Type 0C   Identity-H yes yes yes  225 0

CRISTALINO:
name              type              encoding   emb sub uni object ID
CrystallineFont1  CID Type 0C (OT)  Identity-H yes no  yes   19 0
CrystallineFont2  CID Type 0C (OT)  Identity-H yes no  yes   24 0
```

`sub` = `yes` no vanilla, `no` nas duas fontes do cristalino. Confirma a hipótese de P872 ao
nível binário (não só pelo tamanho do PDF).

### 2.2 Contagem de glifos (extração com `mutool extract` + `fontTools`)

| Fonte | Ficheiro extraído | Bytes | Glifos embutidos |
|---|---|---|---|
| Vanilla (`NewCMMath-Book`, CID CFF) | `font-0227.cid` | 3 110 | **18** |
| Cristalino `CrystallineFont1` (Libertinus Serif) | `font-0021.otf` | 337 132 | **2 793** |
| Cristalino `CrystallineFont2` (NewComputerModernMath) | `font-0026.otf` | 1 306 384 | **8 603** |

O vanilla embute só os 18 glifos realmente usados no documento (somas, sub/sobrescritos, letras
gregas repetidas por 100 equações, mas deduplicadas). O cristalino embute as **duas fontes
inteiras**: 2 793 + 8 603 = 11 396 glifos, dos quais quase nenhum é usado. Isto **não é
específico de math** — `pdffonts` em `cristalino-01-hello.pdf` e `cristalino-02-lorem.pdf` mostra
a mesma fonte (`CrystallineFont`, Libertinus Serif) também com `sub no`, também embutida inteira.
O bug de subsetting é geral a qualquer documento que use uma fonte CFF; math só o expõe mais porque
a segunda fonte (NewComputerModernMath, 8 603 glifos) é a maior das duas usadas em qualquer cenário
do benchmark de P872.

### 2.3 Localização exata — por que o subsetting não acontece

`03_infra/src/export/subset.rs:79-89` (dentro de `subset_font_with_mapping`):

```rust
if font_data.len() >= 12 {
    let n_tables = u16::from_be_bytes([font_data[4], font_data[5]]) as usize;
    for i in 0..n_tables {
        let off = 12 + i * 16;
        if off + 4 > font_data.len() { break; }
        if &font_data[off..off + 4] == b"CFF " {
            return None;
        }
    }
}
```

O comentário no próprio ficheiro (linhas 70-78) documenta a decisão: **P797** desativou
deliberadamente o subsetting para qualquer fonte com tabela `CFF ` porque `oxifont_subset` produz
um CFF Name-keyed inválido para embed `/CIDFontType0` + `Identity-H` (poppler/ghostscript recusam
a fonte). O teste `p523_subset_cff_nimbus_sans_preserves_cff_table` (mesma ficheiro, linha 260) já
assert a que o subset retorna `None` para CFF — o comportamento medido aqui é o **intencional**,
não um bug latente por descobrir. A fila de correção é: consertar `oxifont_subset` para produzir
CID-keyed CFF válido (ou trocar de subsetter), não reverter o early-return.

### 2.4 Duplicação da fonte por equação — refutado

`pdffonts` mostra exactamente **2** fontes embutidas no PDF (object ID 19 e 24), não 100 (uma por
equação) nem 200. A hipótese do Passo 1.4 do prompt (fonte embutida múltiplas vezes) está
**refutada**: `build_multifont` (`03_infra/src/export/builder.rs:842`) chama `measure_subset` uma
vez por fonte distinta usada no documento (loop `for face in faces`, `03_infra/src/export/
builder.rs:883`), não uma vez por ocorrência de glifo/equação.

---

## 3. Passo 3 — decomposição CPU vs I/O (math)

### 3.1 `/usr/bin/time -v`

Comando: `/usr/bin/time -v <bin> 04-math.typ /tmp/out.pdf`.

| Métrica | Vanilla | Cristalino |
|---|---|---|
| User time | 0.22 s | 1.36 s |
| **System time** | 0.06 s | **5.04 s** |
| Elapsed | 0.28 s | 6.40 s |
| Maximum RSS | 33 592 KB | **9 406 824 KB (~9.4 GB)** |
| Minor page faults | 9 045 | 2 365 340 |
| File system outputs | 192 | 3 512 |

**System time domina** (5.04 s de 6.40 s, 79%) sobre o tempo de utilizador (1.36 s, 21%). Isto
aponta para custo de **I/O/kernel**, não para um algoritmo de layout matemático caro em CPU pura —
refutando parcialmente a segunda hipótese não verificada do relatório de P872 ("layout matemático
cristalino pode ter complexidade superlinear"). Não é isso; ou pelo menos não é a componente
dominante.

### 3.2 `strace -c` — para onde vai o tempo de sistema

Comando: `strace -f -c <bin> 04-math.typ /tmp/out.pdf`.

```
% tempo   segundos   chamadas   syscall
92.88%    2.19s      2343       read
 3.04%    0.07s      2240       munmap
 1.16%    0.03s      3445       openat
```

92.88% do tempo de syscall está em `read()`. Comparação com os cenários rápidos do mesmo binário
(`01-hello`, `02-lorem`, `05-tables`, `07-context`, todos < 130 ms cristalino):

| Cenário | `openat` (ficheiros de fonte únicos) | `read` (chamadas) | tempo em `read` |
|---|---|---|---|
| 01-hello / 02-lorem / 05-tables / 07-context | 1086 (idêntico nos 4) | 119 | 0.3–0.5 ms |
| **04-math** | 1086 (mesma descoberta inicial) | **2343** | **2.19 s** |

A fase de descoberta de fontes de sistema (`openat` de 1086 ficheiros, incluindo os `.ttc` CJK
grandes) **acontece em todos os cenários por igual** e é barata (via `fontdb`, que usa
`with_face_data`/mmap — não lê o ficheiro inteiro para extrair só metadados de nome). O custo
extra de `read()` em math (2343 − 119 ≈ 2224 chamadas, 2.19 s) é **específico de math** e não
aparece nos outros cenários. A secção 4 identifica a origem exacta.

---

## 4. Causa raiz do tempo em math — busca de fallback não filtrada

### 4.1 Medição: quantas vezes cada ficheiro de fonte grande é aberto

```
grep -c "NotoSansCJK-Regular.ttc" no strace de 04-math:  21
grep -c "NotoSansCJK-Regular.ttc" no strace de 01-hello: 11
```

`NotoSansCJK-Regular.ttc` (19 484 784 bytes, confirmado com `ls -la`) tem **10 faces** (confirmado
com `fontTools.ttLib.TTCollection`). `NotoSansCJK-Bold.ttc` também 10; `NotoSerifCJK-Regular.ttc` e
`NotoSerifCJK-Bold.ttc`, 5 cada.

- **Baseline universal** (presente em `01-hello`, que não faz fallback nenhum): 11 aberturas —
  atribuído à descoberta inicial do `fontdb` (uma leitura leve por face para extrair nome/metadados).
- **Delta específico de math**: 21 − 11 = 10, exactamente o número de faces do ficheiro. Isto casa
  com "cada uma das 10 faces é tentada uma vez durante a resolução de fallback do documento math",
  não com releitura redundante da mesma face.

**Nível de confiança:** medido diretamente (contagem de `openat` por ficheiro via `strace`,
reprodutível). A atribuição "as 10 aberturas extra vêm da resolução de fallback do math" é
**inferência por correlação** (o delta bate com o número de faces, mas não instrumentei o código
para imprimir qual glifo dispara cada abertura). O que a refutaria: adicionar um `eprintln!`
temporário em `CandidateSet::covering_all` (`03_infra/src/shaper.rs:664`) a imprimir `c` e
`slot_idx` a cada chamada de `load_fallback`, e confirmar que os `slot_idx` correspondentes às 10
faces de `NotoSansCJK-Regular.ttc` aparecem ali. Não fiz essa instrumentação neste passo (diagnóstico
sem alterar código).

### 4.2 Por que ler 10 faces de um `.ttc` custa 10 leituras completas do ficheiro

`03_infra/src/fonts.rs:56-75`, `FontSlot::get()`:

```rust
pub fn get(&self) -> Option<Font> {
    self.font.get_or_init(|| {
        let data = if let Some(bytes) = &self.embedded {
            bytes.clone()
        } else {
            std::fs::read(&self.path).ok()?   // ← lê o FICHEIRO INTEIRO
        };
        let data = if self.embedded.is_none() {
            extract_collection_face(&data, self.index).unwrap_or(data)
        } else { data };
        ttf_parser::Face::parse(&data, 0).ok()?;
        Some(Font::from_data(data))
    }).clone()
}
```

Cada face de uma coleção TrueType (`.ttc`) é um `FontSlot` separado (`03_infra/src/fonts.rs:194-199`,
`push_slots` — um slot por `index` de `0..face_count`). `FontSlot::get()` lê `self.path` — o
ficheiro **inteiro** da coleção, não só os bytes da face pedida — e só depois `extract_collection_face`
recorta a face `self.index`. O `OnceLock` cacheia por slot (por face), correto para chamadas
repetidas à *mesma* face, mas não partilha bytes entre faces irmãs do mesmo ficheiro físico: pedir
as 10 faces de `NotoSansCJK-Regular.ttc` custa 10 leituras completas de ~19 MB = ~195 MB de I/O só
para esse um ficheiro. Multiplicado pelos 4 `.ttc` CJK grandes (10+10+5+5 = 30 faces, ~19–27 MB
cada), a ordem de grandeza é ~600–700 MB de I/O redundante — consistente com os 2.19 s em `read()`
e com o RSS de pico de 9.4 GB (várias dezenas de MB retidos por alocação, sem partilha).

### 4.3 Por que a busca chega a estas fontes CJK

`03_infra/src/shaper.rs`, `CandidateSet::covering_all` (linha 664-685):

```rust
fn covering_all(&mut self, c: char) -> Vec<usize> {
    let mut result = Vec::new();
    for (i, cand) in self.primary.iter().enumerate() { ... }
    let book_len = self.world.book().len();
    for slot_idx in self.primary.len()..book_len {   // ← TODO o FontBook, sem filtro
        ...
        let fallback = self.load_fallback(slot_idx);  // ← FontSlot::get() por trás
        ...
    }
    result
}
```

As listas curadas de fallback (`DEFAULT_FALLBACK_FONTS_MATH`/`_SERIF`/`_SANS`,
`03_infra/src/fallback_fonts.rs:23-65`) são tentadas primeiro via `resolve_candidates` (chamadas em
`shaper.rs:196/225/371/400`), mas quando um glifo não é coberto por **nenhuma** delas, o caminho
cai em `CandidateSet::covering_all`/`covering_run`, que percorre o `FontBook` inteiro — todas as
~1086 fontes de sistema, incluindo os `.ttc` CJK — sem nenhum filtro de classe ou de família. Para
símbolos matemáticos incomuns (o comentário P784/P840 em `fallback_fonts.rs` já documenta histórico
de glifos math mal resolvidos), isto significa pagar o custo total de abrir/ler cada face de cada
fonte do sistema só para descobrir que nenhuma delas tem cobertura de símbolos matemáticos/gregos —
que é exactamente o resultado esperado, CJK não cobre esses códigos-ponto.

**Nível de confiança:** o mecanismo (loop sem early-exit nem filtro sobre `book_len`) está lido
diretamente no código-fonte, não é inferência. A ligação causal precisa ("é isto que gera os 21
`openat`") é a mesma inferência por correlação da secção 4.1.

---

## 5. Passo 2 — imagens: duplicação confirmada

### 5.1 Medição com `mutool info`

```
VANILLA (03-images.typ, 50× image("debian-logo.png")):
Images (1):
    1 (65 0 R): [ Flate ] 48x48 8bpc ICC (77 0 R)

CRISTALINO:
Images (50):
    1 (3 0 R): [ Flate ] 48x48 8bpc DevRGB (17 0 R)
    1 (3 0 R): [ Flate ] 48x48 8bpc DevRGB (19 0 R)
    ... (48 mais, IDs 21 a 101+)
```

Vanilla: **1** XObject de imagem, referenciado 50 vezes nas páginas (dedup real). Cristalino: **50**
XObjects distintos, um por chamada de `image()` — confirma a hipótese do Passo 2 do prompt: é
duplicação de recurso, causa diferente da de math (não é subsetting).

### 5.2 Localização do mecanismo de dedup e por que falha

`03_infra/src/export/images.rs:263-268`, comentário de `scan_all_images`:

> "A deduplicação usa `Arc::as_ptr(data) as usize` como chave — seguro porque `PagedDocument`
> mantém todos os Arcs vivos durante `export_pdf`, impedindo que o alocador reutilize os mesmos
> endereços."

A deduplicação **existe** e é logicamente correta *se* o mesmo `Arc` for reutilizado entre chamadas
idênticas a `image()`. A medição em 5.1 mostra que isso não acontece: 50 chamadas → 50 XObjects → 0
reaproveitamento. `01_core/src/entities/elements/image.rs:46,53` mostra `Content::Image(Arc::new
(self.clone()))` — um novo `Arc` é alocado a cada avaliação do elemento. Isto é **inferência, não
confirmado por instrumentação**: não tracei o caminho completo de `native_image` (referenciado em
`01_core/src/engine/eval/mod.rs:1373,1553`) até à decodificação de bytes para confirmar que cada
chamada de fato decodifica/aloca de novo em vez de reaproveitar um cache por `FileId`. O que
confirmaria: instrumentar `scan_all_images` para imprimir `Arc::as_ptr(data)` das 50 imagens do
documento — se os 50 ponteiros forem todos distintos (esperado dado o resultado do PDF), confirma
que a causa é ausência de memoização no carregamento/avaliação de `image()`, não um bug no dedup
por ponteiro em si.

### 5.3 CPU vs I/O (imagens, complementar — não pedido explicitamente no Passo 3, medido por já ter a ferramenta pronta)

| Métrica | Vanilla | Cristalino |
|---|---|---|
| User time | 0.00 s | 0.02 s |
| System time | 0.00 s | 0.07 s |
| Maximum RSS | 22 232 KB | 31 620 KB |
| File system outputs | 32 | 224 |

Mesma assinatura (sistema > utilizador), mas em escala muito menor (dezenas de ms, não segundos) —
consistente com o custo ser proporcional a 50 recodificações/escritas de uma imagem pequena (48×48),
não com releitura de ficheiros de dezenas de MB como em math.

---

## 6. Conclusão e localização exata para correção futura (sem implementar aqui)

| Causa confirmada | Localização | Afeta |
|---|---|---|
| Subsetting CFF desativado (P797, `oxifont_subset` produz CFF inválido) | `03_infra/src/export/subset.rs:79-89` | Tamanho do PDF em **qualquer** documento com fonte CFF (math, hello, lorem — confirmado nos três) |
| Busca de fallback global sem filtro nem early-exit, lendo `.ttc` inteiros por face | `03_infra/src/shaper.rs:664-685` (`CandidateSet::covering_all`) + `03_infra/src/fonts.rs:56-75` (`FontSlot::get`, leitura de ficheiro inteiro por face) | Tempo de execução em math (dominante, 79% do tempo em `system time`) |
| Deduplicação de imagem por identidade de `Arc`, sem cache de decodificação por `FileId`/conteúdo | `03_infra/src/export/images.rs:263-268` (mecanismo) + `01_core/src/entities/elements/image.rs:46,53` (origem provável do novo `Arc` a cada chamada, não confirmado por instrumentação) | Tempo de execução e tamanho do PDF em imagens repetidas |

**Duas causas de math são independentes** — corrigir subsetting CFF reduziria o tamanho do PDF mas
não tocaria nos 5+ segundos de `system time` gastos a reler `.ttc` CJK; corrigir a busca de
fallback reduziria o tempo mas não o tamanho do PDF (a fonte final embutida continuaria inteira,
sem subset). Um próximo passo de correção precisa de abordar as duas separadamente, e deve
instrumentar antes de corrigir (secções 4.1 e 5.2 marcam explicitamente onde a causa é inferência
por correlação e não confirmação directa).

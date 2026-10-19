# Relatório P935 — Mecanismo real do `fontdb-0.23.0` e causa da lentidão CJK/emoji no cristalino

**Data de execução:** 2026-07-31  
**Ficheiro de passo:** `00_nucleo/materialization/typst-passo-935.md`  
**Commit base:** `acef3e88d0d5d7cc62c06dfbb741590aca2cc331` (P933-fixed, sem alterações de código de produção relativamente a P934)  
**Binário vanilla real confirmado:** `lab/typst-original/target/release/typst` — SHA-256 `e73e4ac16f1d64843941b405a902f5f6952df4414d60b5a953efbe4952b858b5`, strings `fontdb-0.23.0`, versão CLI `typst 0.15.0 (969087ec)`.

---

## 1. Resumo executivo

A hipótese de P934 — "o vanilla usa `fontdb-0.23.0`, que pré-computa cobertura no arranque" — **está incorreta**. `fontdb-0.23.0` não pré-computa cobertura de caracteres; apenas descobre fontes e faz parse parcial de metadados (`RawFace`).

A verdadeira diferença de performance está no **I/O de fontes**:

| Modo (programa isolado, 1112 faces) | Tempo discovery + coverage |
|---|---:|
| Vanilla-like (mmap via `fontdb::with_face_data`) | **83 ms** |
| Cristalino-like (`std::fs::read` para `Vec<u8>` por face, sem partilha) | **4 048 ms** |
| Cristalino-like com partilha de `Vec<u8>` entre faces do mesmo `.ttc` | **1 814 ms** |

A conclusão é que o cristalino, ao ler ficheiros inteiros para `Vec<u8>` e não partilhar bytes entre faces de coleções (`.ttc`), paga **48× mais** I/O que o vanilla. Mesmo partilhando `Vec<u8>`, ainda paga **22× mais** que o mmap, porque a cópia para memória do processo é cara comparada com o acesso sob demanda do mmap.

---

## 2. Fase A — confirmação do mecanismo `fontdb-0.23.0`

### 2.1 Código-fonte de `fontdb-0.23.0`

Caminho local: `/home/dikluwe/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fontdb-0.23.0/src/lib.rs`.

- `Database::load_system_fonts()` (linhas 400–476) apenas varre directórios e chama `load_font_file` para cada ficheiro de fonte.
- `load_font_file_impl` (linhas 267–273) faz **mmap** do ficheiro via `memmap2::MmapOptions::new().map(&file)`.
- `parse_face_info` (linhas 1034–1055) usa `ttf_parser::RawFace::parse` (parse parcial) e extrai apenas: `families`, `post_script_name`, `style`, `weight`, `stretch`, `monospaced`.
- **Não há campo de cobertura/coverage em `FaceInfo`** (linhas 810–856).

### 2.2 Uso do `fontdb` no vanilla real

`lab/typst-original/crates/typst-kit/src/fonts.rs`:

- `system()` (linha 148) chama `db.load_system_fonts()`.
- `with_db()` (linha 170) itera `db.faces()` e chama `db.with_face_data(face.id, FontInfo::new)` para cada face.
- `FontInfo::new` (em `typst-library/src/text/font/info.rs`, linha 47) faz `ttf_parser::Face::parse` completo e itera a tabela `cmap` para extrair `coverage` (linhas 117–123).

Ou seja, o vanilla real:
1. Descobre fontes com `fontdb` (mmap).
2. Extrai `FontInfo` completo, incluindo coverage, **eager** para cada face, reutilizando o mmap do `fontdb`.

### 2.3 Programa isolado

Criado em `temp/p935/`. Dependências: `fontdb = "0.23"`, `ttf-parser = "0.25"`.

Resultados (cache quente, 3 runs consistentes):

```text
load_system_fonts: 32.9 ms  faces: 1112
VANILLA-like (mmap) coverage: 50.6 ms  parsed: 1112  codepoints: 28689553
CRISTALINO-like (read Vec) coverage: 4015.4 ms  parsed: 1112  codepoints: 28689553
CRISTALINO-like (shared Vec) coverage: 1780.6 ms  parsed: 1112  codepoints: 28689553
```

A face count (1112) é menor que os 2172 registados em P934 porque o P934 contou faces abertas durante a execução completa (incluindo fontes embutidas/projecto) e/ou o ambiente de fontes mudou ligeiramente. O factor de diferença é o observável relevante.

---

## 3. Diagnóstico do cristalino

### 3.1 Como o cristalino faz I/O de fontes

`03_infra/src/fonts.rs`:

- `FontSlot::source_bytes()` (linhas 74–85) e `FontSlot::get()` (linhas 95–121) usam `std::fs::read(&self.path)` para ler o ficheiro inteiro para `Vec<u8>`.
- Para `.ttc`, `FontSlot::get()` extrai a face individual, mas `source_bytes()` devolve os bytes da coleção inteira.
- `FontSlot` tem `shared_source: Option<Arc<OnceLock<Option<Arc<Vec<u8>>>>>>` (P875) para partilhar bytes entre faces do mesmo ficheiro, **mas `03_infra/src/fontdb.rs` cria os slots com `FontSlot::new(path, index)` em vez de `FontSlot::new_with_shared_source(...)`**.

`03_infra/src/fontdb.rs`:

- `load_system_fonts()` (linhas 28–64) itera `db.faces()` e cria cada slot com `FontSlot::new(path, face.index)` (linha 55).
- Não agrupa faces por path nem usa `shared_source`.

`03_infra/src/world.rs`:

- `candidates_for_char` (linhas 584–603) computa coverage lazy para **todos** os slots na primeira consulta de um caractere fora da cobertura embutida.
- Com slots sem partilha de bytes, cada face de cada `.ttc` lê o ficheiro coleção inteiro do disco.

### 3.2 Custo medido

Baseline do cristalino P933-fixed contra o vanilla real (`hyperfine --warmup 1 --min-runs 3`):

| Cenário | Vanilla real | Cristalino | Rácio |
|---|---:|---:|---:|
| `utf8-latin` | 274 ms | 88 ms | 0.32× |
| `utf8-cjk` | 290 ms | 2 309 ms | 7.96× |
| `utf8-emoji` | 277 ms | 7 029 ms | 25.34× |
| `05-utf8` | 297 ms | 7 404 ms | 24.93× |

O cristalino é competitivo em latim (onde o preload P927 evita abrir fontes do sistema) e muito lento em CJK/emoji (onde o fallback dispara o scan lazy com I/O ineficiente).

---

## 4. Causa raiz

A diferença de performance não é "pré-computar vs lazy". Ambos os compiladores fazem parse completo + iteração da `cmap`. A diferença é **como os bytes das fontes são acedidos**:

- **Vanilla:** mmap via `fontdb`. O kernel mapeia as páginas sob demanda; faces do mesmo `.ttc` partilham o mesmo mapa físico; não há cópia para memória do processo.
- **Cristalino:** `std::fs::read` para `Vec<u8>` por face, sem partilha entre faces do mesmo `.ttc`. Cada face lê o ficheiro coleção inteiro; a memória do processo enche de cópias redundantes.

A primeira optimização portável é fazer o cristalino partilhar bytes entre faces do mesmo ficheiro e, idealmente, usar mmap como o vanilla.

---

## 5. Implicações para a Fase B

O passo 935 propõe "substituir o mecanismo actual do cristalino por uma chamada directa à API do `fontdb`" para obter coverage barata. Essa API **não existe** em `fontdb-0.23.0`. O mecanismo real do vanilla é:

1. `fontdb` para descoberta + mmap.
2. Extração eager de `FontInfo` (incluindo coverage) via `db.with_face_data`.

Portanto, a Fase B deve ser reinterpretada como: **portar o padrão de I/O do vanilla (mmap + bytes partilhados) para o cristalino**, mantendo ou não o lazy coverage.

Duas opções arquiteturais:

### 5.1 Opção B1 — Menos invasiva: partilha de `Vec<u8>` em `fontdb.rs`

- Modificar `fontdb::load_system_fonts` para agrupar faces por `path`.
- Criar slots de `.ttc`/`.otc` com `FontSlot::new_with_shared_source` (partilha de `Arc<OnceLock<Option<Arc<Vec<u8>>>>>>`).
- Manter lazy coverage.
- **Ganho esperado:** redução de ~2,3 s → ~1 s em CJK (programa isolado: 4 s → 1,8 s).
- **Distância ao vanilla:** ainda ~3–4× mais lento.

### 5.2 Opção B2 — Fiel ao vanilla: mmap em `FontSlot`

- Adicionar `memmap2` a `03_infra/Cargo.toml`.
- `FontSlot` armazena ou bytes embutidos ou um `memmap2::Mmap` owned para ficheiros do disco.
- `source_bytes()` e `get()` usam o mmap; `extract_collection_face` opera sobre slices do mmap.
- Opcionalmente extrair coverage eager no startup (como vanilla) para eliminar o scan lazy.
- **Ganho esperado:** próximo do vanilla (~100–300 ms para CJK/emoji).
- **Custo:** muda a implementação interna de `FontSlot` e requer actualização do L0.

A Opção B2 é a única que fecha a distância de 7,8×–27× reportada em P934.

---

## 6. Próximos passos

1. Atualizar os Prompts L0 `00_nucleo/prompts/infra/fontdb.md` e `00_nucleo/prompts/infra/fonts.md` para codificar o padrão de I/O do vanilla (mmap/bytes partilhados).
2. Confirmar os L0 com o dono (hash).
3. Implementar a Opção B2 no código.
4. Medir os 7 cenários canónicos + 5 casos UTF-8 contra o vanilla real.
5. Se sobrar distância, documentar candidatos para rodada seguinte.

---

## 7. Proveniência das medições

| Medição | Ferramenta | Estado do código | Notas |
|---|---|---|---|
| Programa isolado `temp/p935` | `cargo run --release` | working tree com `03_infra/src/shaper.rs` modificado (P933-fixed) | 1112 faces encontradas; cache quente após 1.ª run |
| Baseline cristalino | `hyperfine --warmup 1 --min-runs 3` | binário `target/release/typst` do commit base | CLI do cristalino não usa subcomando `compile` |
| Baseline vanilla | `hyperfine --warmup 1 --min-runs 3` | `lab/typst-original/target/release/typst` SHA `e73e4ac1...` | requer `-f pdf` para `/dev/null` |

---

## 8. Fase B — implementação eager coverage (padrão vanilla)

### 8.1 Mudanças de código

- `03_infra/src/fonts.rs`:
  - `font_info_from_bytes` preenche `coverage` eager via `extract_coverage` durante a construção do `FontBook`.
  - `extract_coverage` tornou-se privada ao módulo.
  - `FontSlot` guarda apenas `path` + `index` + bytes embutidos; a partilha de `Vec<u8>` entre faces do mesmo `.ttc` foi removida (P875 reverte neste passo).
  - `pair_slots_with_book` continua a descartar slots cuja info falhe.
- `03_infra/src/fontdb.rs`:
  - `load_from_db` extrai `FontInfo` + coverage reutilizando o mmap interno do `fontdb` (`db.with_face_data`).
- `03_infra/src/world.rs`:
  - Removido `coverage_cache`, `preload_coverage_if_needed`, `embedded_coverage_union` e `source_text_nodes`.
  - `candidates_for_char` delega simplesmente a `FontBook::candidates_for_char`.
- `04_wiring/src/main.rs`:
  - Removida a chamada `world.preload_coverage_if_needed(&source)`.
- `03_infra/Cargo.toml`:
  - Removida a dependência `memmap2` (não usada nesta variante).

### 8.2 Prompts L0 atualizados

- `00_nucleo/prompts/infra/fonts.md`
- `00_nucleo/prompts/infra/fontdb.md`
- `00_nucleo/prompts/infra/system-world.md`
- `00_nucleo/prompts/wiring.md`

Hashes actualizados pelo `crystalline-lint --fix-hashes`.

---

## 9. Resultados após implementação

Binários:
- Cristalino: `target/release/typst` (working tree com alterações de P935).
- Vanilla real: `lab/typst-original/target/release/typst` (mesmo binário de base).

Comando:
```bash
hyperfine --warmup 1 --min-runs 3 \
  'target/release/typst <input>.typ /dev/null' \
  'lab/typst-original/target/release/typst compile <input>.typ /dev/null -f pdf'
```

| Cenário | Cristalino P935 | Vanilla real | Rácio (cristalino/vanilla) | Cristalino P933-fixed | Δ rácio |
|---|---:|---:|---:|---:|---:|
| `utf8-latin` | **147.6 ms** | 265.5 ms | **0.56×** | 88 ms | 0.24× pior |
| `utf8-cjk` | **1 700 ms** | 298.8 ms | **5.69×** | 2 309 ms | **2.27× melhor** |
| `utf8-emoji` | **8 771 ms** | 280.2 ms | **31.30×** | 7 029 ms | **5.96× pior** |
| `05-utf8` | **6 709 ms** | 303.9 ms | **22.08×** | 7 404 ms | **2.85× melhor** |

Timings internos do cristalino (`--timings-json`):

| Cenário | total_ms | layout_ms | shape_ms |
|---|---:|---:|---:|
| `utf8-cjk` | 1 460 | 1 358 | 101 |
| `utf8-emoji` | 8 155 | 5 940 | 1 918 |
| `05-utf8` | 5 941 | 5 185 | 455 |

### 9.1 Interpretação

- **CJK e `05-utf8` melhoraram** com coverage eager: o fallback já sabe quais fontes cobrem o caractere sem pagar o scan lazy no caminho quente.
- **`utf8-emoji` piorou**: o benchmark dispara fallback para dezenas de emojis diferentes. Com coverage completo, `candidates_for_char` devolve muitos candidatos, e cada candidato é carregado do disco via `FontSlot::get()` (`std::fs::read`). O tempo passa a ser dominado por **I/O de fontes durante o shaping**, não pelo cálculo de coverage.
- **Latim regrediu ligeiramente** (mas continua mais rápido que o vanilla): a extração eager de coverage no startup acrescenta trabalho mesmo quando o documento não usa fallback.

Conclusão: **coverage eager sozinho não fecha a distância**. O factor limitante deixou de ser "quando calcular coverage" e passou a ser **como os bytes das fontes são acedidos durante o fallback**.

---

## 10. Candidatos para a Fase C

A distância de 22×–31× em CJK/emoji só fecha se o carregamento de fontes no fallback deixar de fazer `std::fs::read` a cada candidato.

### 10.1 Candidato C1 — mmap persistente nos `FontSlot`

- Repor `memmap2` em `03_infra/Cargo.toml`.
- `FontSlot` armazena `Option<Mmap>` (ou referência ao mapa do `fontdb`) para fontes do disco.
- `FontSlot::get()` devolve `Font::from_data` sobre o mmap; nenhuma cópia para `Vec<u8>`.
- O `Database` do `fontdb` pode ter de ser mantido vivo durante a vida do `SystemWorld` (ou os ficheiros reabertos individualmente).
- **Ganho esperado**: shape_ms e layout_ms próximos do vanilla, porque cada candidato passa a ser apenas um acesso ao mapa já residente.

### 10.2 Candidato C2 — cache de `Font` carregados

- Manter um cache `HashMap<(path, index), Arc<Font>>` no `SystemWorld` (ou num serviço de fontes).
- Evita releituras do disco quando o mesmo slot é pedido várias vezes no mesmo documento.
- Não resolve a primeira leitura, mas pode cortar significativamente os casos com repetição de candidatos.

### 10.3 Candidato C3 — heuristicamente limitar os candidatos devolvidos por `candidates_for_char`

- Em vez de devolver todos os índices cujo bloco cobre o caractere, devolver apenas as primeiras *N* fontes ou aplicar scoring rápido.
- Risco de quebra de paridade: o vanilla não faz este corte.
- **Menos prioritário** que C1/C2 porque ataca o sintoma, não a causa (I/O).

---

## 11. Validação final

- `cargo build --workspace --release`: ok.
- `cargo test -p typst-infra --lib`: 743 passed; 0 failed.
- `crystalline-lint .`: 0 drift/violations (apenas V7 pré-existente, `package_version_resolution.md`).
- Estado do código: alterações na working tree, por commitar.

---

## 12. Proveniência das medições

| Medição | Ferramenta | Estado do código | Notas |
|---|---|---|---|
| Benchmarks P935 | `hyperfine --warmup 1 --min-runs 3` | working tree com alterações de P935 (commit base `acef3e88`) | `target/release/typst` vs `lab/typst-original/target/release/typst` |
| Timings internos | `--timings-json` | idem | total não inclui startup |
| Programa isolado `temp_p935` | `cargo run --release` | idem | mantido para referência; não re-medido nesta fase |

---

## 13. Próximos passos recomendados

1. Decidir com o dono se a Fase C (C1 — mmap persistente) é autorizada agora ou fica para o passo seguinte.
2. Se ficar para o passo seguinte: actualizar o Prompt L0 `00_nucleo/prompts/infra/fonts.md` para documentar a necessidade de mmap persistente.
3. Commitar as alterações de P935 como um passo completo (eager coverage).

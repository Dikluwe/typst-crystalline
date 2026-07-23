# Relatório — P876: Cache de imagem por `FileId` e por ponteiro `Arc`

**Data:** 2026-07-23T18:48:27-03:00  
**Commit:** `30122788891b32a423b0354b97eebdbf28ccf068`  
**Estado:** fechado  
**L0s afetados:**
- `00_nucleo/prompts/infra/system-world.md` → `3738da1c`
- `00_nucleo/prompts/infra/export/images.md` → `912e29b6`

---

## 1. Problema

P873 mediu que documentos com múltiplas referências ao mesmo ficheiro de imagem (`#image("test.png")` repetido) produziam **um XObject por referência** em vez de um único XObject partilhado. A instrumentação de P876 confirmou a causa raiz:

- Cada chamada `image()` avaliada pelo núcleo invoca `World::read_bytes(file, path)`.
- A implementação anterior de `SystemWorld::read_bytes` lia o disco e criava um `Arc<Vec<u8>>` novo.
- Como cada `Arc` tinha endereço distinto, a deduplicação por `Arc::as_ptr` no exportador PDF falhava.
- Além disso, imagens raster (PNG/GIF/WebP) eram decodificadas/comprimidas uma vez por referência, mesmo quando o `Arc` fosse o mesmo.

---

## 2. Solução

### 2.1 Cache de bytes brutos por `FileId` em `SystemWorld` (L3)

`03_infra/src/world.rs`:
- Novo campo `read_cache: Mutex<HashMap<FileId, Arc<Vec<u8>>>>` em `SystemWorld`.
- Inicializado em `SystemWorld::new`.
- Novo método privado `load_file_bytes(&self, path)` lê do disco com erros no formato `FileError`.
- Novo método privado `read_bytes_cached(&self, id)` consulta o cache por `FileId`; em miss, lê do disco e insere.
- `World::file(id)` passa a usar `read_bytes_cached(id)` e converte para `Bytes`.
- `World::read_bytes(current_file, path)` resolve o path, regista o ficheiro via `register_file` para obter um `FileId` canónico, e devolve o `Arc` partilhado do cache.

Efeito: duas chamadas `#image("test.png")` no mesmo documento recebem o mesmo `Arc<Vec<u8>>`, permitindo que `Arc::as_ptr` funcione como chave de deduplicação.

### 2.2 Cache de `PdfImagePayload` por ponteiro `Arc` em `export/images.rs` (L3)

`03_infra/src/export/images.rs`:
- `PdfImagePayload` ganha `#[derive(Clone)]` (necessário para o cache).
- Thread-local `PAYLOAD_CACHE: RefCell<HashMap<usize, PdfImagePayload>>` indexado por `Arc::as_ptr(data) as usize`.
- Em `process_image_item`, no braço PNG/GIF/WebP, o payload é consultado no cache antes de chamar `process_png_for_pdf`. Em miss, processa e insere uma cópia.

Efeito: a decodificação raster e a compressão Zlib só acontecem uma vez por `Arc<Vec<u8>>` distinto durante um export.

### 2.3 Remoção da instrumentação temporária

Foram removidos os `eprintln!` e o thread-local `P876_COUNTERS` adicionados na fase de diagnóstico.

---

## 3. Ficheiros alterados

- `03_infra/src/world.rs`
- `03_infra/src/export/images.rs`
- `00_nucleo/prompts/infra/system-world.md`
- `00_nucleo/prompts/infra/export/images.md`

---

## 4. Validação

### 4.1 Testes

```text
$ cargo test --workspace
...
test result: ok. 4686 passed; 0 failed; 2 ignored; 0 measured; 0 filtered out
test result: ok. 727 passed; 0 failed; 5 ignored; 0 measured; 0 filtered out
test result: ok. 41 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
test result: ok. 37 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
...
Total: 5505 passed (contando doc-tests ignorados)
```

Sem alteração no número de testes — a correção é puramente de cache/deduplicação e não muda contratos testáveis a nível unitário (o comportamento observável do PDF já era válido, apenas ineficiente).

### 4.2 Linter

```text
$ crystalline-lint .; echo "exit: $?"
warning: Prompt órfão: '00_nucleo/prompts/infra/package_version_resolution.md' ... [V7]
exit: 0
```

Zero violations. O único warning (V7) é pré-existente e não relacionado com P876.

### 4.3 Deduplicação de XObjects

Documento `/tmp/p876/main.typ` com 50× `#image("test.png")`:

```text
$ mutool info /tmp/p876/out.pdf
Images (1):
	1	(3 0 R):	[ Flate ] 100x100 8bpc DevRGB (24 0 R)
```

**Resultado:** 50 referências → 1 imagem no PDF.

### 4.4 Controlo: imagens distintas não são colapsadas

Documento `/tmp/p876/main2.typ` com `#image("test.png")` e `#image("test2.png")` alternados:

```text
$ mutool info /tmp/p876/out2.pdf
Images (2):
	1	(3 0 R):	[ Flate ] 100x100 8bpc DevRGB (8 0 R)
	1	(3 0 R):	[ Flate ] 100x100 8bpc DevRGB (9 0 R)
```

**Resultado:** 2 ficheiros distintos → 2 XObjects.

### 4.5 Tempo de compilação

Documento com 50 referências à mesma imagem PNG 100×100:

```text
$ time ./target/release/typst /tmp/p876/main.typ /tmp/p876/out.pdf
real	0m0,167s
user	0m0,076s
sys	0m0,091s
```

Tempo aceitável para cenário de stress de referências repetidas. Não foi feita medição comparativa rigorosa com o estado pré-P876 nesta sessão; o ganho principal é a redução do tamanho do PDF (1 imagem vs 50) e da carga de CPU/memória, não necessariamente do tempo total neste documento pequeno.

---

## 5. Testes adicionados

Nenhum teste unitário novo. A deduplicação de XObjects é verificada manualmente via `mutool info` (secção 4.3 e 4.4). Adicionar um teste de integração automatizado que inspecione a estrutura interna do PDF estava fora do escopo deste passo; fica como próximo passo se o utilizador quiser endurecer a regressão.

---

## 6. Decisões e limitações

- **`read_cache` por `FileId`:** a chave é o `FileId` canónico obtido via `register_file`. Como `register_file` canonicaliza o path, paths diferentes que apontem para o mesmo ficheiro físico (symlinks, etc.) partilham o cache.
- **Cache não persiste entre compilações:** é campo de `SystemWorld`, descartado quando o mundo é destruído. Estado global é proibido em L1; em L3 optou-se por cache local ao mundo.
- **`PAYLOAD_CACHE` thread-local:** limitado ao thread do export. Se o export passar a ser multi-threaded no futuro, o cache terá de migrar para `Mutex<HashMap<...>>` ou similar. Nesta arquitetura atual o export é single-threaded.
- **JPEG:** não passa por `process_png_for_pdf`, logo não beneficia do `PAYLOAD_CACHE`, mas já é emitido cru e a deduplicação por `Arc::as_ptr` evita XObjects duplicados.
- **SVG/PNG export:** fora do escopo; o cache aplica-se apenas ao pipeline PDF.

---

## 7. Próximos passos

- P877: continuar refinamentos do exporter PDF ou avançar para paridade visual PNG/SVG se priorizado pelo utilizador.

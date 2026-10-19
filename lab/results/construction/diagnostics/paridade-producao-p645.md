# Relatório de Paridade — P645

**Passo:** 645  
**Data:** 2026-07-09  
**Foco:** Preservar `SourceDiagnostic` (e o `span` correspondente) nos erros de bibliografia detectados durante o layout, em vez de converter para `String`.  
**Dependências:** P644 (onde `layout_errors: Vec<String>` foi introduzido).

---

## 1. Contexto

P644 introduziu `layout_errors: Vec<String>` em `PagedDocument` e `Layouter` para transportar erros de bibliografia gerados durante o layout (`build_cache`/`build_cache_with_style` em `bib_csl.rs`). A conversão para `String` descartava o `span` que o `SourceDiagnostic` original já carregava, perdendo a posição no ficheiro do utilizador.

Este passo corrige essa inconsistência: `layout_errors` passa a ser `Vec<SourceDiagnostic>` em todo o percurso, e a pipeline em L3 propaga os diagnósticos directamente, sem reconstruí-los a partir de texto.

---

## 2. Implementação

### 2.1 `01_core/src/entities/layout_types.rs`

- Importado `SourceDiagnostic`.
- Alterado `PagedDocument.layout_errors` de `Vec<String>` para `Vec<SourceDiagnostic>`.
- Actualizado o comentário para reflectir a preservação de `span`.

### 2.2 `01_core/src/engine/layout/mod.rs`

- Importado `SourceDiagnostic`.
- Alterado `Layouter.layout_errors` de `Vec<String>` para `Vec<SourceDiagnostic>`.
- Alterada a variável local `layout_errors` no início de `layout_with_introspector_and_metrics` para `Vec<SourceDiagnostic>`.
- No tratamento de `Err(diagnostics)` vindo de `build_cache`/`build_cache_with_style`, passou-se a usar `layout_errors.extend(diagnostics)` em vez de extrair `.message` de cada diagnóstico.

### 2.3 `03_infra/src/pipeline.rs`

- No bloco que propaga `doc.layout_errors`, os erros são agora devolvidos directamente (`doc.layout_errors.drain(..).collect()`) em vez de serem reconstruídos com `SourceDiagnostic::error(Span::detached(), msg)`.
- Comentário actualizado para P644/P645.

---

## 3. Validação

### 3.1 Testes automáticos

```bash
cargo test --workspace
```

Resultado: **todos os testes passaram**.

```bash
crystalline-lint .
```

Resultado: **✓ No violations found**.

### 3.2 Teste manual — chave vazia em `.bib`

```bash
cat > /tmp/p645-chave-vazia.bib <<'EOF'
@article{,
  title = {Sem chave},
  author = {Alguém},
  year = {2024}
}
EOF
cat > /tmp/p645-chave-vazia.typ <<'EOF'
#bibliography("/tmp/p645-chave-vazia.bib")
EOF
./target/release/typst /tmp/p645-chave-vazia.typ /tmp/p645.pdf
```

Saída:

```text
/tmp/p645-chave-vazia.typ:<detached>: error: failed to parse BibTeX '/tmp/p645-chave-vazia.bib': esperado key da entrada
```

O erro continua a ser apresentado. A mensagem inclui o caminho do ficheiro `.typ` e o caminho do `.bib`, mas o `span` aparece como `<detached>` porque os pontos de criação do diagnóstico em `01_core/src/engine/eval/bibliography.rs` e no parser BibTeX ainda usam `Span::detached()`. A mudança estrutural deste passo garante que, quando esses pontos passarem a usar um `span` real, a posição será preservada até à saída final.

### 3.3 Teste manual — chave vazia em `.yaml`

```bash
cat > /tmp/p645-chave-vazia.yaml <<'EOF'
"":
  type: Article
  title: Sem chave
  author: Alguém
  date: 2024
EOF
cat > /tmp/p645-chave-vazia-yaml.typ <<'EOF'
#bibliography("/tmp/p645-chave-vazia.yaml")
EOF
./target/release/typst /tmp/p645-chave-vazia-yaml.typ /tmp/p645-yaml.pdf
```

Resultado: o comando terminou com sucesso e produziu PDF. Este cenário não propaga o erro esperado, mas está fora do âmbito de P645 (que limita-se ao transporte do `SourceDiagnostic` no pipeline de layout). Fica registado para investigação futura.

---

## 4. Decisão

- `layout_errors` transporta `SourceDiagnostic` do layout (L1) até à pipeline (L3).
- A pipeline já não reconstrói diagnósticos a partir de strings, preservando `span`, `hints` e `trace`.
- A posição exacta (linha/coluna) ainda depende de futuro trabalho nos pontos que criam os `SourceDiagnostic` originais; a infraestrutura de transporte está agora preparada para isso.

---

## 5. Estado de fecho

- [x] `Vec<String>` substituído por `Vec<SourceDiagnostic>` em todo o percurso.
- [x] Pipeline L3 devolve erros directamente sem reconstrução.
- [x] Sem regressão em `cargo test --workspace`.
- [x] `crystalline-lint .` limpo.
- [x] Relatório em `00_nucleo/diagnosticos/paridade-producao-p645.md`.

---

## 6. Hash do commit

`e497cbe6f`

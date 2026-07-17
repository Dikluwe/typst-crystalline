---
# P645 — Preservar posição nos erros de bibliografia detectados durante o layout

> **Passo:** 645
> **Data:** 2026-07-09
> **Foco:** P644 introduziu `layout_errors: Vec<String>` para propagar erros de bibliografia encontrados durante o layout, mas usa texto simples em vez de `SourceDiagnostic`, perdendo a posição exacta no ficheiro do utilizador. Todos os outros erros corrigidos nesta sequência (P634, P636, P642, P643) apontam para a linha e coluna certas. Este passo corrige a inconsistência.
> **Tipo:** Implementação directa.
> **Tamanho:** S.
> **ADR-0108 EM VIGOR.**
> **Dependências:** P644 (onde `layout_errors: Vec<String>` foi introduzido).

---

## Contexto

`SourceDiagnostic` já é o tipo usado em todos os outros erros desta sequência — carrega `span`, que o motor de renderização de erros usa para mostrar a linha e coluna exactas no ficheiro `.typ` ou `.bib`. `Vec<String>` descarta essa informação, mesmo que o `SourceDiagnostic` original já a tivesse no momento em que o erro foi criado, dentro de `hay_entry_to_bib_entry`/`bib_entry_to_hayagriva`.

---

## Implementação

### Trocar o tipo

- `01_core/src/entities/layout_types.rs`: `PagedDocument.layout_errors` passa de `Vec<String>` para `Vec<SourceDiagnostic>`.
- `01_core/src/rules/layout/mod.rs`: `Layouter.layout_errors` da mesma forma.
- `build_cache`/`build_cache_with_style` já devolvem `SourceResult<...>` (confirmado por P644) — os erros já são `Vec<SourceDiagnostic>` nesse ponto; a mudança é não os converter para `String` antes de os guardar.
- `03_infra/src/pipeline.rs`: ao encontrar `doc.layout_errors` não vazio, devolver `Err(doc.layout_errors)` directamente, sem reconstruir a partir de texto.

### Critério de fecho da implementação

- [ ] `layout_errors` usa `SourceDiagnostic` em todo o percurso, sem conversão para `String` a meio do caminho.
- [ ] Testado que a mensagem de erro final mostra a linha/coluna certa do ficheiro `.bib` ou `.typ`, não só o texto.

---

## Validação

Repetir os testes de P644 (`p644_yaml_chave_vazia_quoted_produz_erro`, etc.), confirmando que a mensagem de erro agora inclui posição, não só texto:

```bash
cat > /tmp/p645-chave-vazia.bib <<'EOF'
@article{,
  title = {Sem chave},
  author = {Alguém},
  year = {2024}
}
EOF
cat > /tmp/p645-chave-vazia.typ <<'EOF'
#bibliography("p645-chave-vazia.bib")
EOF
./target/release/typst /tmp/p645-chave-vazia.typ /tmp/p645.pdf
```

Confirmar que a mensagem de erro aponta para o ficheiro `.typ` (linha do `#bibliography(...)`), ou idealmente para o próprio `.bib`, se o mecanismo de `span` do cristalino já suportar isso — confirmar qual dos dois é possível antes de decidir qual usar.

```bash
cargo test --workspace
crystalline-lint .
```

---

## Critério de fecho do passo

- [ ] `Vec<String>` substituído por `Vec<SourceDiagnostic>` em todo o percurso.
- [ ] Mensagem de erro final com posição, não só texto.
- [ ] Sem regressão em `cargo test --workspace`.
- [ ] `crystalline-lint .` limpo.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p645.md`, com hash do commit.

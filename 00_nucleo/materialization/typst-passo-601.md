---
# P601 — Emitir `/Info` sempre, mesmo sem metadados do utilizador

> **Passo:** 601
> **Data:** 2026-07-05
> **Foco:** P600 encontrou, como observação lateral, que o cristalino só escreve `/Info` quando o documento tem `title`, `author`, ou `keywords`. O vanilla escreve sempre, com `/Creator`, `/CreationDate`, `/ModDate`, mesmo em documentos sem nenhum metadado definido pelo utilizador. Isto afecta a maioria dos documentos simples usados ao longo desta conversa. Este passo corrige isto.
> **Tipo:** Implementação directa. A causa já está confirmada por P600.
> **Tamanho:** S.
> **ADR-0108 EM VIGOR.**
> **Dependências:** P600 (onde a diferença foi encontrada, sem ser o foco desse passo), P536 (onde `/Info` foi implementado pela primeira vez, só condicional a metadados presentes).

---

## Contexto

`03_infra/src/export/builder.rs` (`emit_info`, ou função equivalente) só constrói o dicionário `/Info` quando pelo menos um de `title`, `author`, `keywords` está presente. O vanilla constrói `/Info` sempre, com `/Creator (Typst 0.15.0)`, `/CreationDate`, `/ModDate`, independentemente de o utilizador ter definido algo.

---

## Implementação

Alterar a condição em `emit_info` (ou onde for chamado) para construir `/Info` sempre, não só quando há metadados de utilizador. Os campos `title`, `author`, `keywords` continuam opcionais dentro do dicionário (só aparecem se definidos); `/Creator`, `/CreationDate` aparecem sempre.

### Critério de fecho da implementação

- [ ] Documento sem nenhum `#set document(...)` produz `/Info` com `/Creator` e `/CreationDate`.
- [ ] Documento com metadados continua a funcionar como antes (sem regressão).
- [ ] `/ModDate` — confirmar se o cristalino já tem este campo ou precisa de ser adicionado também (P600 não o mencionou explicitamente).

---

## Validação

```bash
cat > /tmp/p601-sem-metadados.typ <<'EOF'
Texto sem metadados.
EOF
./target/release/typst /tmp/p601-sem-metadados.typ /tmp/p601.pdf
pdfinfo /tmp/p601.pdf
```

Confirmar que `/Info` aparece, com `/Creator` pelo menos.

```bash
cargo test --workspace
crystalline-lint .
```

Correr também o corpus geral, dado que isto muda o output de praticamente todos os documentos de teste sem metadados explícitos:

```bash
for f in lab/parity/corpus/p490/*.typ lab/parity/corpus/p500/*.typ; do
  ./target/release/typst "$f" /tmp/out.pdf >/dev/null 2>&1 && echo "OK: $(basename $f)" || echo "FAIL: $(basename $f)"
done
```

Snapshots P307b provavelmente precisam de regeneração, dado que isto muda o output binário de praticamente todos os PDFs de teste — confirmar e regenerar se for o caso, com o mesmo mecanismo já usado antes (`UPDATE_P307B_SNAPSHOTS=1`).

---

## Critério de fecho do passo

- [ ] `/Info` presente em documentos sem metadados, com `/Creator` e `/CreationDate`.
- [ ] `/ModDate` confirmado como presente ou adicionado.
- [ ] Documentos com metadados sem regressão.
- [ ] Corpus geral sem falhas novas.
- [ ] Snapshots P307b regenerados, se necessário.
- [ ] Sem regressão em `cargo test --workspace`.
- [ ] `crystalline-lint .` limpo.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p601.md`, com hash do commit.
- [ ] Listas de disparidades actualizadas.

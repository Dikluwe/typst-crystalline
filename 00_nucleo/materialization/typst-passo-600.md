---
# P600 — Preencher `/Producer` no `/Info` do PDF

> **Passo:** 600
> **Data:** 2026-07-05
> **Foco:** `/Producer` no dicionário `/Info` não é preenchido. Registado antes como "scope-out" sem razão escrita — na revisão feita depois de P594, ficou confirmado que é falta de implementação, não decisão. Este passo confirma o que o vanilla escreve neste campo, e preenche o equivalente no cristalino.
> **Tipo:** Sonda + Implementação directa.
> **Tamanho:** XS.
> **ADR-0108 EM VIGOR.**
> **Dependências:** P536 (onde os outros campos de `/Info` foram implementados; `/Producer` ficou de fora).

---

## Sonda

### Confirmar o que o vanilla escreve

```bash
cat > /tmp/p600-teste.typ <<'EOF'
Texto de teste.
EOF
lab/typst-original/target/release/typst compile /tmp/p600-teste.typ /tmp/p600-vanilla.pdf
pdfinfo /tmp/p600-vanilla.pdf | grep -i producer
```

### Confirmar o estado actual do cristalino

```bash
./target/release/typst /tmp/p600-teste.typ /tmp/p600-cristalino.pdf
pdfinfo /tmp/p600-cristalino.pdf | grep -i producer
```

### Localizar onde `/Info` é escrito

```bash
grep -n "\"/Producer\"\|/Info\|emit_info" 03_infra/src/export/builder.rs | head -20
```

### Critério de fecho da sonda

- [ ] Valor exacto que o vanilla escreve confirmado (provavelmente algo como "Typst" mais a versão).
- [ ] Confirmado que o cristalino não escreve nada neste campo hoje.
- [ ] Ponto exacto no código onde os outros campos de `/Info` (`/Title`, `/Author`, etc.) já são escritos, para adicionar `/Producer` ao lado.

---

## Implementação

Adicionar `/Producer` ao mesmo ponto onde os outros campos de `/Info` já são escritos (`emit_info`, ou equivalente, em `builder.rs`). O valor deve identificar o cristalino como o produtor — por exemplo `"Typst Crystalline"` mais uma versão, não copiar literalmente o texto do vanilla (isso seria falso, o ficheiro não foi produzido pelo Typst original).

```rust
// Esboço, a confirmar contra a estrutura real:
parts.push(format!("/Producer {}", utf16be_hex_string("Typst Crystalline")));
```

Reutilizar a mesma função de codificação já corrigida em P538b (`utf16be_hex_string`), para não repetir o problema de acentos corrompidos que esse passo já resolveu para os outros campos.

### Critério de fecho da implementação

- [ ] `/Producer` preenchido no PDF gerado.
- [ ] Testado com `pdfinfo`, confirmando o valor.
- [ ] Testado que não introduz o mesmo bug de codificação já corrigido para outros campos em P538b.

---

## Validação

```bash
./target/release/typst /tmp/p600-teste.typ /tmp/p600-depois.pdf
pdfinfo /tmp/p600-depois.pdf | grep -i producer
cargo test --workspace
crystalline-lint .
```

---

## Critério de fecho do passo

- [ ] `/Producer` preenchido, confirmado com `pdfinfo`.
- [ ] Sem regressão em `cargo test --workspace`.
- [ ] `crystalline-lint .` limpo.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p600.md`, com hash do commit.
- [ ] Item actualizado na lista de disparidades: de "falta de implementação" para "corrigido".

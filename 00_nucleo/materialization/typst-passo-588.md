---
# P588 — Corrigir o espaço a mais no início do parágrafo depois de `#set`

> **Passo:** 588
> **Data:** 2026-07-05
> **Foco:** P587 localizou, com prova e `file:line`, que uma quebra de linha depois de `#set text(...)` é convertida em `Content::Space` e renderizada como espaço visual no início do parágrafo seguinte, empurrando o texto 10 pontos para a direita. Isto não é específico de RTL — é um bug geral de qualquer parágrafo que comece logo a seguir a uma regra `#set` na sua própria linha. Este passo corrige a causa já confirmada, e testa de forma geral, não só no caso árabe onde apareceu.
> **Tipo:** Implementação directa. A causa já está confirmada com `file:line`; não é preciso sonda nova para a causa em si, só para confirmar o alcance.
> **Tamanho:** M.
> **ADR-0108 EM VIGOR.**
> **Dependências:** P587 (causa confirmada com instrumentação), P577/P586 (onde o sintoma apareceu, em RTL).

---

## Contexto

`01_core/src/engine/eval/mod.rs:494` converte qualquer `SyntaxKind::Space` (incluindo a quebra de linha depois de `#set text(...)`) em `Content::Space`. `01_core/src/engine/layout/mod.rs:729` avança o cursor por `space_width()` sempre que encontra esse `Content::Space`, mesmo quando é o primeiro item de uma linha ou parágrafo. O vanilla não faz este avanço quando o espaço fica no início.

---

## Verificar o alcance antes de corrigir

### Confirmar se afecta texto latino comum

```bash
cat > /tmp/p588-latim.typ <<'EOF'
#set text(size: 20pt)
Texto normal aqui.
EOF
./target/release/typst /tmp/p588-latim.typ /tmp/p588.pdf
pdftotext -tsv /tmp/p588.pdf -
```

Confirmar se o `left` da primeira palavra ("Texto") está na margem (perto de 70,87pt para A4 com margem default) ou 10pt mais à direita. Se estiver deslocado, confirma que o bug é geral, não só do caso árabe.

### Confirmar se afecta outros pontos de entrada, não só `#set` sozinho numa linha

```bash
cat > /tmp/p588-variantes.typ <<'EOF'
= Heading
Texto depois de heading.

#figure([Imagem])
Texto depois de figura.
EOF
./target/release/typst /tmp/p588-variantes.typ /tmp/p588-variantes.pdf
pdftotext -tsv /tmp/p588-variantes.pdf -
```

Confirmar se o mesmo tipo de deslocamento aparece depois de outras construções que terminam com quebra de linha, não só `#set`.

---

## Implementação

Em `01_core/src/engine/layout/mod.rs:729` (ou onde `Content::Space` é processado no layout): não avançar o cursor quando o espaço é o primeiro item da linha corrente (ou seja, quando `cursor_x` ainda está na posição de margem/início, sem nenhum item de texto antes dele na mesma linha).

```rust
// Esboço, a confirmar contra a estrutura real:
Content::Space => {
    if !self.regions.current.current_line.is_empty() {
        self.layout_space(); // ou o avanço já existente
    }
    // se current_line estiver vazia, ignorar o espaço — é espaço
    // inicial de linha/parágrafo, não deve ser visível.
}
```

### Critério de fecho da implementação

- [ ] Espaço inicial de parágrafo deixa de avançar o cursor.
- [ ] Documento latino de teste: "Texto" começa exactamente na margem, sem os 10pt a mais.
- [ ] Documento árabe de referência (`الكتاب 42 على الطاولة`) re-testado, confirmando se a quebra de linha desaparece com margem default.
- [ ] Espaços que NÃO são o primeiro item de uma linha continuam a funcionar normalmente (por exemplo, o espaço entre duas palavras no meio do texto).

---

## Validação

```bash
./target/release/typst /tmp/p588-latim.typ /tmp/p588-depois.pdf
pdftotext -tsv /tmp/p588-depois.pdf -
```

Repetir o documento de referência da sequência RTL:

```bash
cat > /tmp/p588-rtl.typ <<'EOF'
#set text(dir: rtl, lang: "ar", size: 40pt)
الكتاب 42 على الطاولة
EOF
./target/release/typst /tmp/p588-rtl.typ /tmp/p588-rtl.pdf
pdftotext -tsv /tmp/p588-rtl.pdf -
```

Confirmar se as quatro palavras agora cabem numa linha só. Se ainda sobrar a diferença de fronteira de 0,46pt que P587 também encontrou (ligada à largura do "42"), registar isso à parte — é uma diferença muito pequena, a decidir se vale a pena perseguir ou aceitar por escrito.

```bash
cargo test --workspace
crystalline-lint .
```

Correr também o corpus geral, dado que este bug pode ter afectado documentos comuns:

```bash
for f in lab/parity/corpus/p490/*.typ lab/parity/corpus/p500/*.typ; do
  ./target/release/typst "$f" /tmp/out.pdf >/dev/null 2>&1 && echo "OK: $(basename $f)" || echo "FAIL: $(basename $f)"
done
```

---

## Critério de fecho do passo

- [ ] Alcance confirmado — geral, não só RTL.
- [ ] Correcção aplicada.
- [ ] Documento latino sem deslocamento inicial.
- [ ] Documento árabe de referência re-testado, com decisão sobre a diferença de fronteira residual, se sobrar.
- [ ] Corpus geral sem regressão nova.
- [ ] Sem regressão em `cargo test --workspace`.
- [ ] `crystalline-lint .` limpo.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p588.md`, com hash do commit.

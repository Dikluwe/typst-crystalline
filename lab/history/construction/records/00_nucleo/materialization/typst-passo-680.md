---
# P680 — Cadeias de import de três ficheiros e dependências em diamante

> **Passo:** 680
> **Data:** 2026-07-10
> **Foco:** P679 testou import de um nível (`A` importa `B`) e um ciclo simples de dois ficheiros (`A↔B`). Nunca testou uma cadeia de três ou mais ficheiros sem ciclo (`A→B→C`), nem uma dependência em diamante (`A` importa `B` e `C`, ambos importam `D`) — este segundo caso é importante porque `D` é "visto" duas vezes, em ramos diferentes, não em sequência circular, e podia ser confundido com um ciclo por um mecanismo de detecção mal desenhado.
> **Tipo:** Verificação directa. Correcção se confirmado algum problema.
> **Tamanho:** S.
> **ADR-0108 EM VIGOR.**
> **Dependências:** P679 (onde `#import` foi implementado, sem estes dois casos testados).

---

## Verificação

### Cadeia de três ficheiros, sem ciclo

```bash
cat > /tmp/p680-c.typ <<'EOF'
#let valor_c = "de C"
EOF
cat > /tmp/p680-b.typ <<'EOF'
#import "p680-c.typ": valor_c
#let valor_b = "de B, com " + valor_c
EOF
cat > /tmp/p680-a.typ <<'EOF'
#import "p680-b.typ": valor_b
#valor_b
EOF
lab/typst-original/target/release/typst compile /tmp/p680-a.typ /tmp/p680-vanilla.pdf
pdftotext /tmp/p680-vanilla.pdf -

./target/release/typst /tmp/p680-a.typ /tmp/p680-cristalino.pdf
pdftotext /tmp/p680-cristalino.pdf -
```

Confirmar que os dois produzem o mesmo texto ("de B, com de C"), e que o cristalino não trata isto como ciclo por engano.

### Dependência em diamante, sem ciclo

```bash
cat > /tmp/p680-d.typ <<'EOF'
#let valor_d = "de D"
EOF
cat > /tmp/p680-diamante-b.typ <<'EOF'
#import "p680-d.typ": valor_d
#let valor_b = "B usa " + valor_d
EOF
cat > /tmp/p680-diamante-c.typ <<'EOF'
#import "p680-d.typ": valor_d
#let valor_c = "C usa " + valor_d
EOF
cat > /tmp/p680-diamante-a.typ <<'EOF'
#import "p680-diamante-b.typ": valor_b
#import "p680-diamante-c.typ": valor_c
#valor_b #valor_c
EOF
lab/typst-original/target/release/typst compile /tmp/p680-diamante-a.typ /tmp/p680-diamante-vanilla.pdf
pdftotext /tmp/p680-diamante-vanilla.pdf -

./target/release/typst /tmp/p680-diamante-a.typ /tmp/p680-diamante-cristalino.pdf
pdftotext /tmp/p680-diamante-cristalino.pdf -
```

Confirmar que `D` é importado com sucesso pelos dois ramos (`B` e `C`), sem ser confundido com um ciclo, e que o texto final bate com o vanilla.

### Critério de fecho da verificação

- [ ] Cadeia de três ficheiros testada, texto idêntico ao vanilla.
- [ ] Diamante testado, texto idêntico ao vanilla, sem falso positivo de ciclo.

---

## Decisão

Se ambos os casos funcionarem sem problema: confirmar e registar como parte da cobertura de P679, sem código a mudar.

Se algum falhar (por exemplo, o diamante a ser tratado como ciclo por engano): corrigir `Route`/`Route::contains` para distinguir correctamente "está activamente a ser avaliado agora, na cadeia actual" de "já foi avaliado nalgum ramo anterior, mas já terminou".

---

## Validação

```bash
cargo test --workspace
crystalline-lint .
```

Adicionar testes automatizados para os dois casos (cadeia de três, diamante), não deixar como só verificação manual.

---

## Critério de fecho do passo

- [ ] Cadeia de três ficheiros e diamante testados directamente.
- [ ] Se algum falhar: corrigido e testado de novo.
- [ ] Testes automatizados adicionados para os dois casos.
- [ ] Sem regressão em `cargo test --workspace`.
- [ ] `crystalline-lint .` limpo.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p680.md`, com hash do commit.

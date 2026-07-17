---
# P679 — Implementar `#import` de ficheiros locais (nível 5a de P678)

> **Passo:** 679
> **Data:** 2026-07-10
> **Foco:** P678 confirmou que `eval_module_import` é um stub que devolve erro sempre, mesmo para `#import "ficheiro-local.typ": foo`. Isto é o pré-requisito absoluto de tudo o resto na sequência de pacotes (P678, níveis 2-4), e é útil por si só — muitos documentos Typst reais usam `#import` entre ficheiros do mesmo projecto, sem pacotes nenhuns envolvidos.
> **Tipo:** Sonda mínima + Implementação.
> **Tamanho:** L.
> **ADR-0108 EM VIGOR.**
> **Dependências:** P678 (sonda e mapa), `eval_module_include` já implementado (`01_core/src/rules/eval/modules.rs:33`, reaproveitável como referência de estrutura).

---

## Sonda mínima

### Confirmar o comportamento completo do vanilla para `#import`

```bash
cat > /tmp/p679-utils.typ <<'EOF'
#let saudacao(nome) = "Olá, " + nome + "!"
#let PI = 3.14159
EOF

cat > /tmp/p679-import-simples.typ <<'EOF'
#import "p679-utils.typ": saudacao
#saudacao("Mundo")
EOF

cat > /tmp/p679-import-wildcard.typ <<'EOF'
#import "p679-utils.typ": *
#saudacao("Mundo") #PI
EOF

cat > /tmp/p679-import-rename.typ <<'EOF'
#import "p679-utils.typ": saudacao as ola
#ola("Mundo")
EOF

cat > /tmp/p679-import-nome-modulo.typ <<'EOF'
#import "p679-utils.typ"
#p679-utils.saudacao("Mundo")
EOF

cat > /tmp/p679-import-with-name.typ <<'EOF'
#import "p679-utils.typ" as u
#u.saudacao("Mundo")
EOF

for f in p679-import-simples p679-import-wildcard p679-import-rename p679-import-nome-modulo p679-import-with-name; do
  echo "=== $f ==="
  lab/typst-original/target/release/typst compile /tmp/$f.typ /tmp/$f-vanilla.pdf
  pdftotext /tmp/$f-vanilla.pdf -
done
```

Confirmar todas as formas de `#import` que o vanilla suporta, com o texto extraído de cada uma.

### Confirmar detecção de ciclos

```bash
cat > /tmp/p679-ciclo-a.typ <<'EOF'
#import "p679-ciclo-b.typ": x
EOF
cat > /tmp/p679-ciclo-b.typ <<'EOF'
#import "p679-ciclo-a.typ": x
EOF
lab/typst-original/target/release/typst compile /tmp/p679-ciclo-a.typ /tmp/p679-ciclo.pdf
```

Confirmar a mensagem de erro exacta para importação cíclica.

### Confirmar se `Route::contains` (já usado para ciclos de função) se aplica directamente a ciclos de ficheiro

```bash
grep -n "struct Route\|fn contains" 01_core/src/rules/eval/*.rs 01_core/src/entities/*.rs | head -10
```

Confirmar se esta estrutura já rastreia ficheiros, ou só chamadas de função — se for só chamadas, pode precisar de uma extensão, não reaproveitamento directo.

### Critério de fecho da sonda mínima

- [ ] As cinco formas de `#import` testadas contra o vanilla, com texto extraído de cada uma.
- [ ] Mensagem de erro de ciclo confirmada.
- [ ] Confirmado se `Route::contains` já cobre ficheiros ou precisa de extensão.

---

## Implementação

`eval_module_import` (`01_core/src/rules/eval/modules.rs:25`) precisa de:

1. Resolver o caminho do ficheiro (relativo ao ficheiro actual), reutilizando o mecanismo já usado por `eval_module_include`.
2. Avaliar o ficheiro importado num escopo de módulo próprio (não o escopo do ficheiro que importa).
3. Suportar as cinco formas confirmadas pela sonda: import com lista de nomes, wildcard (`*`), rename (`as`), nome de módulo implícito, nome de módulo explícito (`as nome`).
4. Detectar ciclos de importação, com a mensagem de erro confirmada pela sonda.
5. Devolver um `Value::Module` (ou equivalente) que o resto do avaliador já saiba usar para acesso a campos (`modulo.campo`).

### Critério de fecho da implementação

- [ ] As cinco formas de `#import` funcionam, testadas contra os mesmos documentos usados na sonda.
- [ ] Detecção de ciclos funciona, com erro claro.
- [ ] `#include` (já implementado) sem regressão.
- [ ] Import de ficheiro inexistente produz erro claro (confirmar mensagem do vanilla para este caso também).

---

## Validação

```bash
for f in p679-import-simples p679-import-wildcard p679-import-rename p679-import-nome-modulo p679-import-with-name; do
  ./target/release/typst /tmp/$f.typ /tmp/$f-cristalino.pdf
  pdftotext /tmp/$f-cristalino.pdf -
done
```

Comparar cada saída com a do vanilla já obtida na sonda.

```bash
./target/release/typst /tmp/p679-ciclo-a.typ /tmp/p679-ciclo-cristalino.pdf
echo "Exit code: $?"
```

```bash
cargo test --workspace
crystalline-lint .
```

---

## Critério de fecho do passo

- [ ] Sonda mínima completa, cinco formas de `#import` confirmadas contra o vanilla.
- [ ] Implementação cobre as cinco formas, testada contra o vanilla.
- [ ] Ciclos detectados com erro claro.
- [ ] `#include` sem regressão.
- [ ] Sem regressão em `cargo test --workspace`.
- [ ] `crystalline-lint .` limpo.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p679.md`, com hash do commit.

---

## Próximo passo

P-β (nível 5b + 3 de P678): pacotes só offline, usando a cache local já existente no sistema, reaproveitando o `#import` agora implementado.

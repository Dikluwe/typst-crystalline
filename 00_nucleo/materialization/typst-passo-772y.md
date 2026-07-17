---
# P772y — Espaçamento automático por `MathClass` + `math.class(...)`

> **Passo:** 772y
> **Data:** 2026-07-17
> **Foco:** P772w confirmou `math.class(...)` ausente e, ao investigar, encontrou que `MathClass` não chega ao motor de layout do cristalino hoje (`grep -rln MathClass 01_core/src/rules/layout` sem resultados) — só existe em `entities/math_class.rs` e no parsing/lexing. Isso sugere que o espaçamento automático entre símbolos matemáticos por classe (relação, operador binário, abertura/fechamento, etc.) pode não existir, ou usa outro mecanismo ainda não identificado. Este passo primeiro confirma o estado real do espaçamento automático no cristalino, depois decide se implementar `math.class()` cabe num só passo ou precisa de fase própria.
> **Tipo:** Sonda de arquitetura + Decisão registada (regra 1) + Implementação condicional ao alcance.
> **Tamanho:** L — pode ser XL se o espaçamento por classe não existir de todo.
> **ADR-0108 EM VIGOR** — confirmar o mecanismo real antes de assumir que falta, dado P772w já ter registado incerteza ("ou usa outro mecanismo não descoberto nesta sonda").
> **Dependências:** P772w (achado original, pista já registada).

---

## Sonda — o espaçamento automático por classe existe no cristalino?

```bash
grep -rn "MathClass\|Class::Relation\|Class::Binary\|Class::Opening" 01_core/src/rules/layout/*.rs 01_core/src/rules/math/**/*.rs 2>/dev/null
```

Se não houver nenhuma ocorrência (confirmando a suspeita de P772w): o espaçamento entre símbolos em modo matemático hoje é fixo/sem classe, ou usa alguma heurística diferente — confirmar qual, testando casos que deveriam ter espaçamento diferente por classe:

```bash
cat > /tmp/p772y-spacing-test.typ <<'EOF'
$ a = b $
$ a + b $
$ (a) $
EOF
lab/typst-original/target/release/typst compile /tmp/p772y-spacing-test.typ /tmp/p772y-vanilla.pdf
./target/release/typst compile /tmp/p772y-spacing-test.typ /tmp/p772y-cristalino.pdf
mutool trace /tmp/p772y-vanilla.pdf > /tmp/p772y-trace-vanilla.txt
mutool trace /tmp/p772y-cristalino.pdf > /tmp/p772y-trace-cristalino.txt
```

Comparar o espaçamento ao redor de `=` (relação) vs `+` (binário) vs `(`/`)` (abertura/fechamento) — o vanilla usa larguras diferentes conforme a classe adjacente (tabela de espaçamento matemático padrão, tipo TeX). Confirmar se o cristalino já produz espaçamento visualmente diferenciado (por algum mecanismo não óbvio) ou se é uniforme.

### Mecanismo do vanilla

```bash
grep -n "fn spacing\|MathClass::" lab/typst-original/crates/typst-layout/src/math/*.rs 2>/dev/null | head -30
```

Confirmar a tabela de espaçamento exata (pares de classe → largura) antes de replicar.

---

## Decisão de âmbito

| Cenário | Decisão |
|---|---|
| Espaçamento por classe já existe (por outro mecanismo), só falta `math.class()` para override manual | Implementar `math.class()` diretamente, aproveitando o mecanismo existente |
| Espaçamento por classe não existe, layout usa espaçamento fixo/heurística simples | Decisão maior — implementar a tabela de espaçamento por classe é pré-requisito para `math.class()` fazer sentido; avaliar se cabe neste passo (tamanho L declarado) ou precisa de fase própria (L0 e passo dedicado, análogo à decisão de P763h para inline/block) |

Registar a decisão com evidência da sonda, não assumida do cabeçalho deste prompt.

---

## Implementação (conforme a decisão)

Se a tabela de espaçamento por classe precisar de ser criada:
1. Confirmar onde no pipeline de layout matemático cada símbolo/elemento já tem (ou pode obter) sua `MathClass` — via `entities/math_class.rs`, já existente para parsing.
2. Implementar a tabela de espaçamento (pares classe→classe, largura), replicando os valores do vanilla.
3. Aplicar no motor de layout matemático, no ponto onde elementos adjacentes são posicionados.

Depois (ou em paralelo, se a tabela já existir por outro caminho):
4. Implementar `math.class(class, body)` — força a classe de um símbolo/expressão, override do valor inferido automaticamente.

---

## Validação

```bash
cat > /tmp/p772y-class-test.typ <<'EOF'
#let loves = math.class("relation", sym.suit.heart)
$x loves y$
EOF
./target/release/typst compile /tmp/p772y-class-test.typ /tmp/p772y-class-cristalino.pdf
lab/typst-original/target/release/typst compile /tmp/p772y-class-test.typ /tmp/p772y-class-vanilla.pdf
```

Confirmar que compila e que o espaçamento ao redor de `loves` bate com o de uma relação real (`=`), não com o espaçamento por omissão do símbolo.

```bash
cargo test --workspace
crystalline-lint .
```

Reconfirmar que os testes de math já existentes (P299-301, P765b) continuam verdes — este passo mexe num mecanismo central de layout matemático.

---

## Critério de fecho do passo

- [ ] Estado real do espaçamento automático por classe confirmado (existe por outro mecanismo, ou não existe).
- [ ] Decisão de âmbito registada com evidência.
- [ ] Se implementado: tabela de espaçamento por classe confirmada contra o vanilla, aplicada no motor de layout matemático.
- [ ] `math.class(...)` implementado e testado com override real (caso do relatório de P772w).
- [ ] Sem regressão em testes de math existentes.
- [ ] `cargo test --workspace` verde.
- [ ] `crystalline-lint .` zero violações.
- [ ] L0 de layout matemático atualizado antes do código.
- [ ] Relatório em `00_nucleo/diagnosticos/paridade-producao-p772y.md`.

---

## Próximo passo

Se o âmbito revelar-se maior que este passo (tabela de espaçamento inteira): devolver para decisão antes de prosseguir, não forçar. Se fechado: restam `image::pdf` e fallback de fontes matemáticas como débitos de P772w, ou encerrar a série P765a-P772y.

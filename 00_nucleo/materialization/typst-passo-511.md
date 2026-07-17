---

# P511 — Sonda A.0: Math Elements Granulares (Binom, Class, Limits, Mid, Primes, Scripts, Stretch)

> **Passo:** 511
> **Data:** 2026-06-30
> **Foco:** Verificar empiricamente, antes de qualquer spec de materialização, se os 7 elementos math granulares existem no cristalino. Não declarar conclusão — medir antes de decidir.
> **Tipo:** Sonda A.0 (diagnóstico imutável, ADR-0114).
> **Tamanho:** S (~15 min de sonda).
> **ADR-0107 ACEITE** — paridade é linguagem, não mecânica.
> **ADR-0108 ACEITE** — medir antes de decidir; língua vs mecânica explícita.
> **ADR-0114 ACEITE** — sonda A.0 antes da spec; gate duro.
> **Dependências:** P510 (math styles fechados), P490 (math base funciona).

---

## 1. Contexto

O diagnóstico P508 identificou que **7 elementos math granulares** estão ausentes no cristalino. Antes de redigir uma spec de materialização (ADR-0114), é obrigatório correr a **sonda A.0** para verificar:

1. Se os elementos **já existem** parcialmente (substrato escondido).
2. Se o **parser** já reconhece a sintaxe (mas o eval/ layout falha).
3. Se o **layout engine** já tem stubs (mas não expõe).
4. Se a **classificação** como "ausente" está correta (língua vs mecânica, ADR-0107).

---

## 2. Metodologia da Sonda

### 2.1 Comandos de Verificação

```bash
cd /home/dikluwe/Documentos/Antigravity/typst-crystalline

# Sonda 1: Parser reconhece a sintaxe?
echo '$binom(n, k)$' > /tmp/sonda_binom.typ
cargo run --release -p typst-wiring -- compile /tmp/sonda_binom.typ /tmp/out.pdf 2>&1
# Se erro for "unknown variable: binom" → parser NÃO reconhece (ausente no scope)
# Se erro for "cannot call binom" → parser reconhece, mas eval/constructor falha
# Se compila → elemento já existe (substrato escondido)

echo '$class("unary", x)$' > /tmp/sonda_class.typ
cargo run --release -p typst-wiring -- compile /tmp/sonda_class.typ /tmp/out.pdf 2>&1

echo '$limits(sum, sub: i=0, sup: n)$' > /tmp/sonda_limits.typ
cargo run --release -p typst-wiring -- compile /tmp/sonda_limits.typ /tmp/out.pdf 2>&1

echo '$mid(|)$' > /tmp/sonda_mid.typ
cargo run --release -p typst-wiring -- compile /tmp/sonda_mid.typ /tmp/out.pdf 2>&1

echo "x'" > /tmp/sonda_primes.typ
cargo run --release -p typst-wiring -- compile /tmp/sonda_primes.typ /tmp/out.pdf 2>&1

echo '$scripts(x, sub: i, sup: j)$' > /tmp/sonda_scripts.typ
cargo run --release -p typst-wiring -- compile /tmp/sonda_scripts.typ /tmp/out.pdf 2>&1

echo '$stretch(left: "|", x, right: "|")$' > /tmp/sonda_stretch.typ
cargo run --release -p typst-wiring -- compile /tmp/sonda_stretch.typ /tmp/out.pdf 2>&1
```

### 2.2 Sonda 2: Variants no Content

```bash
# Verificar se os variants já existem no enum Content
rg -n "MathBinom\|MathClass\|MathLimits\|MathMid\|MathPrimes\|MathScripts\|MathStretch" src/entities/content.rs --type rs
# Se encontrar → substrato escondido (variant existe mas não expõe)
# Se não encontrar → realmente ausente
```

### 2.3 Sonda 3: Construtores Nativos

```bash
# Verificar se os construtores já existem no stdlib
rg -n "native_binom\|native_class\|native_limits\|native_mid\|native_primes\|native_scripts\|native_stretch" src/stdlib/ --type rs
# Se encontrar → substrato escondido
# Se não encontrar → realmente ausente
```

### 2.4 Sonda 4: Layout Handlers

```bash
# Verificar se o layout engine já tem handlers
rg -n "MathBinom\|MathClass\|MathLimits\|MathMid\|MathPrimes\|MathScripts\|MathStretch" src/engine/layout/ --type rs
# Se encontrar → layout parcialmente implementado
# Se não encontrar → layout ausente
```

---

## 3. Critérios de Classificação (ADR-0107: Língua vs Mecânica)

Para cada elemento, classificar antes de decidir:

| Elemento | Sintaxe (língua) | Semântica (língua) | Morfologia (língua) | Mecânica (diverge) |
|----------|------------------|--------------------|---------------------|-------------------|
| `binom(n, k)` | `$binom(n, k)$` | Coeficiente binomial | Dois operandos empilhados | Estrutura interna do `Content` |
| `class("unary", x)` | `$class("unary", x)$` | Classe de operador | String + body | Estrutura interna do `Content` |
| `limits(sum, sub: i, sup: n)` | `$limits(sum, sub: i, sup: n)$` | Limites em operadores | Body + sub + sup | Estrutura interna do `Content` |
| `mid(|)` | `$mid(|)$` | Delimitador de meio | Body | Estrutura interna do `Content` |
| `x'` | `$x'$` | Primos | Body + count | Estrutura interna do `Content` |
| `scripts(x, sub: i, sup: j)` | `$scripts(x, sub: i, sup: j)$` | Subscripts/superscripts | Base + sub + sup + pre_sub + pre_sup | Estrutura interna do `Content` |
| `stretch(left: "|", x, right: "|")` | `$stretch(left: "|", x, right: "|")$` | Delimitadores esticáveis | Left + body + right + size | Estrutura interna do `Content` |

**Conclusão prévia:** Todos os 7 elementos são **língua** (sintaxe, semântica, morfologia). A mecânica (estrutura interna do `Content`) diverge de propósito (ADR-0107, ADR-0026). A paridade exige que a sintaxe produza o resultado semântico correto, não que a estrutura interna seja idêntica ao vanilla.

---

## 4. Resultado da Sonda (a preencher após execução)

| Elemento | Parser | Eval (constructor) | Layout | Classificação | Decisão |
|----------|--------|-------------------|--------|---------------|---------|
| `binom` | | | | | |
| `class` | | | | | |
| `limits` | | | | | |
| `mid` | | | | | |
| `primes` | | | | | |
| `scripts` | | | | | |
| `stretch` | | | | | |

**Legenda:**
- `OK` — funciona
- `FAIL` — falha com erro descritivo
- `AUSENTE` — não reconhecido (unknown variable)
- `SUBSTRATO` — existe parcialmente (variant/construtor/layout stub)

---

## 5. Decisão Derivada da Sonda

**Se a sonda revelar que ≥4 elementos já existem parcialmente (substrato):**
- P511 = **Verificação Retroativa** (ADR-0114).
- O passo não produz código novo além do que a verificação exigir.
- O artefacto principal é o relatório de verificação.

**Se a sonda revelar que ≥4 elementos estão completamente ausentes:**
- P511 = **Spec de Materialização** (ADR-0114, gate duro satisfeito).
- O passo produz código novo.

**Se a sonda revelar mistura (alguns substrato, alguns ausentes):**
- P511a = Verificação Retroativa (substratos)
- P511b = Spec de Materialização (ausentes)

---

## 6. Critério de Fecho da Sonda

- [ ] 7 comandos de sonda executados (parser).
- [ ] 4 comandos de sonda executados (variants, construtores, layout, classificação).
- [ ] Tabela de resultados preenchida (seção 4).
- [ ] Decisão derivada da sonda documentada (seção 5).
- [ ] Classificação língua vs mecânica explícita para cada elemento (seção 3).
- [ ] `00_nucleo/diagnosticos/sonda-p511-math-elements-granulares.md` produzido.
- [ ] Se gate duro satisfeito (≥4 ausentes), P511-spec de materialização redigida.
- [ ] Se gate duro NÃO satisfeito (≥4 substratos), P511-verificação retroativa redigida.

---

## A. Apêndice — Comandos de Reprodução

```bash
cd /home/dikluwe/Documentos/Antigravity/typst-crystalline

# Sonda completa
for elem in binom class limits mid primes scripts stretch; do
  echo "=== Sonda: $elem ==="
  case $elem in
    binom) echo '$binom(n, k)$' > /tmp/sonda.typ ;;
    class) echo '$class("unary", x)$' > /tmp/sonda.typ ;;
    limits) echo '$limits(sum, sub: i=0, sup: n)$' > /tmp/sonda.typ ;;
    mid) echo '$mid(|)$' > /tmp/sonda.typ ;;
    primes) echo "x'" > /tmp/sonda.typ ;;
    scripts) echo '$scripts(x, sub: i, sup: j)$' > /tmp/sonda.typ ;;
    stretch) echo '$stretch(left: "|", x, right: "|")$' > /tmp/sonda.typ ;;
  esac
  cargo run --release -p typst-wiring -- compile /tmp/sonda.typ /tmp/out.pdf 2>&1 | head -5
  echo "---"
done

# Sonda de variants
rg -n "MathBinom\|MathClass\|MathLimits\|MathMid\|MathPrimes\|MathScripts\|MathStretch" src/entities/content.rs --type rs

# Sonda de construtores
rg -n "native_binom\|native_class\|native_limits\|native_mid\|native_primes\|native_scripts\|native_stretch" src/stdlib/ --type rs

# Sonda de layout
rg -n "MathBinom\|MathClass\|MathLimits\|MathMid\|MathPrimes\|MathScripts\|MathStretch" src/engine/layout/ --type rs
```

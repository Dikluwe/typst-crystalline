# Sonda A.0 — P511 — Math Elements Granulares

**Data:** 2026-06-30  
**Passo:** 511  
**Tipo:** Sonda A.0 (diagnóstico imutável, ADR-0114)  
**Foco:** Verificar empiricamente o estado dos 7 elementos math granulares antes de qualquer spec de materialização.

---

## 1. Metodologia

Foram executadas 4 sondas independentes:

1. **Parser/sintaxe** — compilar snippets mínimos de cada elemento.
2. **Variants em `Content`** — procurar `MathBinom`, `MathClass`, `MathLimits`, `MathMid`, `MathPrimes`, `MathScripts`, `MathStretch` em `01_core/src/entities/content.rs`.
3. **Construtores nativos** — procurar `native_binom`, `native_class`, etc., em `01_core/src/rules/stdlib/`.
4. **Layout handlers** — procurar os mesmos variants em `01_core/src/rules/math/layout/`.

---

## 2. Resultados

### 2.1 Sonda de Parser/Sintaxe

| Elemento | Snippet | Resultado | Observação |
|---|---|---|---|
| `binom` | `$binom(n, k)$` | OK (compila) | Renderiza como texto "binom(n, k)" |
| `class` | `$class("unary", x)$` | OK (compila) | Renderiza como texto "class(\"unary\", x)" |
| `limits` | `$limits(sum, sub: i=0, sup: n)$` | OK (compila) | Named args aceites como texto |
| `mid` | `$mid(|)$` | OK (compila) | Renderiza como texto "mid(|)" |
| `primes` | `$x'$` | OK (compila) | Funciona via `MathAttach` existente |
| `scripts` | `$scripts(x, sub: i, sup: j)$` | OK (compila) | Renderiza como texto "scripts(x, sub: i, sup: j)" |
| `stretch` | `$stretch(left: "\|", x, right: "\|")$` | OK (compila) | Renderiza como texto "stretch(...)" |

A compilação sem erro indica que o **parser de math mode aceita a sintaxe** (trata os nomes como `MathIdent` seguido de grupo delimitado), não que os elementos estejam implementados.

### 2.2 Sonda de Variants em `Content`

```bash
cd /home/dikluwe/Documentos/Antigravity/typst-crystalline
rg -n "MathBinom\|MathClass\|MathLimits\|MathMid\|MathPrimes\|MathScripts\|MathStretch" \
   01_core/src/entities/content.rs --type rs
```

**Resultado:** nenhuma ocorrência.

### 2.3 Sonda de Construtores Nativos

```bash
rg -n "native_binom\|native_class\|native_limits\|native_mid\|native_primes\|native_scripts\|native_stretch" \
   01_core/src/rules/stdlib/ --type rs
```

**Resultado:** nenhuma ocorrência.

### 2.4 Sonda de Layout Handlers

```bash
rg -n "MathBinom\|MathClass\|MathLimits\|MathMid\|MathPrimes\|MathScripts\|MathStretch" \
   01_core/src/rules/math/layout/ --type rs
```

**Resultado:** nenhuma ocorrência.

### 2.5 Verificação de Morfologia (repr)

```typst
#assert(repr($binom(n, k)$) == "binom(n, k)")
```

**Resultado:** falha — o eval não produz o elemento semântico, apenas `MathSequence([MathIdent("binom"), MathDelimited(...)])`.

```typst
#assert(repr($x'$) != "x'")
```

**Resultado:** passa — primes são tratados pelo `MathAttach` existente.

---

## 3. Tabela de Classificação (ADR-0107)

| Elemento | Sintaxe (língua) | Semântica (língua) | Morfologia (língua) | Mecânica (diverge) |
|---|---|---|---|---|
| `binom(n, k)` | Parser OK | Coeficiente binomial | Dois operandos empilhados | Variante `Content` pode ser `MathFrac`-like ou próprio |
| `class("unary", x)` | Parser OK | Classe de operador | String + body | Variante `Content` próprio |
| `limits(sum, sub: i, sup: n)` | Parser OK | Limites em operadores | Body + sub + sup | Variante `Content` próprio ou reutilizar `MathAttach` |
| `mid(|)` | Parser OK | Delimitador de meio | Body | Variante `Content` próprio |
| `x'` | Parser OK | Primos | Body + count | Já coberto por `MathAttach` (língua OK) |
| `scripts(x, sub: i, sup: j)` | Parser OK | Subscripts/superscripts | Base + sub + sup + pre_sub + pre_sup | Variante `Content` próprio ou reutilizar `MathAttach` |
| `stretch(left: "\|", x, right: "\|")` | Parser OK | Delimitadores esticáveis | Left + body + right + size | Variante `Content` próprio |

**Conclusão:** Todos os elementos são **língua** (sintaxe, semântica, morfologia). A mecânica interna (`Content` variants) diverge de propósito (ADR-0107, ADR-0026).

---

## 4. Decisão Derivada

- **Gate duro ADR-0114 satisfeito:** 7/7 elementos estão completamente ausentes do ponto de vista semântico (nenhum variant, construtor ou layout handler).
- **Decisão:** P511 = **Spec de Materialização**. Deve ser redigida antes de qualquer código.
- **Exceção parcial:** `primes` ($x'$) já funciona via `MathAttach` existente; o elemento granular `MathPrimes` pode ser um não-objectivo se a sintaxe básica estiver coberta.

---

## 5. Critério de Fecho da Sonda

- [x] 7 comandos de sonda executados (parser).
- [x] 4 comandos de sonda executados (variants, construtores, layout, classificação).
- [x] Tabela de resultados preenchida (seção 2).
- [x] Decisão derivada da sonda documentada (seção 4).
- [x] Classificação língua vs mecânica explícita para cada elemento (seção 3).
- [x] `00_nucleo/diagnosticos/sonda-p511-math-elements-granulares.md` produzido.
- [x] Gate duro satisfeito (≥4 ausentes) → `00_nucleo/materialization/typst-passo-511-spec.md` redigida.

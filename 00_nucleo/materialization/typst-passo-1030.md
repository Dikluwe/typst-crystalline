# Passo 1030 — Achado C (P998): `#set math.*` ignorado em silêncio para elementos além de `equation`

**Tipo**: Investigar alcance → classificar gate → corrigir se aprovado. **Prioridade
sobre o resto do Bloco 3** — bug funcional real (falha silenciosa), não lacuna de
documentação, catalogado desde o Passo 998 e nunca tratado.
**Confirmado no Passo 1029**: `eval/rules.rs:870-905` só trata `#set math.equation(...)`.
Qualquer outro `#set math.<elemento>(...)` cai no fallback de linha 1319, que emite
`warning: set: target '' ainda não suportado` e **ignora silenciosamente** os parâmetros
— o documento compila, sem erro visível ao utilizador além do warning, e o comportamento
pedido nunca acontece.

**Caso mínimo confirmado**:
```typst
#set math.mat(delim: "[")
$ mat(1, 2; 3, 4) $
```
Vanilla: compila, delimitador `[`. Cristalino: warning + delimitador continua a ser `(`.

**Pré-condição**: `git status` limpo. HEAD ≥ Passo 1029.

---

## Fase A — Medir o alcance completo antes de desenhar o fix

1. Listar todos os elementos math que aceitam `#set` no vanilla — não presumir que é só
   `mat`/`cases`/`vec` (achado original do P998 mencionava também `frac`/`lr`).
   ```
   grep -rn 'pub struct.*Elem' lab/typst-original/crates/typst-library/src/math/
   ```
   Para cada um, confirmar quais têm parâmetros configuráveis via `#set` (nem todos têm).
2. Para cada elemento com `#set` no vanilla, reproduzir o caso mínimo (como o de `mat`
   acima) e confirmar que o cristalino tem o mesmo defeito — não presumir que o padrão se
   repete identicamente para todos.
3. Confirmar se `#set math.equation(...)` (o único caminho que já funciona) dá pistas de
   como generalizar — é código específico a `equation` que precisa de ser copiado e
   adaptado por elemento, ou há um mecanismo mais genérico de `set`-rule para elementos
   que só precisa de ser estendido à lista de alvos math?

## Fase B — Classificar o gate

Mudança de comportamento por defeito (parâmetros que hoje são ignorados passam a ter
efeito) → categoria 2/3, ADR-0127. L0 antes de código.

```
Dado #set math.mat(delim: "[") seguido de mat(1, 2; 3, 4)
Quando renderizado
Então usa o delimitador "[", batendo com vanilla — repetir para cada parâmetro relevante
  de cada elemento identificado na Fase A

Dado #set math.cases(...) [parâmetros relevantes per Fase A]
Quando renderizado
Então aplica, batendo com vanilla

Dado #set math.equation(...) (caminho já funcional)
Quando renderizado
Então comportamento inalterado — guarda de não-regressão directa
```

Não-regressão: todos os testes math existentes, em particular os que já tocam
`cases`/`mat`/`vec`/`frac`/`lr` sem `#set` (comportamento por defeito sem override).

## Fase C — Implementar (só após gate) e validar

```
crystalline-lint .
cargo test --workspace
```
Decalque contra `00_nucleo/corpus-docs/math/` (`mat.typ`, `cases.typ`, e outros ficheiros
do corpus que exercitem `#set` nesses elementos, se existirem — se não existirem casos de
`#set` no corpus actual, é um sinal de que o corpus também tinha esta lacuna, considerar
acrescentar).

---

## Resultado esperado

`#set math.<elemento>(...)` funcional para todos os elementos que o vanilla suporta, não
só `equation`. Warning silencioso substituído por comportamento real, ou por erro
explícito nos casos que genuinamente não são suportados (nunca mais "aceite e ignorado
sem avisar de forma útil").

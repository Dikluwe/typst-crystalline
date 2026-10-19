# L0 — Passo 1091: Auditoria — Degrau Residual de ~0.055pt Após `)` em Termos com Attach Duplo

**Gate**: nenhum — auditoria, sem código alterado.

**Base**: análise dos outputs brutos do P1090 (`pdftotext -bbox-layout`,
Crystalline vs Vanilla, Equação 3 completa). O P1090 reduziu o erro de
`g_i`/`∇g_i(x*)` de `+0.275pt` para um degrau residual de `~0.055pt`, que
continua a acumular (2 ocorrências → `~0.110pt`, consistente com o resíduo de
página já reportado como "alinhamento de bloco", que provavelmente é este
mesmo bug, não uma causa separada).

**Achado que contradiz a hipótese anterior**: o degrau aparece **igualmente**
depois de `g_i(x*)` **e** depois de `h_j(x*)` — mas o dump real da tabela da
fonte (`MathItalicsCorrectionInfo`) confirma `IC(g)=25du=0.275pt` e `h` **não
consta na cobertura MATH → IC=0**. Se o degrau fosse sobre magnitude de
`italics_correction`, `h` não devia ter degrau nenhum. Ter o mesmo degrau que
`g` sugere que a causa **não é** (ou não é só) `italics_correction` — pode ser
algo estrutural, comum aos dois por partilharem a mesma sintaxe
(`identificador + subscrito` seguido de `(x^*)`, um argumento entre
parênteses que por sua vez tem o seu próprio attach `x^*`).

---

## 1. Não presumir a causa — isolar por estrutura, não por glifo

Testar variantes que separam as duas hipóteses (IC do identificador vs
estrutura sintáctica de parênteses com sub-attach interno):

1. `$ g_i $` sem parênteses — o degrau (se for sobre IC de `g`) devia
   aparecer aqui, mesmo sem `(x*)` a seguir. Já testado no P1090 como
   "exacto" — confirmar se esse teste usou o mesmo nível de precisão
   (`pdftotext -bbox-layout` bruto) ou a tabela reformatada que já se
   provou não confiável.
2. `$ g_i (y) $` — parêntese com argumento **sem** sub/sobrescrito interno
   (`y` simples, não `x^*`). Se o degrau desaparecer aqui, a causa é
   especificamente sobre o argumento entre parênteses ter o seu próprio
   attach, não sobre `g` ter IC≠0.
3. `$ h_j (y) $` — mesmo teste com `h` (IC=0 confirmado), argumento simples.
   Se não houver degrau aqui nem no caso 2, a hipótese estrutural (parênteses
   com sub-attach interno) fica mais forte que a hipótese de IC.
4. `$ w_k (x^*) $` — usar um identificador de IC=0 confirmado (`w`, ou outro
   a confirmar na tabela da fonte) com parêntese `(x^*)` completo (sub-attach
   interno presente). Se o degrau aparecer aqui também, confirma que a causa
   é estrutural (não IC), porque `w` não tem correcção itálica nenhuma a
   aplicar.

## 2. Ler o código real do fecho de parêntese com attach interno

```bash
grep -rn "fn.*paren\|fn.*close\|delimit" 01_core/src/compiler/math/layout/
```

Confirmar se existe algum mecanismo de kerning/ajuste específico para
delimitadores (`(`/`)`) que envolvem conteúdo com o seu próprio nó de attach
— candidato a onde um valor de ~0.055pt poderia estar a ser mal calculado ou
duplicado, independente de `italics_correction`.

## 3. Verificar se 0.055pt tem alguma correspondência com outra métrica conhecida

Não presumir que é fracção de `italics_correction` (0.275/5=0.055, pode ser
coincidência numérica, não causa real). Verificar contra outras constantes já
catalogadas nesta investigação: `space_after_script` (0.616pt),
`gap_min` (1.76pt, do P1088), ou alguma métrica de delimitador (altura/kerning
de parêntese) ainda não mapeada nesta linha de investigação.

## Critério de conclusão

- 4 casos do §1 medidos com `pdftotext -bbox-layout` bruto (não tabela
  reformatada) — mesma disciplina que expôs o problema desta vez.
- Código real do fecho de parêntese com attach interno lido.
- Causa distinguida entre "IC do identificador" vs "estrutura de parênteses
  com sub-attach interno" — com prova, não suposição.
- Se a causa não for nenhuma das duas hipóteses testadas, reportar isso
  explicitamente, não forçar uma das duas.

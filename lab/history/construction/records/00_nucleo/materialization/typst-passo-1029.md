# Passo 1029 — Bloco 3 (P1021), família math: 22 achados graves

**Tipo**: Investigar cada achado → citar/corrigir se verificável, ou classificar como
decisão interna se não. Alguns podem escalar para passo de gate próprio (padrão P1027/
P1028), se a investigação revelar divergência real de output.
**Base**: catálogo do Passo 1021, os 22 achados graves de Bloco 3 na família math (lista
completa em `temp/p1024/bloco3_graves.md`, já produzida — usar essa lista, não
re-derivar).
**Pré-condição**: `git status` limpo. HEAD ≥ Passo 1028.

---

## Lição obrigatória, antes de qualquer achado (achado meta do Passo 1026)

**Antes de reler o vanilla para verificar uma afirmação, `grep` o nome da
função/mecanismo em `tests.rs`.** Se existir um teste-guarda com `file:line` do vanilla já
citado (como `p945_grid_delim_target_du_e_altura_vezes_1_1` tinha, e dois passos
ignoraram antes de o descobrirem da forma cara), a resposta já está lá, verificada e
verde. Só reler o vanilla de raiz quando não houver guarda, ou quando a guarda for
insuficiente para o achado específico.

## Lição obrigatória, sobre o alvo de paridade (Passo 1025)

O binário de referência é o vanilla ratificado (upstream/main, hash confirmado em
`lab/typst-original` — reconfirmar por `--version` antes de medir, não presumir). **Não**
é a tag 0.15.0. `/usr/local/bin/typst` pode estar noutra versão — confirmar antes de usar.

---

## Processo, por achado

Para cada um dos 22:

1. `grep` do mecanismo em `tests.rs` (lição acima). Se houver guarda com vanilla citado:
   ler, confirmar que a afirmação do achado bate ou não com o que a guarda já provou.
   Isto pode fechar o achado sem tocar no binário vanilla de todo.
2. Se não houver guarda: verificar contra `00_nucleo/corpus-docs/math/` (P998) primeiro
   — é mais barato que medir o binário de raiz, e já tem citação verbatim da documentação
   onde existir.
3. Se nem a documentação cobrir o ponto exacto: medição directa do vanilla, **varrendo o
   pipeline inteiro do mecanismo**, não parando no primeiro call site plausível — lição
   directa do erro do P1024 na margem de 10% (parou em `resolve_skewed_frac` por engano,
   sem confirmar que era o construto certo).
4. Classificar o resultado:
   - **Afirmação correcta, só falta citação**: adicionar `file:line`/citação, sem mudar
     texto.
   - **Afirmação errada, é só o L0**: corrigir o L0 para bater com o código real
     (padrão `pagebreak.md`/`outline.md` do P1024).
   - **Afirmação errada, é o código**: **não corrigir aqui** — escalar para passo de gate
     próprio (padrão P1027/P1028: investigar causa, medir alcance, classificar gate,
     implementar só depois de L0 aprovado). Listar como achado a escalar, não implementar
     inline.
   - **Não verificável** (comportamento interno sem equivalente documentado nem vanilla
     medível): marcar "decisão de implementação, não paridade".

## Controlo de confound (lição do P1026)

Se algum achado envolver comparação de output visual/glifo, confirmar primeiro que os
dois binários não têm fontes de referência diferentes a confundir a medição (mesmo
procedimento do P1026 — verificar `typst-assets` fixado em cada lado antes de comparar
píxeis).

## Output

Tabela: achado, classificação final, `file:line`/citação onde aplicável, achados a
escalar para passo próprio (com prioridade sugerida, não implementar).

## Validação

```
crystalline-lint .
cargo test --workspace
```
Zero regressão — este passo só corrige L0/citações; qualquer achado que precise de mudar
código escala para passo próprio, não é implementado aqui.

---

## Resultado esperado

22 achados da família math fechados (citação, correcção de L0, ou escalados com dono).
Restam os 68 fora da família math para passo seguinte.

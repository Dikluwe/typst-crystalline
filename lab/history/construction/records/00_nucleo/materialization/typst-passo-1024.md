# Passo 1024 — Resolver o catálogo do Passo 1021: sistémicos primeiro, depois bugs, depois Bloco 3

**Tipo**: Correcção, em três partes, nesta ordem estrita — não paralelizar entre agentes
sem coordenação, porque a Parte 1 pode reduzir drasticamente o volume da Parte 3 se as
frases sistémicas também aparecerem em achados do Bloco 3.
**Base**: catálogo do Passo 1021 (489 achados, 80 prompts, 134 graves).
**Pré-condição**: `git status` limpo. HEAD ≥ Passo 1023.

---

## Parte 1 — Origem comum das duas frases sistémicas

Antes de corrigir prompt a prompt, confirmar se têm fonte partilhada:

```bash
grep -rln "Comportamento idêntico ao braço atual\|Comportamento (idêntico ao braço atual)" \
  00_nucleo/prompts/entities/elements/
grep -rln "Não-locatável" 00_nucleo/prompts/entities/elements/
```

1. Se as ocorrências vierem de um template partilhado ou de `entities/elements/_comum.md`
   (citado no P1021 como um dos prompts auditados, com 2 achados leves) — corrigir na
   fonte: definir "braço actual" e "Não-locatável" uma vez, com referência clara (qual
   braço de que `match`; que invariante exacto "não-locatável" implica — por exemplo,
   "não implementa `Locatable`, não recebe `Location` na introspecção").
2. Se **não** houver fonte comum (cada prompt escreveu a frase independentemente, por
   convenção não-declarada): a correcção continua a poder ser uma só — escrever a
   definição em `entities/elements/_comum.md` (criando a secção se não existir) e, em
   cada prompt afectado, substituir a frase solta por referência a essa definição comum.
   Mesmo resultado, forma ligeiramente diferente consoante o que a Fase A encontrar.
3. Listar todos os prompts tocados (o P1021 já dá a lista via os achados de Bloco 2 com
   estas duas frases — usar essa lista, não remedir do zero).

**Confirmar antes de fechar**: correr `auditar-spec.md` Bloco 2 outra vez só nesses
prompts, para confirmar que a ambiguidade desapareceu (não presumir que apontar para uma
definição resolve — verificar que resolve mesmo).

## Parte 2 — Os dois achados tipo-bug

### `pagebreak.md` — contradição `Hash`

1. Ler `entities/elements/pagebreak.md` e o código real (`PagebreakElem`, provavelmente
   `entities/elements/pagebreak.rs` ou equivalente pós-fatiamentos).
2. Confirmar qual das duas afirmações bate com o código: `#[derive(..., Hash)]` presente,
   ou `Hash` manual via `Debug`?
3. Corrigir o L0 para bater com o código real — **não** mudar o código para bater com uma
   das duas frases do L0 sem primeiro confirmar qual delas era a intenção original (se
   houver histórico que explique a divergência, citar; se não, a fonte de verdade é o
   código, ADR-0107).

### `outline.md` — exemplo inválido (`indent: true` para `OutlineIndent`)

1. Ler o tipo `OutlineIndent` real — aceita `bool`, ou só os tipos que o L0 lista?
2. Se o tipo **não** aceita `bool`: o exemplo está errado — corrigir para um valor válido
   real, testado (compilar o exemplo, não só escrevê-lo).
3. Se o tipo **aceita** `bool` (e o L0 estava desactualizado ao restringir os tipos
   aceites): é o L0 que estava errado na definição do tipo, não no exemplo — corrigir a
   secção de tipos aceites, manter o exemplo.
4. Qualquer que seja o caso, confirmar com `cargo build`/teste real, não inspecção visual.

## Parte 3 — Bloco 3 (fundamentação em falta), por gravidade

Ordenar os 83 achados do Bloco 3 (P1021) por severidade (`grave` primeiro — eram, per o
sumário, uma fracção dos 134 graves totais). Para cada achado grave de Bloco 3:

1. A afirmação é verificável contra `typst.app/docs/` (usar o corpus `00_nucleo/
   corpus-docs/math/` já existente onde aplicável, per a prática estabelecida no P998) ou
   contra medição directa do vanilla?
2. Se verificável e **correcta**: adicionar a citação/medição em falta, sem mudar a
   afirmação.
3. Se verificável e **incorrecta** (mesmo padrão do Passo 996 — generalização além do que
   a fonte diz): corrigir a afirmação para o que a fonte realmente sustenta.
4. Se não verificável directamente (comportamento interno do cristalino sem equivalente
   documentado nem vanilla a medir): marcar como "decisão de implementação, não paridade"
   em vez de forçar uma citação que não existe.

Não avançar para os achados leves de Bloco 3 neste passo — ficam para decisão posterior
sobre se vale o esforço, dado o volume.

---

## Validação (após as três partes)

```
crystalline-lint .
cargo test --workspace
```
Zero regressão — são correcções de prompt e, no máximo, de um exemplo em L0; qualquer
mudança de código (caso `pagebreak.md`) tem de passar pelos testes normais.

## Resultado esperado

As duas frases sistémicas definidas uma vez, resolvendo dezenas de achados de Bloco 2 de
uma só correcção. Os dois achados tipo-bug resolvidos com evidência de qual lado (L0 ou
código) estava errado. Achados graves de Bloco 3 fechados com citação real ou reclassificados
como decisão interna. Achados leves e o resto do Bloco 4 (referências a passo, 171 achados)
ficam para passos futuros, por decisão já tomada de os tratar à parte.

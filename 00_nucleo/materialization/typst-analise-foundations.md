# Análise — typst-library/src/foundations/

## Objectivo

Identificar quais tipos de `typst-library/src/foundations/` são
domínio puro (candidatos a L1) e quais têm dependências externas
que os mantêm fora de L1. O resultado alimenta o Passo 7
(SystemWorld em L3) e os passos seguintes.

Não é uma migração — é um diagnóstico que produz um mapa.

---

## Tipos prioritários para o Passo 7

Estes são os stubs actualmente em L1 que `World` retorna.
São os mais urgentes de classificar.

```bash
# Library — world.library() retorna &Library
grep -rn "^pub struct Library\|^pub enum Library" \
  lab/typst-original/crates/typst-library/src/ | head -5

grep "^use\|^extern" \
  lab/typst-original/crates/typst-library/src/foundations/library.rs 2>/dev/null \
  | grep -v "crate::\|super::\|std::" | head -20

grep -n "^pub fn\|^pub struct\|impl Library" \
  lab/typst-original/crates/typst-library/src/foundations/library.rs 2>/dev/null \
  | head -20

# FontBook — world.book() retorna &FontBook
grep -rn "^pub struct FontBook" \
  lab/typst-original/crates/ | head -5

# Font — world.font() retorna Option<Font>
grep -rn "^pub struct Font" \
  lab/typst-original/crates/ | head -5

# Bytes — world.file() retorna FileResult<Bytes>
grep -rn "^pub struct Bytes" \
  lab/typst-original/crates/ | head -5

# Datetime — world.today() retorna Option<Datetime>
grep -rn "^pub struct Datetime\|^pub enum Datetime" \
  lab/typst-original/crates/ | head -5
```

---

## Mapa completo de foundations/

```bash
# Listar todos os ficheiros de foundations/
find lab/typst-original/crates/typst-library/src/foundations \
  -name "*.rs" | sort

# Para cada ficheiro, contar externos (excluindo std/crate/super)
for f in $(find lab/typst-original/crates/typst-library/src/foundations \
  -name "*.rs" | sort); do
  count=$(grep "^use\|^extern" "$f" \
    | grep -v "crate::\|super::\|std::" | wc -l)
  echo "$count $f"
done | sort -n
```

---

## Critério de classificação

Para cada tipo encontrado, aplicar:

| Condição | Classificação |
|----------|--------------|
| Zero externos, zero I/O, zero estado global | **L1 directo** |
| Usa apenas `ecow` como externo | **L1 com ADR-0015** (substituir ou avaliar) |
| Usa `comemo` | **L1 com ADR-0001** (autorizado) |
| Usa standard Unicode sem I/O | **L1 com ADR 0010–0013** (autorizado) |
| Usa `typst-syntax` apenas | **L1** (já em L1) |
| Tem I/O (fs, net, process) | **L3** |
| Depende de tipos não-migrados de `typst-library` | **stub em L1, migração adiada** |

---

## Reportar

Para cada tipo dos stubs prioritários:

```
Tipo: Library
Ficheiro: typst-library/src/foundations/library.rs
Externos: [lista]
Classificação: L1 / L3 / stub
Razão: [uma linha]

Tipo: FontBook
...

Tipo: Font
...

Tipo: Bytes
...

Tipo: Datetime
...
```

E para o mapa completo de `foundations/`, reportar:
- Quantos ficheiros têm zero externos (candidatos directos a L1)
- Quais têm apenas `ecow` (candidatos com ADR-0015)
- Quais têm I/O ou dependências pesadas (ficam em L3)
- Quais são os tipos que bloqueiam `Module` e `Value` reais

Esta informação determina:
1. Quais stubs podem ser substituídos antes do Passo 7
2. Quais stubs o `SystemWorld` em L3 vai receber como tipos reais vs opacos
3. A sequência de ADRs 0017+ para os tipos migráveis

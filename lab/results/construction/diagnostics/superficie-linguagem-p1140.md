# P1140 — Inventário verificável da superfície da linguagem

**Executado em:** 2026-08-23T22:21:03-03:00
**HEAD cristalino:** `0314efaea6a5cb4c377b9190c814e3b38a080d68`
**Working tree:** não commitada
**Vanilla ratificado:** `upstream/main a51e02804`

Recontagem P1140.2 após corrigir `label`; a proveniência detalhada do primeiro
inventário permanece no histórico Git deste documento.

## Resultado executivo

O catálogo combinado contém **2.180 paths públicos distintos**. O vanilla
ratificado forneceu **2.134** entradas e o cristalino **1.020** entradas após
percorrer o `std` real e sondar os paths de tipos/prelude que não são
enumeráveis pelo módulo.

| Classe | Total | Leitura correta |
|---|---:|---|
| `MATCH` | 799 | presença e kind público coincidem |
| `UNVERIFIED_METADATA` | 173 | função coincide, mas o cristalino não representa sua assinatura nativa |
| `MISSING_BINDING` | 5 | global vanilla ausente |
| `MISSING_MEMBER` | 1.155 | membro público vanilla ausente |
| `WRONG_KIND` | 2 | path existe com kind público diferente |
| `EXTRA_BINDING` | 46 | path cristalino sem correspondente no vanilla ratificado |

Os `MISSING_MEMBER` incluem **768 símbolos**, **334 funções** e **53 valores
ou módulos de outros kinds**. A magnitude não é uma prioridade automática:
uma família gerada de símbolos deve ser tratada separadamente de um global ou
construtor estrutural.

## Achados prioritários

### Globais ausentes

- `html` — módulo feature-gated no vanilla (`--features html`);
- `linebreak`, `page`, `parbreak` — funções/elementos globais;
- `path` — tipo global.

O caso `html` não autoriza simplesmente criar um binding default: a feature e
o pipeline HTML precisam ser lidos juntos no L0 antes de decidir o contrato.

### Kind público divergente

P1140.1 corrigiu `decimal`, `duration`, `regex`, `selector`, `stroke`, `tiling`
e `version`; P1140.2 corrigiu `label`. Restam dois casos: `math.equation` é
função no vanilla e `none` no cristalino; `math.sqrt` é função no vanilla e
símbolo no cristalino. Ambos são observáveis por `repr(type(path))`, portanto
não são diferenças meramente mecânicas.

### Membros funcionais ausentes

Há 334 membros de kind `function` ausentes. A amostra pública confirmou, entre
outros, `array.all`, `str.clusters`, `gradient.kind`, `datetime.day`,
`int.bit-and`, `counter.get` e `content.fields`. A fila deve ser atomizada pelos
L0 donos:

- coleções: `compiler/stdlib/collections.md` e tipos relacionados;
- texto: `compiler/stdlib/foundations/str.md` e `compiler/stdlib/text/*`;
- valores/tipos: `entities/value.md`, `entities/func.md` e L0 específico;
- estado/counter: `compiler/stdlib/state.md` e `counter.md`;
- módulos `math`, `sym`, `emoji`, `pdf`: seus L0 próprios.

### Símbolos

Faltam 768 paths de kind `symbol`: 263 sob `math`, 263 sob `sym` e 242 sob
`emoji`. O
cristalino também possui três símbolos sem path ratificado equivalente
(`math.registered`, `sym.registered`, `sym.sqrt`). Este lote pede comparação
de tabelas e aliases, não 768 implementações manuais independentes.

### Assinaturas

As 173 funções comuns ficaram como `UNVERIFIED_METADATA`. Isso é deliberado:
o vanilla expõe `Func::params()`/`ParamInfo` em Rust, enquanto o `Func`
cristalino não carrega metadados equivalentes para nativas. Presença e kind não
provam positional/named/variadic/required/settable/default. Acrescentar esses
campos mudaria contrato público L1 e exige novo L0 + gate ADR-0127; P1140 não o
faz.

## Probes observáveis

Foram executados **22 probes estratificados** com a fórmula
`repr(type(path))` nos dois binários. Resultado após P1140.2: **7 MATCH** e
**15 DIFF**.

Os controles positivos `calc.abs`, `math.sum` e `sym.arrow` coincidiram. Os
globais, kinds e membros listados acima divergiram como previsto. `html` foi
sondado no vanilla com `--features html`. O JSON bruto preserva comandos,
exit code, stdout e stderr em
`superficie-linguagem-p1140-probes.json`.

O primeiro ensaio com `type(path)` revelou que o cristalino ainda não serializa
valores `type` em `eval --format json`; a fórmula foi corrigida para
`repr(type(path))`. Isso evita classificar uma limitação do DTO JSON como
ausência do binding.

## Método e reprodução

Dois extratores independentes evitam misturar workspaces:

```sh
cd lab/typst-original
cargo run --release -p p1140-inventory -- /tmp/p1140-vanilla.json

cd ../..
cargo run --offline --release \
  --manifest-path lab/surface-inventory/Cargo.toml -- \
  /tmp/p1140-crystalline.json /tmp/p1140-vanilla.json

python3 lab/surface-inventory/merge.py \
  /tmp/p1140-vanilla.json /tmp/p1140-crystalline.json \
  00_nucleo/diagnosticos/superficie-linguagem-p1140.json \
  --generated-at 2026-08-24T00:37:00+00:00

python3 lab/surface-inventory/run_probes.py
```

O extrator vanilla percorre `Library.global`, módulos, scopes de funções e
scopes de tipos, incluindo parâmetros/defaults. O cristalino percorre o módulo
`std` construído pelo avaliador e valida paths vanilla ainda ausentes por
expressões runtime. Nenhuma lista de bindings cristalinos foi duplicada à mão.

O merge foi repetido com o mesmo timestamp e comparado por `cmp`: saída
byte-idêntica. Os 2.134 registros vanilla têm `source` resolvido para um
`file:line` existente. Os sources cristalinos `runtime:std` e
`runtime:probe:<path>` são justificativas explícitas do mecanismo de medição,
não alegações de linha estática.

## tekt-cargo-dsm

A lente está disponível em
`/home/dikluwe/Documentos/Antigravity/tekt-cargo-dsm/target/release/lente`.
Seu `--estrutura` mede módulos, dependências e ciclos Rust. Ela não enumera
bindings Typst nem `ParamInfo`; por ADR-0107, não foi usada como oráculo de
superfície da linguagem. Continua útil em passos posteriores para estimar o
impacto estrutural de uma correção já legitimada por L0.

## Proveniência da working tree

Antes do fechamento documental, `git diff HEAD --stat` registrou **27
ficheiros rastreados, 524 inserções e 398 remoções**, além dos ficheiros não
rastreados P1139/P1140. A diferença adicional rastreada em relação ao fim de
P1139 é `lab/typst-original/Cargo.lock`, atualizada pelo utilitário vanilla.
Nenhum ficheiro em `01_core/`, `02_shell/`, `03_infra/` ou `04_wiring/` foi
alterado por P1140; as modificações já presentes nessas camadas pertencem a
P1139.

## Próximos passos derivados

Sem interferir na ordem P1141–P1145, a superfície pede passos próprios:

1. reconciliar os cinco globais, começando por medir feature/pipeline de
   `html` separadamente dos quatro não feature-gated;
2. corrigir os dez kinds públicos, por entidade/L0 dono;
3. decompor os 334 membros funcionais por módulo;
4. comparar tabelas `math`/`sym`/`emoji` e aliases;
5. decidir, em gate próprio, se metadados de assinatura nativa pertencem ao
   contrato L1 ou se probes por callable bastam para a paridade da linguagem.

Nenhuma dessas correções está autorizada por este relatório.

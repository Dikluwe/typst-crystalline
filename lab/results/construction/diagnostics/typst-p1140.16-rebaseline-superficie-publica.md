# Diagnóstico P1140.16 — rebaseline da superfície pública

**Data:** 2026-08-24  
**Passo:** `00_nucleo/materialization/typst-passo-1140.16.md`  
**Estado:** concluído

## 1. Proveniência

Medição final em:

- HEAD cristalino: `45b547073d7686cdd5d3e3030c82de3e22ec395f`;
- working tree não commitado;
- hora final: `2026-08-24T13:18:16-03:00`;
- `git diff HEAD --stat`: `75 files changed, 567 insertions(+), 488 deletions(-)`;
- vanilla ratificado: upstream/main `a51e02804`;
- SHA-256 dos dois binários vanilla:
  `7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8`;
- `/usr/local/bin/typst` e
  `lab/typst-original/target/release/typst` são byte-idênticos;
- ambos responderam `"function"` à sentinela
  `repr(type(calc.abs))`;
- o cristalino foi reconstruído da working tree e identificou
  `typst 0.15.1 (45b54707)`; a sentinela também respondeu `"function"`.

Arquivos atribuíveis a P1140.16 no estado final da medição:

- `lab/surface-inventory/probes.json`;
- `lab/surface-inventory/run_probes.py`;
- `00_nucleo/diagnosticos/superficie-linguagem-p1140.16.json`;
- `00_nucleo/diagnosticos/superficie-linguagem-p1140.16-probes.json`;
- este relatório e o passo P1140.16.

As alterações já existentes em L0–L4 pertencem a passos anteriores. P1140.16
não escreveu nessas camadas.

## 2. Comandos principais

```sh
cd lab/typst-original
cargo run --release -p p1140-inventory -- /tmp/p1140.16-vanilla.json

cd ../..
cargo run --offline --release \
  --manifest-path lab/surface-inventory/Cargo.toml -- \
  /tmp/p1140.16-crystalline.json /tmp/p1140.16-vanilla.json

python3 lab/surface-inventory/merge.py \
  /tmp/p1140.16-vanilla.json \
  /tmp/p1140.16-crystalline.json \
  00_nucleo/diagnosticos/superficie-linguagem-p1140.16.json \
  --generated-at 2026-08-24T16:15:13+00:00

python3 lab/surface-inventory/run_probes.py \
  00_nucleo/diagnosticos/superficie-linguagem-p1140.16-probes.json
```

Merge e probes foram repetidos para `/tmp` e comparados por `cmp`.

## 3. Inventários

- vanilla: 2.134 paths;
- cristalino: 1.020 paths;
- união classificada: 2.179 paths.

| Classe | P1140 histórico | P1140.16 | Delta |
|---|---:|---:|---:|
| `MATCH` | 799 | 799 | 0 |
| `UNVERIFIED_METADATA` | 173 | 176 | +3 |
| `MISSING_BINDING` | 5 | 4 | −1 |
| `MISSING_MEMBER` | 1.155 | 1.155 | 0 |
| `WRONG_KIND` | 2 | 0 | −2 |
| `EXTRA_BINDING` | 46 | 45 | −1 |
| **Total** | **2.180** | **2.179** | **−1** |

O total caiu porque `sym.sqrt`, extra sem correspondente ratificado, foi
removido enquanto `math.sqrt` passou a ser função no path correto.

## 4. Paths cuja classificação mudou

| Path | Antes | Agora | Leitura |
|---|---|---|---|
| `linebreak` | `MISSING_BINDING` | `UNVERIFIED_METADATA` | função presente; assinatura não representada pelo inventário cristalino |
| `math.equation` | `WRONG_KIND` | `UNVERIFIED_METADATA` | agora é função nos dois lados |
| `math.sqrt` | `WRONG_KIND` | `UNVERIFIED_METADATA` | agora é função nos dois lados |
| `sym.sqrt` | `EXTRA_BINDING` | ausente da união | alias extra removido |

Os tipos corrigidos anteriormente — `decimal`, `duration`, `regex`,
`version`, `label`, `selector`, `stroke` e `tiling` — coincidiram nos probes.

## 5. Bindings globais remanescentes

| Path | Vanilla | Cristalino | Classificação |
|---|---|---|---|
| `html` | módulo com feature `html` | ausente | fila feature-gated separada |
| `page` | função com 19 parâmetros | ausente | contrato amplo; não atomizar junto |
| `parbreak` | função sem parâmetros | ausente | menor unidade reproduzível |
| `path` | tipo | ausente | requer entidade/tipo público próprio |

`linebreak` já não pertence a esta lista.

## 6. Membros remanescentes

Os 1.155 `MISSING_MEMBER` dividem-se em:

- 768 símbolos;
- 334 funções;
- 18 cores;
- 15 arrays;
- 8 alinhamentos;
- 5 módulos;
- 4 direções;
- 2 floats;
- 1 tipo.

Isto continua sendo uma medida de tamanho, não uma prioridade automática.
Símbolos devem ser tratados por tabelas e aliases; funções, por L0 dono.

## 7. Probes observáveis

Foram executados 28 probes: 15 coincidências semânticas e 13 divergências.

Coincidiram:

- `linebreak`, `decimal`, `duration`, `regex`, `version`, `label`,
  `selector`, `stroke`, `tiling`, `math.equation`, `math.sqrt`;
- controles `calc.abs`, `math.sum` e `sym.arrow`;
- `pdf.data-cell`, ausente nos dois lados.

Divergiram:

- `html`, `page`, `parbreak`, `path`;
- `array.all`, `str.clusters`, `color.black`, `gradient.kind`,
  `datetime.day`, `int.bit-and`, `counter.get`, `content.fields`;
- `emoji.heart`.

O probe histórico `pdf.data-cell` tinha premissa obsoleta: o vanilla também o
rejeita. O instrumento foi corrigido em `lab/` para considerar semanticamente
iguais dois resultados de ausência, preservando exit code/stdout/stderr brutos.

## 8. Reprodutibilidade

- dois merges com as mesmas entradas e timestamp: byte-idênticos;
- duas execuções dos 28 probes: JSON byte-idêntico;
- contagens repetidas: idênticas;
- `crystalline-lint .`: exit 0.

## 9. Fila atomizada

1. **P1140.17 — `parbreak()` global.** O cristalino já possui
   `Content::Parbreak`, avaliação sintática, repr e layout. Falta a superfície
   chamável sem argumentos. Auditar `entities/content.md`, `compiler/eval.md`
   e o L0 stdlib dono; adicionar binding é contrato público, logo exige L0
   primeiro e gate ADR-0127.
2. **`page`.** A função ratificada tem 19 parâmetros. O código contém uma
   decisão histórica de removê-la em favor de `#set page`; isso contradiz a
   superfície vanilla atual e precisa de passo próprio de auditoria/L0.
3. **`path`.** Tipo público inexistente; requer contrato e entidade próprios.
4. **Membros funcionais.** Atomizar por coleções, texto, datetime, inteiros,
   counter e content.
5. **Símbolos/cores/aliases.** Comparar tabelas mecanicamente, sem criar 768
   implementações independentes.
6. **`html`.** Manter numa frente feature-gated separada; ausência default não
   autoriza habilitação implícita.
7. **Metadados nativos.** Decisão de contrato separada; não contam como
   divergência funcional provada enquanto os probes de chamada não falharem.

## 10. Recomendação

O próximo passo é **P1140.17 — restaurar a função global `parbreak()`**.

É a menor divergência pública não feature-gated confirmada: zero parâmetros,
representação de conteúdo já existente e teste RED direto por
`repr(type(parbreak))` e chamada `parbreak()`. O passo deve primeiro medir o
contrato vanilla, atualizar o L0 e parar no gate ADR-0127 antes de adicionar o
binding.

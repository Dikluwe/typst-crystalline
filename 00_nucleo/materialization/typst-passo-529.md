---
# P529 — Re-teste de instanciação VF: subset-first + skia-pathops

> **Passo:** 529
> **Data:** 2026-07-01
> **Foco:** Testar duas hipóteses levantadas na revisão de P528: (1) a ordem "instanciar primeiro, subsetar depois" é ineficiente porque opera sobre todos os glifos da fonte; a ordem "subsetar primeiro (com oxifont-subset, rápido e já validado), depois instanciar a fonte pequena" pode reduzir drasticamente o tempo. (2) o backend `skia-pathops` pode não estar instalado no venv, fazendo o `fontTools` cair no fallback puro-Python `booleanOperations` (conhecido por ser catastroficamente lento). Verificar instalação, instalar se ausente, e repetir medições. Zero código de produção — apenas diagnóstico empírico com medição de tempos.
> **Tipo:** Diagnóstico
> **Tamanho:** S (~20 min)
> **ADR-0108 EM VIGOR** — medir antes de decidir; medir após cada hipótese.
> **ADR-0114 EM VIGOR** — sonda A.0 antes de spec; aqui a sonda é sobre a viabilidade de uma abordagem já testada mas com método incorrecto.
> **Dependências:** P528 (diagnóstico de instanciação VF que concluiu "inviável" por ordem errada e backend possivelmente ausente)
---

## Contexto

O P528 concluiu que a instanciação estática de VF via `fontTools.varLib.instancer` é **inviável** (~4 minutos por instância). No entanto, a revisão identificou dois problemas metodológicos na experiência:

1. **Ordem errada:** o teste instanciou a **fonte completa** (Ubuntu Sans com cobertura multi-script, milhares de glifos) e só depois subsetaria. O custo de interpolação `gvar` escala com o número de glifos. A ordem sensata é: **subsetar primeiro** (com `oxifont-subset`, já validado em P516/P522 como rápido — reduz para ~50-100 glifos usados no documento), **depois instanciar** a fonte pequena.

2. **Backend de pathops:** a variação com `overlap=False` praticamente não alterou o tempo (240,6s → 235,3s), sugerindo que o `fontTools` não está a usar `skia-pathops` (backend C++ compilado para operações booleanas de contorno) e sim o fallback puro-Python `booleanOperations`, conhecido por ser ordens de magnitude mais lento. Verificar e instalar se necessário.

Este passo testa ambas as hipóteses. Se qualquer uma (ou ambas) reduzir o tempo para < 5 segundos por instância, a Abordagem A de P528 torna-se viável e o fix de VF pode ser implementado. Se ambas falharem, a conclusão de P528 mantém-se e a pesquisa de alternativas (Fontations/skrifa) continua.

---

## Sub-tarefa 1 — Verificar e instalar skia-pathops

### 1.1 Verificar instalação

```bash
lab/.venv/bin/python3 -c "import pathops; print('skia-pathops instalado:', pathops.__version__)"
```

### 1.2 Se ausente, instalar

```bash
lab/.venv/bin/pip install skia-pathops
```

### 1.3 Verificar após instalação

```bash
lab/.venv/bin/python3 -c "import pathops; print('skia-pathops:', pathops.__version__)"
```

### 1.4 Critério de fecho

- [ ] `skia-pathops` confirmado instalado (ou instalado nesta sub-tarefa).
- [ ] Versão registada.

---

## Sub-tarefa 2 — Re-teste com skia-pathops: instanciar fonte completa

Repetir exactamente o teste de P528, mas agora com `skia-pathops` instalado. Comparar com o baseline de P528 (~240s).

### 2.1 Script de teste

```bash
time lab/.venv/bin/python3 - <<'PY'
from fontTools.ttLib import TTFont
from fontTools.varLib.instancer import instantiateVariableFont
import io

path = '/usr/share/fonts/truetype/ubuntu/UbuntuSans[wdth,wght].ttf'
f = TTFont(path)
inst = instantiateVariableFont(f, {'wght': 700, 'wdth': 100}, overlap=False)
buf = io.BytesIO()
inst.save(buf)
print('size:', len(buf.getvalue()))
PY
```

### 2.2 Variantes a testar

| Teste | overlap | Notas |
|-------|---------|-------|
| A | `overlap=False` | Igual ao P528, mas com skia-pathops |
| B | `overlap=True` (default) | Sem remoção de overlaps — mais rápido? |
| C | `updateFontNames=True` | Renomeia a fonte para "Ubuntu Sans Bold" etc. |

### 2.3 Critério de fecho

- [ ] Tempo de cada variação registado.
- [ ] Comparação com baseline P528 (~240s) documentada.
- [ ] Se tempo < 30s: **viável**; anotar como candidato a implementação.
- [ ] Se tempo > 30s: continuar para Sub-tarefa 3 (subset-first).

---

## Sub-tarefa 3 — Teste subset-first: oxifont-subset → fontTools instancer

### 3.1 Estratégia

1. **Passo 1 — Subset com oxifont-subset:** reduzir a fonte VF completa para apenas os glifos usados no documento (ex.: "Hello world." + notdef + alguns glifos de ligatures/kerning). Esperado: ~10-50 glifos, tempo < 1s (validado em P516).
2. **Passo 2 — Instanciar a fonte subsetada:** aplicar `instantiateVariableFont` na fonte pequena. O número de glifos é ordens de magnitude menor, pelo que o tempo deve ser proporcionalmente menor.

### 3.2 Script de teste

```bash
time lab/.venv/bin/python3 - <<'PY'
import subprocess
from fontTools.ttLib import TTFont
from fontTools.varLib.instancer import instantiateVariableFont
import io

# Passo 1: Subset com oxifont-subset (via Rust binary ou Python API se disponível)
# Se oxifont-subset não tem API Python, usar fontTools.subset como proxy

path = '/usr/share/fonts/truetype/ubuntu/UbuntuSans[wdth,wght].ttf'

# Simular subset: carregar só os glifos necessários
# Para "Hello world." precisamos: H, e, l, o, ' ', w, r, d, ., notdef
# Mas o subsetter real também inclui glifos de kerning/ligatures se necessário
needed_gids = [0]  # notdef
# Mapear codepoints para GIDs usando cmap
f = TTFont(path)
cmap = f['cmap'].getBestCmap()
for cp in [ord(c) for c in "Hello world."]:
    gid = cmap.get(cp)
    if gid is not None:
        needed_gids.append(gid)
needed_gids = list(set(needed_gids))
print(f"Glifos necessários: {len(needed_gids)}")

# Subset com fontTools.subset (proxy para oxifont-subset)
from fontTools.subset import Subsetter, save_font
subsetter = Subsetter()
subsetter.populate(gids=needed_gids)
subsetter.subset(f)
buf_subset = io.BytesIO()
save_font(f, buf_subset, options=None)
subset_data = buf_subset.getvalue()
print(f"Tamanho subset: {len(subset_data)} bytes")

# Passo 2: Instanciar a fonte subsetada
f_subset = TTFont(io.BytesIO(subset_data))
inst = instantiateVariableFont(f_subset, {'wght': 700, 'wdth': 100}, overlap=False)
buf_inst = io.BytesIO()
inst.save(buf_inst)
print(f"Tamanho instanciada: {len(buf_inst.getvalue())} bytes")
PY
```

**Nota:** se `fontTools.subset` não for um proxy fiável para `oxifont-subset` (preservação de `fvar`/`gvar`/`avar`), ajustar o script. O `oxifont-subset` já foi validado em P522 para preservar tabelas VF; se não houver API Python para `oxifont-subset`, usar o binary Rust via subprocess:

```bash
# Gerar subset com oxifont-subset CLI (se existir)
# ou usar fontTools.subset como aproximação para este teste
```

### 3.3 Medições a registar

| Métrica | Valor |
|---------|-------|
| Número de glifos no subset | |
| Tempo subset (Passo 1) | |
| Tempo instanciação (Passo 2) | |
| Tempo total (subset + instanciar) | |
| Tamanho fonte subsetada | |
| Tamanho fonte instanciada | |
| Tamanho fonte original | |

### 3.4 Critério de fecho

- [ ] Tempo total (subset + instanciar) < 5 segundos: **viável**; anotar como implementação candidata.
- [ ] Tempo total 5-30 segundos: marginal; considerar caching de instâncias.
- [ ] Tempo total > 30 segundos: inviável mesmo com subset-first; manter conclusão de P528.

---

## Sub-tarefa 4 — Decisão e documentação

### 4.1 Tabela de resultados

| Hipótese | Tempo | Conclusão |
|----------|-------|-----------|
| P528 baseline (sem pathops, fonte completa) | ~240s | Inviável |
| Com skia-pathops, fonte completa | _a preencher_ | _a preencher_ |
| Subset-first + skia-pathops | _a preencher_ | _a preencher_ |

### 4.2 Decisão

- **Se subset-first + skia-pathops < 5s:**
  - P530: Implementar fix de VF (instanciação estática por peso/estilo, subset-first, embutir múltiplas instâncias no PDF). Tamanho M.
  - Trilha 7: fecha com P530.

- **Se skia-pathops ajuda mas subset-first ainda > 30s:**
  - A instanciação é intrinsecamente lenta para fontes VF com muitos eixos/glifos, mesmo reduzidos.
  - P530: Pesquisar alternativas (Fontations/skrifa) ou aceitar faux-bold/faux-italic como fallback documentado.
  - Trilha 7: permanece parcial; scope-out documentado.

- **Se skia-pathops não ajuda significativamente (< 2× speedup) e subset-first não é testável:**
  - Problema não é o backend; a instanciação é intrinsecamente lenta.
  - Mesma conclusão que acima.

### 4.3 Critério de fecho

- [ ] Tabela de resultados preenchida.
- [ ] Decisão documentada em `00_nucleo/diagnosticos/instanciacao-vf-p529.md`.
- [ ] Handoff actualizado com estado de VF (viável ou permanece parcial).
- [ ] Próximo passo definido (P530 implementação ou P530 pesquisa alternativas).

---

## Critério de fecho do passo

- [ ] Sub-tarefa 1: `skia-pathops` verificado/instalado.
- [ ] Sub-tarefa 2: re-teste com fonte completa + skia-pathops; tempo registado.
- [ ] Sub-tarefa 3: teste subset-first; tempo registado.
- [ ] Sub-tarefa 4: decisão documentada; handoff actualizado; próximo passo definido.
- [ ] Commit atómico: `P529: re-teste instanciação VF — subset-first + skia-pathops`.

---

## Próximo passo (depende dos resultados)

| Cenário | Passo | Tamanho | Descrição |
|---------|-------|---------|-----------|
| Subset-first + skia-pathops < 5s | **P530** | M | Implementar fix de VF: instanciar estaticamente por peso/estilo, subsetar primeiro, embutir múltiplas instâncias no PDF |
| Ainda inviável | **P530** | S | Documentar scope-out de VF instanciação; pesquisar Fontations/skrifa como futuro |
| Inovação independente de VF | **P531** | L | Lookahead Layout Engine (opção A do handoff) |

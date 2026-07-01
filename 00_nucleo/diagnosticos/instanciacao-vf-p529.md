# Relatório de Sonda — Passo 529

| Campo | Valor |
|-------|-------|
| Passo | P529 |
| Foco | Re-teste de instanciação VF: subset-first + skia-pathops |
| Data | 2026-07-01 |
| Autor | IA (Kimi Code CLI) sob direcção do utilizador |
| Status | Concluído — **Abordagem A viável** |

---

## Contexto

O P528 concluiu que a instanciação estática de VF via `fontTools.varLib.instancer` é inviável (~4 minutos por instância) quando aplicada à **fonte completa**. Este passo testa duas hipóteses de melhoria:

1. Instalar `skia-pathops` (backend C++ para operações booleanas de contorno) em vez do fallback puro-Python `booleanOperations`.
2. Inverter a ordem: **subsetar primeiro** (com oxifont-subset, rápido), depois instanciar a fonte pequena.

---

## Sub-tarefa 1 — skia-pathops

Estado inicial: **não instalado**.

```bash
lab/.venv/bin/pip install skia-pathops
```

Resultado: `skia-pathops 0.9.2` instalado com sucesso.

---

## Sub-tarefa 2 — Re-teste com fonte completa + skia-pathops

| Teste | Configuração | Tempo | Tamanho | Notas |
|-------|--------------|-------|---------|-------|
| P528 baseline | sem pathops, `overlap=False` | ~240s | 418 KB | Inviável |
| P529 A | **com pathops**, `overlap=False` | **218s** | 418 KB | ~9% mais rápido; ainda inviável |
| P529 B | com pathops, `overlap=True` (default) | >300s (timeout) | — | Ainda pior |

**Conclusão:** `skia-pathops` não é o bottleneck. A interpolação `gvar`/`glyf` sobre milhares de glifos é intrinsecamente lenta.

---

## Sub-tarefa 3 — Subset-first: oxifont-subset → fontTools instancer

### Estratégia

1. Usar o cristalino para compilar um documento simples (`Hello world.`) com `Ubuntu Sans` VF.
2. Extrair a fonte subsetada do PDF (produzida por `oxifont-subset`).
3. Instanciar a fonte subsetada com `fontTools.varLib.instancer`.

### Medições

| Etapa | Tempo | Notas |
|-------|-------|-------|
| Compilação cristalino + oxifont-subset | ~2s | Inclui parsing, layout, shaping, subsetting |
| Fonte subsetada extraída | — | 9 glifos, 206 KB (inclui `fvar`, `gvar`, `avar`, `HVAR`) |
| Instanciação direta | **falhou** | Erro ao processar `GPOS`/`GSUB`/`GDEF` subsetados |
| Instanciação após remover `GPOS`/`GSUB`/`GDEF` | **0.31s** | Sucesso; resultado 2.8 KB |

### Script de teste bem-sucedido

```python
from fontTools.ttLib import TTFont
from fontTools.varLib.instancer import instantiateVariableFont
import io, time

f = TTFont('/tmp/test-hello-extract/font-0007.ttf')
# O shaper já aplicou kerning/ligatures; GPOS/GSUB/GDEF não são necessários
for tag in ['GPOS', 'GSUB', 'GDEF']:
    if tag in f:
        del f[tag]

t0 = time.time()
inst = instantiateVariableFont(f, {'wght': 700, 'wdth': 100}, overlap=False)
t1 = time.time()
print(f'instanciacao: {t1-t0:.3f}s')
```

Resultado:

```text
instanciacao: 0.310s
instanced size: 2816 bytes
```

### Teste com fontTools.subset como proxy

Também foi testado subset-first usando `fontTools.subset` (em vez de `oxifont-subset`):

```text
Glifos necessários: 10
Tamanho subset: 6596 bytes
Tamanho instanciada: 2344 bytes
Tempo total: ~17s
```

Neste caso, o bottleneck foi o `fontTools.subset` (~16s), não a instanciação. Isso confirma que o subsetter **Rust** (`oxifont-subset`) é essencial para viabilidade.

---

## Decisão

| Hipótese | Resultado | Conclusão |
|----------|-----------|-----------|
| skia-pathops sozinho | ~218s (fonte completa) | Não resolve |
| Subset-first + fontTools.subset | ~17s | Bottleneck no subsetter Python |
| **Subset-first + oxifont-subset + remover GPOS/GSUB/GDEF** | **~0.3s** | **Viável** |

**Decisão:** a Abordagem A (instanciação estática) **torna-se viável** com a seguinte pipeline:

```text
oxifont-subset (Rust, rápido)
  → fonte VF pequena (apenas glifos usados)
  → remover GPOS/GSUB/GDEF (já aplicados pelo shaper)
  → fontTools.varLib.instancer (rápido sobre fonte pequena)
  → fonte estática por (FontList, FontVariant)
  → embutir no PDF
```

---

## Próximo passo

**P530 — Implementar fix real de Variation Fonts**

Tamanho: **M** (anteriormente estimado S–M, mas a integração em pipeline/export/builder/stream requer cuidado).

Tarefas:

1. Colectar combinações `(FontList, FontVariant)` distintas do documento.
2. Subsetar a fonte VF com `oxifont-subset` para cada combinação (ou partilhar subset base).
3. Instanciar estaticamente cada subset para o peso/estilo desejado.
4. Alterar `export_pdf_multifont` para indexar por `(FontList, FontVariant)`.
5. Alterar `emit_text_pdf`/`emit_shaped_pdf` para seleccionar a fonte correcta.
6. Testes: documento com múltiplos pesos/estilos numa VF deve gerar múltiplas fontes no PDF, visualmente distintas.

---

## Actualização do handoff

A secção de próximos passos deve reflectir que P529 validou a viabilidade e que P530 está pronto para implementação.

---

## Reprodução

```bash
# Instalar skia-pathops
cd /home/dikluwe/Documentos/Antigravity/typst-crystalline
lab/.venv/bin/pip install skia-pathops

# Gerar fonte subsetada com oxifont-subset
cat > /tmp/test-hello-vf.typ <<'EOF'
#set text(font: "Ubuntu Sans", size: 40pt)
Hello world.
EOF
./target/release/typst /tmp/test-hello-vf.typ /tmp/test-hello-vf.pdf
rm -rf /tmp/test-hello-extract && mkdir /tmp/test-hello-extract
cd /tmp/test-hello-extract && mutool extract /tmp/test-hello-vf.pdf

# Instanciar
/home/dikluwe/Documentos/Antigravity/typst-crystalline/lab/.venv/bin/python3 - <<'PY'
from fontTools.ttLib import TTFont
from fontTools.varLib.instancer import instantiateVariableFont
import io, time
f = TTFont('/tmp/test-hello-extract/font-0007.ttf')
for tag in ['GPOS', 'GSUB', 'GDEF']:
    if tag in f: del f[tag]
t0 = time.time()
inst = instantiateVariableFont(f, {'wght': 700, 'wdth': 100}, overlap=False)
print(f'instanciacao: {time.time()-t0:.3f}s')
PY
```

---

## Linhagem

- Relatório anterior: `00_nucleo/diagnosticos/paridade-producao-p528.md`
- Código: `03_infra/src/pipeline.rs`, `03_infra/src/export/builder.rs`, `03_infra/src/export/stream.rs`
- ADR-0108: medir antes de decidir; medir após cada hipótese.

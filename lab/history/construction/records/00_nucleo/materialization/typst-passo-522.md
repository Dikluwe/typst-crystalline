---
# P522 — Trilha 6: Sonda de CFF subsetting (diagnóstico baseline)

> **Passo:** 522
> **Data:** 2026-07-01
> **Foco:** Diagnóstico empírico do estado actual do CFF subsetting no cristalino. Medir: (1) quantas fontes CFF existem no corpus de paridade e no sistema; (2) qual o comportamento actual quando uma fonte CFF é usada (fallback, panic, corrupção silenciosa); (3) se o `oxifont-subset` (P516) suporta CFF ou se precisamos de outra estratégia; (4) impacto em documentos reais. Zero código de produção neste passo — apenas medição e decisão.
> **Tipo:** Diagnóstico
> **Tamanho:** M (~40 min)
> **ADR-0108 EM VIGOR** — medir antes de decidir. **A alegação do handoff ("CFF: fallback activo") é tratada como não verificada** — P519 já mostrou uma vez que uma alegação de scope-out sem sonda estava errada (GPOS/GSUB "removidas" quando na verdade estavam preservadas). Este passo confirma ou refuta, não assume.
> **ADR-0114 EM VIGOR** — sonda A.0 antes de spec.
> **Dependências:** P516, P517, P519, P520, P521.
> **Trilha:** 6 — CFF subsetting (XL potencial).

---

## Contexto

A Trilha 5 (P515–P521) fechou o subsetting TrueType, kerning e ToUnicode com sucesso. O handoff marca CFF como scope-out com a nota "Fallback ativo, XL-size" — mas essa nota nunca foi verificada empiricamente. Este passo é a sonda A.0 da Trilha 6.

---

## Grupo 1 — Inventário de fontes CFF no sistema

```bash
fc-list : file format 2>/dev/null | grep -i "cff\|opentype"
```

Inventário detalhado via fontTools (nota: sintaxe corrigida — `split('\n')` numa linha, não uma quebra de linha literal dentro da string):

```bash
python3 - <<'PY'
import subprocess, os
from fontTools.ttLib import TTFont

result = subprocess.run(['fc-list', ':', 'file'], capture_output=True, text=True)
fonts = [line.strip().rstrip(':') for line in result.stdout.split('\n') if line.strip()]

cff_count = 0
ttf_count = 0
other_count = 0
cff_fonts = []

for path in fonts[:500]:
    if not os.path.exists(path):
        continue
    try:
        font = TTFont(path)
        if 'CFF ' in font or 'CFF2' in font:
            cff_count += 1
            cff_fonts.append(path)
        elif 'glyf' in font:
            ttf_count += 1
        else:
            other_count += 1
    except Exception:
        other_count += 1

print(f"CFF: {cff_count}, TrueType: {ttf_count}, Other/Error: {other_count}")
if cff_fonts:
    print("Exemplos CFF:")
    for f in cff_fonts[:10]:
        print(f"  {f}")
PY
```

### Critério de fecho

- [ ] Contagem CFF vs TrueType no sistema.
- [ ] Lista das 5–10 fontes CFF mais comuns (se houver).

---

## Grupo 2 — Comportamento actual do cristalino com CFF

### 2.0 — Garantir uma fonte CFF de teste, independentemente do sistema

**O Grupo 1 pode devolver zero fontes CFF** — é o cenário mais provável em ambientes Linux com Noto/DejaVu/Liberation como defaults (todas TrueType). Sem uma fonte CFF garantida, este grupo — o mais importante, porque decide entre panic, fallback limpo, ou corrupção silenciosa — fica sem dados.

**Não depender do sistema.** Fixar uma fonte CFF de referência conhecida e obtê-la explicitamente antes de prosseguir:

```bash
# Verificar primeiro se já existe localmente (ex.: bundled em lab/ ou fixtures)
find / -iname "*.otf" 2>/dev/null | head -20
find lab/ 03_infra/fixtures/ -iname "*.otf" 2>/dev/null

# Se nenhuma disponível, obter uma fonte CFF open-source conhecida
# (ex.: Source Serif 4, licença OFL, tem variante CFF/.otf estática)
# Confirmar antes de descarregar se há acesso de rede disponível no ambiente.
```

Se não houver acesso de rede nem fonte CFF local disponível: **registar explicitamente esta limitação** na tabela final ("Grupo 2 não testável neste ambiente — sem fonte CFF disponível") em vez de pular o grupo silenciosamente. Isto é uma classificação válida por si só, distinta de "CFF é raro" — significa que a decisão de prosseguimento fica sem uma das suas entradas mais importantes, e deve ser assinalado como tal, não escondido.

### 2.1 — Documento de teste

```bash
cat > /tmp/test-cff.typ << 'EOF'
#set text(font: "NOME_DA_FONTE_CFF_CONFIRMADA", size: 12pt)
Hello world.
EOF
./target/release/typst /tmp/test-cff.typ /tmp/cff-cristalino.pdf
echo "Exit code: $?"
```

**Nota:** capturar o exit code explicitamente. Um panic pode não ser óbvio no output se o processo terminar sem mensagem clara — o exit code distingue sucesso (0) de crash (>0, tipicamente 101 para panic em Rust).

### 2.2 — Verificar o PDF gerado

```bash
pdffonts /tmp/cff-cristalino.pdf
ls -la /tmp/cff-cristalino.pdf
mutool extract /tmp/cff-cristalino.pdf
file font-*

# Verificar se a fonte extraída é um CFF/OTF válido, ou se está corrompida
python3 - <<'PY'
from fontTools.ttLib import TTFont
try:
    f = TTFont('font-0001.otf')  # ajustar nome conforme output do mutool
    print("Fonte válida. Tabelas:", sorted(f.keys()))
except Exception as e:
    print("FONTE CORROMPIDA ou ilegível:", e)
PY
```

### 2.3 — Tabela de comportamentos possíveis (ampliada)

| Comportamento | Classificação | Notas |
|---------------|---------------|-------|
| Fonte CFF completa embebida, íntegra | ✅ Funciona sem subsetting | Confirma a alegação do handoff. PDF maior, mas correcto. |
| Fonte não embebida (substituída por fallback TrueType) | ⚠️ Regressão silenciosa de aparência | Texto renderiza com fonte diferente do pedido, sem aviso. |
| Panic / erro no processo | ❌ Bug prioritário | Exit code ≠ 0. Deve ser zero, por convenção do projecto. |
| `.notdef` para todos os glifos | ❌ Bug prioritário | Fonte embebida mas inutilizável. |
| **Subsetter trata CFF como TrueType (tabelas `glyf`/`loca` aplicadas a fonte CFF) — fonte corrompida embebida** | ❌ **Bug prioritário, mais grave que os anteriores** | Pode parecer funcionar nalguns leitores de PDF e falhar silenciosamente noutros. Não é panic nem fallback limpo — é o pior caso: dados corrompidos que passam despercebidos sem inspecção manual da fonte extraída (por isso o passo 2.2 verifica a fonte extraída, não só se o PDF abre). |

Este último caso é a razão pela qual o Grupo 2.2 extrai e valida a fonte embebida com `fontTools`, e não se limita a confirmar que o PDF "abre" — um PDF com fonte corrompida frequentemente abre sem erro visível em leitores tolerantes.

### 2.4 — Comparação com vanilla

```bash
./lab/typst-original/target/release/typst compile /tmp/test-cff.typ /tmp/cff-vanilla.pdf
pdffonts /tmp/cff-vanilla.pdf
ls -la /tmp/cff-vanilla.pdf
```

### Critério de fecho

- [ ] Fonte CFF de teste garantida (local, bundled, ou descarregada) — ou limitação registada explicitamente se nenhuma via disponível.
- [ ] Exit code do processo de compilação registado.
- [ ] Fonte extraída do PDF validada estruturalmente com `fontTools` (não só "o PDF abre").
- [ ] Comportamento classificado na tabela ampliada (5 categorias, não 4).
- [ ] Comparação com vanilla (tamanho, fontes embebidas).

---

## Grupo 3 — Capacidade do oxifont-subset com CFF

```bash
grep -rn "CFF\|cff\|postscript" 03_infra/src/export/subset.rs
grep -rn "CFF\|cff\|postscript" ~/.cargo/registry/src/*/oxifont-subset-*/src/ 2>/dev/null | head -20
```

Verificar também se o código actual **detecta** o formato antes de subsetar, ou se assume TrueType incondicionalmente — isto é o que decide se o Grupo 2.3 caso "fonte corrompida" é sequer possível:

```bash
grep -n "glyf\|loca\|CFF\|sfntVersion\|OTTO" 03_infra/src/export/subset.rs
```

Se não houver nenhuma verificação de `sfntVersion`/assinatura de formato antes de aplicar a lógica de subsetting, isso confirma o risco do Grupo 2.3 independentemente do resultado empírico obtido — o código aceita cegamente qualquer fonte como TrueType.

### Alternativas de crate, se `oxifont-subset` não suportar CFF

```bash
grep -rn "CFF\|cff" ~/.cargo/registry/src/*/subsetter-*/ 2>/dev/null | head -10
```

| Opção | Descrição | Complexidade |
|-------|-----------|--------------|
| A | oxifont-subset já suporta CFF | XS — apenas activar |
| B | Usar `subsetter` crate, se suportar CFF | S/M |
| C | `fontTools` via subprocess | M — quebra pureza Rust |
| D | Implementar CFF subsetting manual | XL |
| E | Scope-out permanente (embeber CFF completo, com verificação de formato para evitar o caso 2.3) | S — sem subsetting, mas **com** detecção de formato para nunca corromper |

**Nota sobre a Opção E:** mesmo que a decisão final seja scope-out, a detecção de formato (`sfntVersion`) para desviar fontes CFF do caminho de subsetting TrueType (embebendo-as completas em vez de as corromper) é obrigatória, não opcional — é a diferença entre "scope-out documentado" e "bug silencioso". Se o Grupo 2 confirmar o caso 2.3, esta detecção é um fix XS independente da decisão sobre a Trilha 6 completa.

### Critério de fecho

- [ ] `oxifont-subset`: CFF suportado? Sim/Não + evidência.
- [ ] Confirmado se o código actual detecta formato de fonte antes de subsetar.
- [ ] Alternativas avaliadas.

---

## Grupo 4 — Impacto no corpus de paridade

```bash
grep -rn "text(font:" lab/parity/corpus/ 2>/dev/null | grep -v "Noto\|DejaVu\|Linux\|Liberation"
grep -rn "CFF\|\.otf\|Source Serif\|Minion\|Myriad" 01_core/src/ 03_infra/src/ --include="*.rs"
```

### Critério de fecho

- [ ] Número de documentos do corpus que usam fontes CFF (esperado: 0).
- [ ] Se > 0: listar e avaliar impacto.

---

## Grupo 5 — Complexidade técnica de CFF subsetting (estimativa)

| Componente | Esforço estimado | Notas |
|------------|-------------------|-------|
| Parse CFF | 0–S | `ttf-parser` já parseia CFF. |
| Subset charstrings sem optimizar subrs | M | PDF maior, mas funcional. |
| Subset + reindexar subrs | L | Necessário para PDFs compactos. |
| Suporte CID (FDSelect/FDArray) | M | Fontes CFF asiáticas comuns. |
| CFF2 (variable fonts) | XL | Fora de escopo (Trilha 7). |
| **Detecção de formato para evitar corrupção (Grupo 3, Opção E)** | **XS** | Independente da decisão sobre subsetting completo — fazer sempre. |

---

## Tabela final de classificação

| Item | Resultado | Impacto na decisão |
|------|-----------|-------------------|
| Fontes CFF no sistema | _a preencher_ | — |
| Fonte CFF de teste garantida? | _a preencher_ | Se não, marcar limitação explícita, não pular. |
| Comportamento actual (5 categorias) | _a preencher_ | Se corrupção silenciosa ou panic: bug prioritário, independente da Trilha 6. |
| Código detecta formato antes de subsetar? | _a preencher_ | Se não, fix XS obrigatório mesmo com scope-out da trilha completa. |
| oxifont-subset suporta CFF? | _a preencher_ | — |
| Corpus usa CFF? | _a preencher_ | — |
| Esforço estimado (trilha completa) | _a preencher_ | — |

---

## Decisão de prosseguimento

- **Se corrupção silenciosa (Grupo 2.3) ou ausência de detecção de formato (Grupo 3) forem confirmadas:** P523 é o fix XS de detecção de formato, **antes** de qualquer decisão sobre a trilha completa — isto é bug de robustez, não feature nova.
- **Se CFF é comum E oxifont-subset não suporta:** P523 é a especificação da Trilha 6 completa (L/XL).
- **Se CFF é raro E o comportamento actual é fonte completa embebida sem corrupção:** scope-out confirmado (não assumido). P523 documenta em ADR e retoma opções A/B do handoff.
- **Se Grupo 2 não foi testável (sem fonte CFF disponível):** registar como limitação da sonda, não como "CFF é raro" — são conclusões diferentes. Considerar obter a fonte de referência num passo dedicado antes de fechar a decisão.

---

## Relatório de execução

`00_nucleo/diagnosticos/sonda-cff-p522.md` — resultados dos 5 grupos, tabela final, recomendação.
